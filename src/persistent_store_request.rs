use core::ffi::c_void;

use crate::batch_operation::{
    NSBatchDeleteRequest, NSBatchDeleteResult, NSBatchInsertRequest, NSBatchInsertResult,
    NSBatchUpdateRequest, NSBatchUpdateResult,
};
use crate::cloudkit_mirroring::NSPersistentCloudKitContainerEventResult;
use crate::context::{NSManagedObject, NSManagedObjectContext};
use crate::error::CoreDataError;
use crate::ffi;
use crate::history::NSPersistentHistoryResult;
use crate::managed_object::NSManagedObjectID;
use crate::persistent_store_coordinator::NSPersistentStore;
use crate::private::{collect_array, error_from_status, impl_object_wrapper, parse_error_ptr};
use crate::query::NSFetchRequest;

pub trait NSFetchRequestResult {}

impl NSFetchRequestResult for NSManagedObject {}
impl NSFetchRequestResult for NSManagedObjectID {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum NSPersistentStoreRequestType {
    Fetch,
    Save,
    BatchInsert,
    BatchUpdate,
    BatchDelete,
    Unknown(u64),
}

impl NSPersistentStoreRequestType {
    const fn from_raw(raw: u64) -> Self {
        match raw {
            1 => Self::Fetch,
            2 => Self::Save,
            5 => Self::BatchInsert,
            6 => Self::BatchUpdate,
            7 => Self::BatchDelete,
            other => Self::Unknown(other),
        }
    }
}

impl_object_wrapper!(NSPersistentStoreRequest);
impl_object_wrapper!(NSPersistentStoreResult);
impl_object_wrapper!(NSPersistentStoreAsynchronousResult);
impl_object_wrapper!(NSAsynchronousFetchRequest);
impl_object_wrapper!(NSAsynchronousFetchResult);
impl_object_wrapper!(NSSaveChangesRequest);
impl_object_wrapper!(NSFetchRequestExpression);

fn clone_retained_wrapper<T>(ptr: *mut c_void, context: &str) -> Result<T, CoreDataError>
where
    T: crate::private::FromRetainedPtr,
{
    let retained = unsafe { ffi::cd_retain_object(ptr) };
    unsafe { T::from_retained_ptr(retained, context) }
}

fn collect_object_slice(
    objects: &[&NSManagedObject],
) -> Result<(*const *mut c_void, i32, Vec<*mut c_void>), CoreDataError> {
    let raw_objects = objects
        .iter()
        .map(|object| object.as_ptr())
        .collect::<Vec<_>>();
    let count = i32::try_from(raw_objects.len())
        .map_err(|_| CoreDataError::bridge(-1, "managed object count overflow"))?;
    Ok((raw_objects.as_ptr(), count, raw_objects))
}

impl NSPersistentStoreRequest {
    pub fn affected_stores(&self) -> Result<Vec<NSPersistentStore>, CoreDataError> {
        let array_ptr =
            unsafe { ffi::cd_persistent_store_request_get_affected_stores(self.as_ptr()) };
        collect_array(array_ptr, "persistent store request affected stores")
    }

