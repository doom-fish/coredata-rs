use std::collections::BTreeMap;

use crate::context::NSManagedObjectContext;
use crate::error::CoreDataError;
use crate::ffi;
use crate::managed_object::NSManagedObjectID;
use crate::private::{collect_array, error_from_status, impl_object_wrapper, take_string};
use crate::query::{NSFetchRequest, NSPredicate};
use crate::schema::NSEntityDescription;
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

    pub fn fetch_request(&self) -> Result<NSFetchRequest, CoreDataError> {
        let ptr = unsafe { ffi::cd_batch_delete_request_get_fetch_request(self.as_ptr()) };
        unsafe { NSFetchRequest::from_retained_ptr(ptr, "batch delete request fetch request") }
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

    pub fn with_entity(
        entity: &NSEntityDescription,
        objects: &[BTreeMap<String, Value>],
    ) -> Result<Self, CoreDataError> {
        let objects_json = encode_object_rows(objects)?;
        let mut out_request = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_batch_insert_request_new_with_entity(
                entity.as_ptr(),
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

    pub fn entity(&self) -> Result<Option<NSEntityDescription>, CoreDataError> {
        let ptr = unsafe { ffi::cd_batch_insert_request_get_entity(self.as_ptr()) };
        if ptr.is_null() {
            return Ok(None);
        }
        Ok(Some(unsafe {
            NSEntityDescription::from_retained_ptr(ptr, "batch insert request entity")?
        }))
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum BatchUpdateRequestResultType {
    StatusOnly,
    ObjectIds,
    Count,
    Unknown(u64),
}

impl BatchUpdateRequestResultType {
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

impl_object_wrapper!(NSBatchUpdateRequest);
impl_object_wrapper!(NSBatchUpdateResult);

fn encode_update_properties(
    properties: &BTreeMap<String, Value>,
) -> Result<std::ffi::CString, CoreDataError> {
    let payload = properties
        .iter()
        .map(|(key, value)| (key.clone(), ValuePayload::from(value)))
        .collect::<BTreeMap<_, _>>();
    crate::private::json_cstring(&payload, "batch update properties")
}

impl NSBatchUpdateRequest {
    pub fn new(entity_name: &str) -> Result<Self, CoreDataError> {
        let entity_name =
            crate::private::cstring_from_str(entity_name, "batch update entity name")?;
        let mut out_request = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_batch_update_request_new_with_entity_name(
                entity_name.as_ptr(),
                &mut out_request,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_request, "batch update request") }
    }

    pub fn entity_name(&self) -> Result<String, CoreDataError> {
        let ptr = unsafe { ffi::cd_batch_update_request_get_entity_name(self.as_ptr()) };
        unsafe { take_string(ptr) }
            .ok_or_else(|| CoreDataError::bridge(-1, "batch update entity name was nil"))
    }

    pub fn entity(&self) -> Result<Option<NSEntityDescription>, CoreDataError> {
        let ptr = unsafe { ffi::cd_batch_update_request_get_entity(self.as_ptr()) };
        if ptr.is_null() {
            return Ok(None);
        }
        Ok(Some(unsafe {
            NSEntityDescription::from_retained_ptr(ptr, "batch update request entity")?
        }))
    }

    pub fn includes_subentities(&self) -> bool {
        unsafe { ffi::cd_batch_update_request_get_includes_subentities(self.as_ptr()) != 0 }
    }

    pub fn set_includes_subentities(&self, includes_subentities: bool) {
        unsafe {
            ffi::cd_batch_update_request_set_includes_subentities(
                self.as_ptr(),
                i32::from(includes_subentities),
            );
        }
    }

    pub fn result_type(&self) -> BatchUpdateRequestResultType {
        BatchUpdateRequestResultType::from_raw(unsafe {
            ffi::cd_batch_update_request_get_result_type(self.as_ptr())
        })
    }

    pub fn set_result_type(&self, result_type: BatchUpdateRequestResultType) {
        unsafe { ffi::cd_batch_update_request_set_result_type(self.as_ptr(), result_type.as_raw()) }
    }

    pub fn set_predicate(&self, predicate: Option<&NSPredicate>) {
        unsafe {
            ffi::cd_batch_update_request_set_predicate(
                self.as_ptr(),
                predicate.map_or(core::ptr::null_mut(), NSPredicate::as_ptr),
            );
        }
    }

    pub fn set_properties_to_update(
        &self,
        properties_to_update: Option<&BTreeMap<String, Value>>,
    ) -> Result<(), CoreDataError> {
        let properties_to_update = properties_to_update
            .map(encode_update_properties)
            .transpose()?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_batch_update_request_set_properties_to_update_json(
                self.as_ptr(),
                properties_to_update
                    .as_ref()
                    .map_or(core::ptr::null(), |value| value.as_c_str().as_ptr()),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn execute(
        &self,
        context: &NSManagedObjectContext,
    ) -> Result<NSBatchUpdateResult, CoreDataError> {
        let mut out_result = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_context_execute_batch_update_request(
                context.as_ptr(),
                self.as_ptr(),
                &mut out_result,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { NSBatchUpdateResult::from_retained_ptr(out_result, "batch update result") }
    }
}

impl NSBatchUpdateResult {
    pub fn result_type(&self) -> BatchUpdateRequestResultType {
        BatchUpdateRequestResultType::from_raw(unsafe {
            ffi::cd_batch_update_result_get_result_type(self.as_ptr())
        })
    }

    pub fn status(&self) -> bool {
        unsafe { ffi::cd_batch_update_result_get_status(self.as_ptr()) != 0 }
    }

    pub fn count(&self) -> usize {
        unsafe { ffi::cd_batch_update_result_get_count(self.as_ptr()) as usize }
    }

    pub fn object_ids(&self) -> Result<Vec<NSManagedObjectID>, CoreDataError> {
        let array_ptr = unsafe { ffi::cd_batch_update_result_get_object_ids(self.as_ptr()) };
        collect_array(array_ptr, "batch update result object IDs")
    }
}
