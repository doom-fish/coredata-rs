use crate::context::{NSManagedObject, NSManagedObjectContext};
use crate::error::CoreDataError;
use crate::ffi;
use crate::private::{
    collect_array, cstring_from_str, error_from_status, impl_object_wrapper, parse_json_ptr,
    take_string,
};
use crate::query::NSFetchRequest;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum NSFetchedResultsChangeType {
    Insert,
    Delete,
    Move,
    Update,
    Unknown(u64),
}

impl NSFetchedResultsChangeType {
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
pub struct FetchedResultsIndexPath {
    pub section: usize,
    pub item: usize,
}

impl FetchedResultsIndexPath {
    pub const fn new(section: usize, item: usize) -> Self {
        Self { section, item }
    }
}

impl_object_wrapper!(NSFetchedResultsController);
impl_object_wrapper!(NSFetchedResultsSectionInfo);

impl NSFetchedResultsController {
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
                &mut out_controller,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_controller, "fetched results controller") }
    }

    pub fn perform_fetch(&self) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe { ffi::cd_fetched_results_controller_perform_fetch(self.as_ptr(), &mut out_error) };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn fetch_request(&self) -> Result<NSFetchRequest, CoreDataError> {
        let ptr = unsafe { ffi::cd_fetched_results_controller_get_fetch_request(self.as_ptr()) };
        unsafe { NSFetchRequest::from_retained_ptr(ptr, "fetched results controller fetch request") }
    }

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

    pub fn section_name_key_path(&self) -> Option<String> {
        let ptr = unsafe { ffi::cd_fetched_results_controller_get_section_name_key_path(self.as_ptr()) };
        unsafe { take_string(ptr) }
    }

    pub fn cache_name(&self) -> Option<String> {
        let ptr = unsafe { ffi::cd_fetched_results_controller_get_cache_name(self.as_ptr()) };
        unsafe { take_string(ptr) }
    }

    pub fn fetched_objects(&self) -> Result<Vec<NSManagedObject>, CoreDataError> {
        let array_ptr = unsafe { ffi::cd_fetched_results_controller_get_fetched_objects(self.as_ptr()) };
        collect_array(array_ptr, "fetched results controller objects")
    }

    pub fn sections(&self) -> Result<Vec<NSFetchedResultsSectionInfo>, CoreDataError> {
        let array_ptr = unsafe { ffi::cd_fetched_results_controller_get_sections(self.as_ptr()) };
        collect_array(array_ptr, "fetched results controller sections")
    }

    pub fn section_index_titles(&self) -> Result<Vec<String>, CoreDataError> {
        let mut out_json = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_fetched_results_controller_get_section_index_titles_json(
                self.as_ptr(),
                &mut out_json,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { parse_json_ptr(out_json, "fetched results controller section index titles") }
    }

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
                &mut out_object,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { NSManagedObject::from_retained_ptr(out_object, "fetched results controller object") }
    }

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
                &mut section,
                &mut item,
                &mut out_error,
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
    pub fn name(&self) -> Result<String, CoreDataError> {
        let ptr = unsafe { ffi::cd_fetched_results_section_info_get_name(self.as_ptr()) };
        unsafe { take_string(ptr) }
            .ok_or_else(|| CoreDataError::bridge(-1, "fetched results section name was nil"))
    }

    pub fn index_title(&self) -> Option<String> {
        let ptr = unsafe { ffi::cd_fetched_results_section_info_get_index_title(self.as_ptr()) };
        unsafe { take_string(ptr) }
    }

    pub fn number_of_objects(&self) -> usize {
        unsafe { ffi::cd_fetched_results_section_info_get_number_of_objects(self.as_ptr()) as usize }
    }

    pub fn objects(&self) -> Result<Vec<NSManagedObject>, CoreDataError> {
        let array_ptr = unsafe { ffi::cd_fetched_results_section_info_get_objects(self.as_ptr()) };
        collect_array(array_ptr, "fetched results section objects")
    }
}
