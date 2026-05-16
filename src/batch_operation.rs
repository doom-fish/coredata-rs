use std::collections::BTreeMap;

use crate::context::NSManagedObjectContext;
use crate::error::CoreDataError;
use crate::ffi;
use crate::managed_object::NSManagedObjectID;
use crate::private::{collect_array, error_from_status, impl_object_wrapper, take_string};
use crate::query::NSFetchRequest;
use crate::value::{Value, ValuePayload};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum BatchDeleteRequestResultType {
    StatusOnly,
    ObjectIds,
    Count,
    Unknown(u64),
}

impl BatchDeleteRequestResultType {
    const fn from_raw(raw: u64) -> Self {
        match raw {
            0 => Self::StatusOnly,
            1 => Self::ObjectIds,
            2 => Self::Count,
            other => Self::Unknown(other),
        }
    }

    const fn as_raw(self) -> u64 {
        match self {
            Self::StatusOnly => 0,
            Self::ObjectIds => 1,
            Self::Count => 2,
            Self::Unknown(raw) => raw,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum BatchInsertRequestResultType {
    StatusOnly,
    ObjectIds,
    Count,
    Unknown(u64),
}

impl BatchInsertRequestResultType {
    const fn from_raw(raw: u64) -> Self {
        match raw {
            0 => Self::StatusOnly,
            1 => Self::ObjectIds,
            2 => Self::Count,
            other => Self::Unknown(other),
        }
    }

    const fn as_raw(self) -> u64 {
        match self {
            Self::StatusOnly => 0,
            Self::ObjectIds => 1,
            Self::Count => 2,
            Self::Unknown(raw) => raw,
        }
    }
}

impl_object_wrapper!(NSBatchDeleteRequest);
impl_object_wrapper!(NSBatchDeleteResult);
impl_object_wrapper!(NSBatchInsertRequest);
impl_object_wrapper!(NSBatchInsertResult);

fn encode_object_rows(
    objects: &[BTreeMap<String, Value>],
) -> Result<std::ffi::CString, CoreDataError> {
    let payload = objects
        .iter()
        .map(|object| {
            object
                .iter()
                .map(|(key, value)| (key.clone(), ValuePayload::from(value)))
                .collect::<BTreeMap<_, _>>()
        })
        .collect::<Vec<_>>();
    crate::private::json_cstring(&payload, "batch insert rows")
}

impl NSBatchDeleteRequest {
    pub fn from_fetch_request(fetch_request: &NSFetchRequest) -> Result<Self, CoreDataError> {
        let mut out_request = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_batch_delete_request_new_with_fetch_request(
                fetch_request.as_ptr(),
                &mut out_request,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_request, "batch delete request") }
    }

    pub fn from_object_ids(object_ids: &[&NSManagedObjectID]) -> Result<Self, CoreDataError> {
        let raw_object_ids = object_ids
            .iter()
            .map(|object_id| object_id.as_ptr())
            .collect::<Vec<_>>();
        let count = i32::try_from(raw_object_ids.len())
            .map_err(|_| CoreDataError::bridge(-1, "batch delete object ID count overflow"))?;
        let mut out_request = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_batch_delete_request_new_with_object_ids(
                raw_object_ids.as_ptr(),
                count,
                &mut out_request,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_request, "batch delete request") }
    }

    pub fn result_type(&self) -> BatchDeleteRequestResultType {
        BatchDeleteRequestResultType::from_raw(unsafe {
            ffi::cd_batch_delete_request_get_result_type(self.as_ptr())
        })
    }

    pub fn set_result_type(&self, result_type: BatchDeleteRequestResultType) {
        unsafe { ffi::cd_batch_delete_request_set_result_type(self.as_ptr(), result_type.as_raw()) }
    }

    pub fn execute(
        &self,
        context: &NSManagedObjectContext,
    ) -> Result<NSBatchDeleteResult, CoreDataError> {
        let mut out_result = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_context_execute_batch_delete_request(
                context.as_ptr(),
                self.as_ptr(),
                &mut out_result,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { NSBatchDeleteResult::from_retained_ptr(out_result, "batch delete result") }
    }
}

impl NSBatchDeleteResult {
    pub fn result_type(&self) -> BatchDeleteRequestResultType {
        BatchDeleteRequestResultType::from_raw(unsafe {
            ffi::cd_batch_delete_result_get_result_type(self.as_ptr())
        })
    }

    pub fn status(&self) -> bool {
        unsafe { ffi::cd_batch_delete_result_get_status(self.as_ptr()) != 0 }
    }

    pub fn count(&self) -> usize {
        unsafe { ffi::cd_batch_delete_result_get_count(self.as_ptr()) as usize }
    }

    pub fn object_ids(&self) -> Result<Vec<NSManagedObjectID>, CoreDataError> {
        let array_ptr = unsafe { ffi::cd_batch_delete_result_get_object_ids(self.as_ptr()) };
        collect_array(array_ptr, "batch delete result object IDs")
    }
}

impl NSBatchInsertRequest {
    pub fn new(
        entity_name: &str,
        objects: &[BTreeMap<String, Value>],
    ) -> Result<Self, CoreDataError> {
        let entity_name =
            crate::private::cstring_from_str(entity_name, "batch insert entity name")?;
        let objects_json = encode_object_rows(objects)?;
        let mut out_request = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_batch_insert_request_new_with_entity_name(
                entity_name.as_ptr(),
                objects_json.as_ptr(),
                &mut out_request,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_request, "batch insert request") }
    }

    pub fn entity_name(&self) -> Result<String, CoreDataError> {
        let ptr = unsafe { ffi::cd_batch_insert_request_get_entity_name(self.as_ptr()) };
        unsafe { take_string(ptr) }
            .ok_or_else(|| CoreDataError::bridge(-1, "batch insert entity name was nil"))
    }

    pub fn result_type(&self) -> BatchInsertRequestResultType {
        BatchInsertRequestResultType::from_raw(unsafe {
            ffi::cd_batch_insert_request_get_result_type(self.as_ptr())
        })
    }

    pub fn set_result_type(&self, result_type: BatchInsertRequestResultType) {
        unsafe { ffi::cd_batch_insert_request_set_result_type(self.as_ptr(), result_type.as_raw()) }
    }

    pub fn execute(
        &self,
        context: &NSManagedObjectContext,
    ) -> Result<NSBatchInsertResult, CoreDataError> {
        let mut out_result = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_context_execute_batch_insert_request(
                context.as_ptr(),
                self.as_ptr(),
                &mut out_result,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { NSBatchInsertResult::from_retained_ptr(out_result, "batch insert result") }
    }
}

impl NSBatchInsertResult {
    pub fn result_type(&self) -> BatchInsertRequestResultType {
        BatchInsertRequestResultType::from_raw(unsafe {
            ffi::cd_batch_insert_result_get_result_type(self.as_ptr())
        })
    }

    pub fn status(&self) -> bool {
        unsafe { ffi::cd_batch_insert_result_get_status(self.as_ptr()) != 0 }
    }

    pub fn count(&self) -> usize {
        unsafe { ffi::cd_batch_insert_result_get_count(self.as_ptr()) as usize }
    }

    pub fn object_ids(&self) -> Result<Vec<NSManagedObjectID>, CoreDataError> {
        let array_ptr = unsafe { ffi::cd_batch_insert_result_get_object_ids(self.as_ptr()) };
        collect_array(array_ptr, "batch insert result object IDs")
    }
}
