use core::ffi::c_void;
use core::fmt;

use doom_fish_utils::callback_context::CallbackContext;

use crate::context::{NSManagedObject, NSManagedObjectContext};
use crate::error::CoreDataError;
use crate::ffi;
use crate::private::{
    collect_array, cstring_from_str, error_from_status, impl_sync_object_wrapper, parse_json_ptr,
    status_result, take_string,
};
use crate::query::NSFetchRequest;

/// Mirrors `NSFetchedResultsControllerDelegate`.
pub trait NSFetchedResultsControllerDelegate: Send + Sync + 'static {
    fn controller_will_change_content(&self, _controller: &NSFetchedResultsController) {}

    fn controller_did_change_object(
        &self,
        _controller: &NSFetchedResultsController,
        _object: &NSManagedObject,
        _change: NSFetchedResultsChangeType,
        _index_path: Option<FetchedResultsIndexPath>,
        _new_index_path: Option<FetchedResultsIndexPath>,
    ) {
    }

    fn controller_did_change_section(
        &self,
        _controller: &NSFetchedResultsController,
        _section: &NSFetchedResultsSectionInfo,
        _section_index: usize,
        _change: NSFetchedResultsChangeType,
    ) {
    }

    fn controller_did_change_content(&self, _controller: &NSFetchedResultsController) {}
}

type DelegateContext = CallbackContext<Box<dyn NSFetchedResultsControllerDelegate>>;

const EVENT_WILL_CHANGE_CONTENT: i32 = 0;
const EVENT_DID_CHANGE_OBJECT: i32 = 1;
const EVENT_DID_CHANGE_SECTION: i32 = 2;
const EVENT_DID_CHANGE_CONTENT: i32 = 3;

fn index_path(section: i64, item: i64) -> Option<FetchedResultsIndexPath> {
    let section = usize::try_from(section).ok()?;
    let item = usize::try_from(item).ok()?;
    Some(FetchedResultsIndexPath::new(section, item))
}

#[allow(clippy::too_many_arguments)]
unsafe extern "C" fn fetched_results_trampoline(
    context: *mut c_void,
    event: i32,
    controller: *mut c_void,
    item: *mut c_void,
    change_type: u64,
    from_section: i64,
    from_item: i64,
    to_section: i64,
    to_item: i64,
) {
    let controller = unsafe {
        NSFetchedResultsController::from_retained_ptr(
            controller,
            "fetched results delegate controller",
        )
    };
    let change = NSFetchedResultsChangeType::from_raw(change_type);
    match event {
        EVENT_DID_CHANGE_OBJECT => {
            let object = unsafe {
                NSManagedObject::from_retained_ptr(item, "fetched results delegate object")
            };
            if let (Ok(controller), Ok(object)) = (controller, object) {
                let _ = unsafe {
                    DelegateContext::with(
                        context,
                        "NSFetchedResultsControllerDelegate",
                        |delegate| {
                            delegate.controller_did_change_object(
                                &controller,
                                &object,
                                change,
                                index_path(from_section, from_item),
                                index_path(to_section, to_item),
                            );
                        },
                    )
                };
            }
        }
        EVENT_DID_CHANGE_SECTION => {
            let section = unsafe {
                NSFetchedResultsSectionInfo::from_retained_ptr(
                    item,
                    "fetched results delegate section",
                )
            };
            let section_index = usize::try_from(from_section);
            if let (Ok(controller), Ok(section), Ok(section_index)) =
                (controller, section, section_index)
            {
                let _ = unsafe {
                    DelegateContext::with(
                        context,
                        "NSFetchedResultsControllerDelegate",
                        |delegate| {
                            delegate.controller_did_change_section(
                                &controller,
                                &section,
                                section_index,
                                change,
                            );
                        },
                    )
                };
            }
        }
        EVENT_WILL_CHANGE_CONTENT | EVENT_DID_CHANGE_CONTENT => {
            if let Ok(controller) = controller {
                let _ = unsafe {
                    DelegateContext::with(
                        context,
                        "NSFetchedResultsControllerDelegate",
                        |delegate| {
                            if event == EVENT_WILL_CHANGE_CONTENT {
                                delegate.controller_will_change_content(&controller);
                            } else {
                                delegate.controller_did_change_content(&controller);
                            }
                        },
                    )
                };
            }
            if !item.is_null() {
                unsafe { ffi::cd_release_object(item) };
            }
        }
        _ => {
            if !item.is_null() {
                unsafe { ffi::cd_release_object(item) };
            }
        }
    }
}