    pub fn set_affected_stores(&self, stores: &[&NSPersistentStore]) -> Result<(), CoreDataError> {
        let raw_stores = stores
            .iter()
            .map(|store| store.as_ptr())
            .collect::<Vec<_>>();
        let count = i32::try_from(raw_stores.len()).map_err(|_| {
            CoreDataError::bridge(-1, "persistent store request store count overflow")
        })?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_store_request_set_affected_stores(
                self.as_ptr(),
                raw_stores.as_ptr(),
                count,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn request_type(&self) -> NSPersistentStoreRequestType {
        NSPersistentStoreRequestType::from_raw(unsafe {
            ffi::cd_persistent_store_request_get_request_type(self.as_ptr())
        })
    }
}

impl NSPersistentStoreResult {
    pub fn as_asynchronous_result(
        &self,
    ) -> Result<NSPersistentStoreAsynchronousResult, CoreDataError> {
        clone_retained_wrapper(self.as_ptr(), "persistent store asynchronous result")
    }
}

impl NSPersistentStoreAsynchronousResult {
    pub fn managed_object_context(&self) -> Result<NSManagedObjectContext, CoreDataError> {
        let ptr = unsafe {
            ffi::cd_persistent_store_async_result_get_managed_object_context(self.as_ptr())
        };
        unsafe {
            NSManagedObjectContext::from_retained_ptr(
                ptr,
                "persistent store asynchronous result context",
            )
        }
    }

    pub fn operation_error(&self) -> Option<CoreDataError> {
        let ptr = unsafe {
            ffi::cd_persistent_store_async_result_get_operation_error_json(self.as_ptr())
        };
        if ptr.is_null() {
            return None;
        }
        Some(unsafe { parse_error_ptr(ptr) })
    }

    pub fn progress_fraction_completed(&self) -> Option<f64> {
        if unsafe { ffi::cd_persistent_store_async_result_has_progress(self.as_ptr()) } == 0 {
            return None;
        }
        Some(unsafe {
            ffi::cd_persistent_store_async_result_get_progress_fraction_completed(self.as_ptr())
        })
    }
}

impl NSAsynchronousFetchRequest {
    pub fn new(fetch_request: &NSFetchRequest) -> Result<Self, CoreDataError> {
        let mut out_request = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_asynchronous_fetch_request_new(
                fetch_request.as_ptr(),
                &mut out_request,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_request, "asynchronous fetch request") }
    }

    pub fn fetch_request(&self) -> Result<NSFetchRequest, CoreDataError> {
        let ptr = unsafe { ffi::cd_asynchronous_fetch_request_get_fetch_request(self.as_ptr()) };
        unsafe {
            NSFetchRequest::from_retained_ptr(ptr, "asynchronous fetch request fetch request")
        }
    }

    pub fn estimated_result_count(&self) -> i64 {
        unsafe { ffi::cd_asynchronous_fetch_request_get_estimated_result_count(self.as_ptr()) }
    }

    pub fn set_estimated_result_count(&self, estimated_result_count: i64) {
        unsafe {
            ffi::cd_asynchronous_fetch_request_set_estimated_result_count(
                self.as_ptr(),
                estimated_result_count,
            );
        }
    }

    pub fn execute(
        &self,
        context: &NSManagedObjectContext,
    ) -> Result<NSAsynchronousFetchResult, CoreDataError> {
        let mut out_result = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_context_execute_asynchronous_fetch_request(
                context.as_ptr(),
                self.as_ptr(),
                &mut out_result,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe {
            NSAsynchronousFetchResult::from_retained_ptr(out_result, "asynchronous fetch result")
        }
    }

    pub fn as_persistent_store_request(&self) -> Result<NSPersistentStoreRequest, CoreDataError> {
        clone_retained_wrapper(self.as_ptr(), "persistent store request")
    }
}

impl NSAsynchronousFetchResult {
    pub fn asynchronous_result(
        &self,
    ) -> Result<NSPersistentStoreAsynchronousResult, CoreDataError> {
        clone_retained_wrapper(self.as_ptr(), "persistent store asynchronous result")
    }

    pub fn fetch_request(&self) -> Result<NSAsynchronousFetchRequest, CoreDataError> {
        let ptr = unsafe { ffi::cd_asynchronous_fetch_result_get_fetch_request(self.as_ptr()) };
        unsafe {
            NSAsynchronousFetchRequest::from_retained_ptr(ptr, "asynchronous fetch result request")
        }
    }

    pub fn final_result_count(&self) -> usize {
        unsafe { ffi::cd_asynchronous_fetch_result_get_final_result_count(self.as_ptr()) as usize }
    }
}

impl NSSaveChangesRequest {
    pub fn new(
        inserted_objects: &[&NSManagedObject],
        updated_objects: &[&NSManagedObject],
        deleted_objects: &[&NSManagedObject],
        locked_objects: &[&NSManagedObject],
    ) -> Result<Self, CoreDataError> {
        let (inserted_ptr, inserted_count, inserted_raw) = collect_object_slice(inserted_objects)?;
        let (updated_ptr, updated_count, updated_raw) = collect_object_slice(updated_objects)?;
        let (deleted_ptr, deleted_count, deleted_raw) = collect_object_slice(deleted_objects)?;
        let (locked_ptr, locked_count, locked_raw) = collect_object_slice(locked_objects)?;
        let mut out_request = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_save_changes_request_new(
                inserted_ptr,
                inserted_count,
                updated_ptr,
                updated_count,
                deleted_ptr,
                deleted_count,
                locked_ptr,
                locked_count,
                &mut out_request,
                &mut out_error,
            )
        };
        let _ = (inserted_raw, updated_raw, deleted_raw, locked_raw);
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_request, "save changes request") }
    }