#[must_use = "the delegate is detached when the registration is dropped"]
pub struct FetchedResultsControllerDelegateRegistration {
    controller: NSFetchedResultsController,
    context: DelegateContext,
}

impl fmt::Debug for FetchedResultsControllerDelegateRegistration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FetchedResultsControllerDelegateRegistration")
            .field("controller", &self.controller)
            .field("active", &self.context.is_active())
            .finish()
    }
}

impl Drop for FetchedResultsControllerDelegateRegistration {
    fn drop(&mut self) {
        self.context.deactivate();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_fetched_results_controller_remove_delegate(
                self.controller.as_ptr(),
                self.context.as_ptr(),
                &raw mut out_error,
            )
        };
        if status != ffi::status::OK {
            drop(unsafe { error_from_status(status, out_error) });
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
/// Mirrors the corresponding Core Data `NSFetchedResultsChangeType` value.
pub enum NSFetchedResultsChangeType {
    /// Mirrors `NSFetchedResultsChangeType::Insert`.
    Insert,
    /// Mirrors `NSFetchedResultsChangeType::Delete`.
    Delete,
    /// Mirrors `NSFetchedResultsChangeType::Move`.
    Move,
    /// Mirrors `NSFetchedResultsChangeType::Update`.
    Update,
    /// Mirrors `NSFetchedResultsChangeType::Unknown`.
    Unknown(u64),
}

impl NSFetchedResultsChangeType {
    /// Mirrors the corresponding `NSFetchedResultsChangeType` constant.
    pub const fn from_raw(raw: u64) -> Self {
        match raw {
            1 => Self::Insert,
            2 => Self::Delete,
            3 => Self::Move,
            4 => Self::Update,
            other => Self::Unknown(other),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Wraps `FetchedResultsIndexPath`.
pub struct FetchedResultsIndexPath {
    /// Mirrors `FetchedResultsIndexPath.section`.
    pub section: usize,
    /// Mirrors `FetchedResultsIndexPath.item`.
    pub item: usize,
}

impl FetchedResultsIndexPath {
    /// Mirrors the corresponding `FetchedResultsIndexPath` constant.
    pub const fn new(section: usize, item: usize) -> Self {
        Self { section, item }
    }
}

impl_sync_object_wrapper!(NSFetchedResultsController);
impl_sync_object_wrapper!(NSFetchedResultsSectionInfo);

impl NSFetchedResultsController {
    /// Wraps `NSFetchedResultsController.init(...)`.
    pub fn new(
        fetch_request: &NSFetchRequest,
        managed_object_context: &NSManagedObjectContext,
        section_name_key_path: Option<&str>,
        cache_name: Option<&str>,
    ) -> Result<Self, CoreDataError> {
        let section_name_key_path = section_name_key_path
            .map(|value| cstring_from_str(value, "fetched results section name key path"))
            .transpose()?;
        let cache_name = cache_name
            .map(|value| cstring_from_str(value, "fetched results cache name"))
            .transpose()?;
        let mut out_controller = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_fetched_results_controller_new(
                fetch_request.as_ptr(),
                managed_object_context.as_ptr(),
                section_name_key_path
                    .as_ref()
                    .map_or(core::ptr::null(), |value| value.as_c_str().as_ptr()),
                cache_name
                    .as_ref()
                    .map_or(core::ptr::null(), |value| value.as_c_str().as_ptr()),
                &raw mut out_controller,
                &raw mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_controller, "fetched results controller") }
    }

    /// Wraps `NSFetchedResultsController.perform_fetch(...)`.
    pub fn perform_fetch(&self) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_fetched_results_controller_perform_fetch(self.as_ptr(), &raw mut out_error)
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn set_delegate<D>(
        &self,
        delegate: D,
    ) -> Result<FetchedResultsControllerDelegateRegistration, CoreDataError>
    where
        D: NSFetchedResultsControllerDelegate,
    {
        let context: DelegateContext = CallbackContext::new(Box::new(delegate));
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_fetched_results_controller_set_delegate(
                self.as_ptr(),
                context.retained_ptr(),
                fetched_results_trampoline,
                DelegateContext::RELEASE,
                &raw mut out_error,
            )
        };
        status_result(status, out_error)?;
        Ok(FetchedResultsControllerDelegateRegistration {
            controller: self.clone(),
            context,
        })
    }

    /// Wraps `NSFetchedResultsController.fetch_request(...)`.
    pub fn fetch_request(&self) -> Result<NSFetchRequest, CoreDataError> {
        let ptr = unsafe { ffi::cd_fetched_results_controller_get_fetch_request(self.as_ptr()) };
        unsafe {
            NSFetchRequest::from_retained_ptr(ptr, "fetched results controller fetch request")
        }
    }

    /// Wraps `NSFetchedResultsController.managed_object_context(...)`.
    pub fn managed_object_context(&self) -> Result<NSManagedObjectContext, CoreDataError> {
        let ptr =
            unsafe { ffi::cd_fetched_results_controller_get_managed_object_context(self.as_ptr()) };
        unsafe {
            NSManagedObjectContext::from_retained_ptr(
                ptr,
                "fetched results controller managed object context",
            )
        }
    }

    /// Wraps `NSFetchedResultsController.section_name_key_path(...)`.
    pub fn section_name_key_path(&self) -> Option<String> {
        let ptr =
            unsafe { ffi::cd_fetched_results_controller_get_section_name_key_path(self.as_ptr()) };
        unsafe { take_string(ptr) }
    }

    /// Wraps `NSFetchedResultsController.cache_name(...)`.
    pub fn cache_name(&self) -> Option<String> {
        let ptr = unsafe { ffi::cd_fetched_results_controller_get_cache_name(self.as_ptr()) };
        unsafe { take_string(ptr) }
    }

    /// Wraps `NSFetchedResultsController.fetched_objects(...)`.
    pub fn fetched_objects(&self) -> Result<Vec<NSManagedObject>, CoreDataError> {
        let mut out_array = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_fetched_results_controller_get_fetched_objects(
                self.as_ptr(),
                &raw mut out_array,
                &raw mut out_error,
            )
        };
        status_result(status, out_error)?;
        collect_array(out_array, "fetched results controller objects")
    }

    /// Wraps `NSFetchedResultsController.sections(...)`.
    pub fn sections(&self) -> Result<Vec<NSFetchedResultsSectionInfo>, CoreDataError> {
        let mut out_array = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_fetched_results_controller_get_sections(
                self.as_ptr(),
                &raw mut out_array,
                &raw mut out_error,
            )
        };
        status_result(status, out_error)?;
        collect_array(out_array, "fetched results controller sections")
    }

    /// Wraps `NSFetchedResultsController.section_index_titles(...)`.
    pub fn section_index_titles(&self) -> Result<Vec<String>, CoreDataError> {
        let mut out_json = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_fetched_results_controller_get_section_index_titles_json(
                self.as_ptr(),
                &raw mut out_json,
                &raw mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { parse_json_ptr(out_json, "fetched results controller section index titles") }
    }

    /// Wraps `NSFetchedResultsController.object_at_index_path(...)`.
    pub fn object_at_index_path(
        &self,
        index_path: FetchedResultsIndexPath,
    ) -> Result<NSManagedObject, CoreDataError> {
        let section = i64::try_from(index_path.section)
            .map_err(|_| CoreDataError::bridge(-1, "fetched results section overflow"))?;
        let item = i64::try_from(index_path.item)
            .map_err(|_| CoreDataError::bridge(-1, "fetched results item overflow"))?;
        let mut out_object = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_fetched_results_controller_object_at_index_path(
                self.as_ptr(),
                section,
                item,
                &raw mut out_object,
                &raw mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe {
            NSManagedObject::from_retained_ptr(out_object, "fetched results controller object")
        }
    }

    /// Wraps `NSFetchedResultsController.index_path_for_object(...)`.
    pub fn index_path_for_object(
        &self,
        object: &NSManagedObject,
    ) -> Result<Option<FetchedResultsIndexPath>, CoreDataError> {
        let mut section = -1_i64;
        let mut item = -1_i64;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_fetched_results_controller_index_path_for_object(
                self.as_ptr(),
                object.as_ptr(),
                &raw mut section,
                &raw mut item,
                &raw mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        if section < 0 || item < 0 {
            return Ok(None);
        }
        let section = usize::try_from(section)
            .map_err(|_| CoreDataError::bridge(-1, "negative fetched results section"))?;
        let item = usize::try_from(item)
            .map_err(|_| CoreDataError::bridge(-1, "negative fetched results item"))?;
        Ok(Some(FetchedResultsIndexPath::new(section, item)))
    }

    /// Wraps `NSFetchedResultsController.delete_cache_with_name(...)`.
    pub fn delete_cache_with_name(name: Option<&str>) -> Result<(), CoreDataError> {
        let name = name
            .map(|value| cstring_from_str(value, "fetched results cache name"))
            .transpose()?;
        unsafe {
            ffi::cd_fetched_results_controller_delete_cache_with_name(
                name.as_ref()
                    .map_or(core::ptr::null(), |value| value.as_c_str().as_ptr()),
            );
        }
        Ok(())
    }
}

impl NSFetchedResultsSectionInfo {
    /// Wraps `NSFetchedResultsSectionInfo.name(...)`.
    pub fn name(&self) -> Result<String, CoreDataError> {
        let mut out_name = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_fetched_results_section_info_get_name(
                self.as_ptr(),
                &raw mut out_name,
                &raw mut out_error,
            )
        };
        status_result(status, out_error)?;
        unsafe { take_string(out_name) }
            .ok_or_else(|| CoreDataError::bridge(-1, "fetched results section name was nil"))
    }

    /// Wraps `NSFetchedResultsSectionInfo.index_title(...)`.
    pub fn index_title(&self) -> Result<Option<String>, CoreDataError> {
        let mut out_title = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_fetched_results_section_info_get_index_title(
                self.as_ptr(),
                &raw mut out_title,
                &raw mut out_error,
            )
        };
        status_result(status, out_error)?;
        Ok(unsafe { take_string(out_title) })
    }

    /// Wraps `NSFetchedResultsSectionInfo.number_of_objects(...)`.
    pub fn number_of_objects(&self) -> Result<usize, CoreDataError> {
        let mut out_count = 0_u64;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_fetched_results_section_info_get_number_of_objects(
                self.as_ptr(),
                &raw mut out_count,
                &raw mut out_error,
            )
        };
        status_result(status, out_error)?;
        usize::try_from(out_count)
            .map_err(|_| CoreDataError::bridge(-1, "fetched results section count overflow"))
    }

    /// Wraps `NSFetchedResultsSectionInfo.objects(...)`.
    pub fn objects(&self) -> Result<Vec<NSManagedObject>, CoreDataError> {
        let mut out_array = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_fetched_results_section_info_get_objects(
                self.as_ptr(),
                &raw mut out_array,
                &raw mut out_error,
            )
        };
        status_result(status, out_error)?;
        collect_array(out_array, "fetched results section objects")
    }
}