    pub fn inserted_objects(&self) -> Result<Vec<NSManagedObject>, CoreDataError> {
        let array_ptr = unsafe { ffi::cd_save_changes_request_get_inserted_objects(self.as_ptr()) };
        collect_array(array_ptr, "save changes request inserted objects")
    }

    pub fn updated_objects(&self) -> Result<Vec<NSManagedObject>, CoreDataError> {
        let array_ptr = unsafe { ffi::cd_save_changes_request_get_updated_objects(self.as_ptr()) };
        collect_array(array_ptr, "save changes request updated objects")
    }

    pub fn deleted_objects(&self) -> Result<Vec<NSManagedObject>, CoreDataError> {
        let array_ptr = unsafe { ffi::cd_save_changes_request_get_deleted_objects(self.as_ptr()) };
        collect_array(array_ptr, "save changes request deleted objects")
    }

    pub fn locked_objects(&self) -> Result<Vec<NSManagedObject>, CoreDataError> {
        let array_ptr = unsafe { ffi::cd_save_changes_request_get_locked_objects(self.as_ptr()) };
        collect_array(array_ptr, "save changes request locked objects")
    }

    pub fn as_persistent_store_request(&self) -> Result<NSPersistentStoreRequest, CoreDataError> {
        clone_retained_wrapper(self.as_ptr(), "persistent store request")
    }
}

impl NSFetchRequestExpression {
    pub fn new(
        fetch_request: &NSFetchRequest,
        context: &NSManagedObjectContext,
        count_only: bool,
    ) -> Result<Self, CoreDataError> {
        let mut out_expression = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_fetch_request_expression_new(
                fetch_request.as_ptr(),
                context.as_ptr(),
                i32::from(count_only),
                &mut out_expression,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_expression, "fetch request expression") }
    }

    pub fn is_count_only_request(&self) -> bool {
        unsafe { ffi::cd_fetch_request_expression_get_count_only_request(self.as_ptr()) != 0 }
    }
}

impl NSFetchRequest {
    pub fn as_persistent_store_request(&self) -> Result<NSPersistentStoreRequest, CoreDataError> {
        clone_retained_wrapper(self.as_ptr(), "persistent store request")
    }
}

impl NSBatchDeleteRequest {
    pub fn as_persistent_store_request(&self) -> Result<NSPersistentStoreRequest, CoreDataError> {
        clone_retained_wrapper(self.as_ptr(), "persistent store request")
    }
}

impl NSBatchInsertRequest {
    pub fn as_persistent_store_request(&self) -> Result<NSPersistentStoreRequest, CoreDataError> {
        clone_retained_wrapper(self.as_ptr(), "persistent store request")
    }
}

impl NSBatchUpdateRequest {
    pub fn as_persistent_store_request(&self) -> Result<NSPersistentStoreRequest, CoreDataError> {
        clone_retained_wrapper(self.as_ptr(), "persistent store request")
    }
}

impl NSBatchDeleteResult {
    pub fn as_persistent_store_result(&self) -> Result<NSPersistentStoreResult, CoreDataError> {
        clone_retained_wrapper(self.as_ptr(), "persistent store result")
    }
}

impl NSBatchInsertResult {
    pub fn as_persistent_store_result(&self) -> Result<NSPersistentStoreResult, CoreDataError> {
        clone_retained_wrapper(self.as_ptr(), "persistent store result")
    }
}

impl NSBatchUpdateResult {
    pub fn as_persistent_store_result(&self) -> Result<NSPersistentStoreResult, CoreDataError> {
        clone_retained_wrapper(self.as_ptr(), "persistent store result")
    }
}

impl NSPersistentHistoryResult {
    pub fn as_persistent_store_result(&self) -> Result<NSPersistentStoreResult, CoreDataError> {
        clone_retained_wrapper(self.as_ptr(), "persistent store result")
    }
}

impl NSPersistentCloudKitContainerEventResult {
    pub fn as_persistent_store_result(&self) -> Result<NSPersistentStoreResult, CoreDataError> {
        clone_retained_wrapper(self.as_ptr(), "persistent store result")
    }
}
