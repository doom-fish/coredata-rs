use std::collections::BTreeMap;

use crate::context::{NSManagedObject, NSManagedObjectContext};
use crate::error::CoreDataError;
use crate::ffi;
use crate::private::{
    collect_array, cstring_from_str, error_from_status, impl_sync_object_wrapper, json_cstring,
    parse_json_ptr, status_result, take_string,
};
use crate::schema::NSEntityDescription;
use crate::value::{Value, ValuePayload};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Wraps `NSSnapshotEventType`.
pub struct NSSnapshotEventType(u64);

impl NSSnapshotEventType {
    /// Mirrors the corresponding `NSSnapshotEventType` constant.
    pub const NONE: Self = Self(0);
    /// Mirrors the corresponding `NSSnapshotEventType` constant.
    pub const UNDO_INSERTION: Self = Self(1 << 1);
    /// Mirrors the corresponding `NSSnapshotEventType` constant.
    pub const UNDO_DELETION: Self = Self(1 << 2);
    /// Mirrors the corresponding `NSSnapshotEventType` constant.
    pub const UNDO_UPDATE: Self = Self(1 << 3);
    /// Mirrors the corresponding `NSSnapshotEventType` constant.
    pub const ROLLBACK: Self = Self(1 << 4);
    /// Mirrors the corresponding `NSSnapshotEventType` constant.
    pub const REFRESH: Self = Self(1 << 5);
    /// Mirrors the corresponding `NSSnapshotEventType` constant.
    pub const MERGE_POLICY: Self = Self(1 << 6);

    /// Mirrors the corresponding `NSSnapshotEventType` constant.
    pub const fn bits(self) -> u64 {
        self.0
    }

    /// Mirrors the corresponding `NSSnapshotEventType` constant.
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Mirrors the corresponding `NSSnapshotEventType` constant.
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl_sync_object_wrapper!(NSManagedObjectID);

type ObjectFlagGetter =
    unsafe extern "C" fn(*mut core::ffi::c_void, *mut i32, *mut *mut core::ffi::c_char) -> i32;

fn value_map_from_json_ptr(
    ptr: *mut core::ffi::c_char,
    context: &str,
) -> Result<BTreeMap<String, Value>, CoreDataError> {
    let payloads: BTreeMap<String, ValuePayload> = unsafe { parse_json_ptr(ptr, context)? };
    payloads
        .into_iter()
        .map(|(key, value)| value.try_into().map(|value| (key, value)))
        .collect()
}

impl NSManagedObject {
    fn flag(&self, getter: ObjectFlagGetter) -> Result<bool, CoreDataError> {
        let mut out_value = 0_i32;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe { getter(self.as_ptr(), &raw mut out_value, &raw mut out_error) };
        status_result(status, out_error)?;
        Ok(out_value != 0)
    }

    /// Wraps `NSManagedObject.managed_object_context(...)`.
    pub fn managed_object_context(&self) -> Result<Option<NSManagedObjectContext>, CoreDataError> {
        let mut out_context = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_get_managed_object_context(
                self.as_ptr(),
                &raw mut out_context,
                &raw mut out_error,
            )
        };
        status_result(status, out_error)?;
        if out_context.is_null() {
            return Ok(None);
        }
        Ok(Some(unsafe {
            NSManagedObjectContext::from_retained_ptr(out_context, "managed object context")?
        }))
    }

    /// Wraps `NSManagedObject.object_id(...)`.
    pub fn object_id(&self) -> Result<NSManagedObjectID, CoreDataError> {
        let mut out_object_id = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_get_object_id(
                self.as_ptr(),
                &raw mut out_object_id,
                &raw mut out_error,
            )
        };
        status_result(status, out_error)?;
        unsafe { NSManagedObjectID::from_retained_ptr(out_object_id, "managed object object ID") }
    }

    /// Wraps `NSManagedObject.is_inserted(...)`.
    pub fn is_inserted(&self) -> Result<bool, CoreDataError> {
        self.flag(ffi::cd_managed_object_get_inserted)
    }

    /// Wraps `NSManagedObject.is_updated(...)`.
    pub fn is_updated(&self) -> Result<bool, CoreDataError> {
        self.flag(ffi::cd_managed_object_get_updated)
    }

    /// Wraps `NSManagedObject.is_deleted(...)`.
    pub fn is_deleted(&self) -> Result<bool, CoreDataError> {
        self.flag(ffi::cd_managed_object_get_deleted)
    }

    /// Wraps `NSManagedObject.has_changes(...)`.
    pub fn has_changes(&self) -> Result<bool, CoreDataError> {
        self.flag(ffi::cd_managed_object_get_has_changes)
    }

    /// Wraps `NSManagedObject.has_persistent_changed_values(...)`.
    pub fn has_persistent_changed_values(&self) -> Result<bool, CoreDataError> {
        self.flag(ffi::cd_managed_object_get_has_persistent_changed_values)
    }

    /// Wraps `NSManagedObject.is_fault(...)`.
    pub fn is_fault(&self) -> Result<bool, CoreDataError> {
        self.flag(ffi::cd_managed_object_get_fault)
    }

    /// Wraps `NSManagedObject.has_fault_for_relationship_named(...)`.
    pub fn has_fault_for_relationship_named(
        &self,
        relationship_name: &str,
    ) -> Result<bool, CoreDataError> {
        let relationship_name = cstring_from_str(relationship_name, "relationship name")?;
        let mut out_value = 0_i32;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_has_fault_for_relationship_named(
                self.as_ptr(),
                relationship_name.as_ptr(),
                &raw mut out_value,
                &raw mut out_error,
            )
        };
        status_result(status, out_error)?;
        Ok(out_value != 0)
    }

    /// Wraps `NSManagedObject.object_ids_for_relationship_named(...)`.
    pub fn object_ids_for_relationship_named(
        &self,
        relationship_name: &str,
    ) -> Result<Vec<NSManagedObjectID>, CoreDataError> {
        let relationship_name = cstring_from_str(relationship_name, "relationship name")?;
        let mut out_array = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_object_ids_for_relationship_named(
                self.as_ptr(),
                relationship_name.as_ptr(),
                &raw mut out_array,
                &raw mut out_error,
            )
        };
        status_result(status, out_error)?;
        collect_array(out_array, "managed object relationship object IDs")
    }

    /// Wraps `NSManagedObject.committed_values(...)`.
    pub fn committed_values(
        &self,
        keys: Option<&[&str]>,
    ) -> Result<BTreeMap<String, Value>, CoreDataError> {
        let keys_json = keys
            .map(|keys| {
                keys.iter()
                    .map(|key| (*key).to_string())
                    .collect::<Vec<_>>()
            })
            .as_ref()
            .map(|keys| json_cstring(keys, "managed object committed value keys"))
            .transpose()?;
        let mut out_json = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_committed_values_json(
                self.as_ptr(),
                keys_json
                    .as_ref()
                    .map_or(core::ptr::null(), |json| json.as_c_str().as_ptr()),
                &raw mut out_json,
                &raw mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        value_map_from_json_ptr(out_json, "managed object committed values")
    }

    /// Wraps `NSManagedObject.changed_values(...)`.
    pub fn changed_values(&self) -> Result<BTreeMap<String, Value>, CoreDataError> {
        let mut out_json = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_changed_values_json(
                self.as_ptr(),
                &raw mut out_json,
                &raw mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        value_map_from_json_ptr(out_json, "managed object changed values")
    }

    /// Wraps `NSManagedObject.changed_values_for_current_event(...)`.
    pub fn changed_values_for_current_event(
        &self,
    ) -> Result<BTreeMap<String, Value>, CoreDataError> {
        let mut out_json = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_changed_values_for_current_event_json(
                self.as_ptr(),
                &raw mut out_json,
                &raw mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        value_map_from_json_ptr(out_json, "managed object changed values for current event")
    }
}

impl NSManagedObjectID {
    /// Wraps `NSManagedObjectID.entity(...)`.
    pub fn entity(&self) -> Result<NSEntityDescription, CoreDataError> {
        let ptr = unsafe { ffi::cd_managed_object_id_get_entity(self.as_ptr()) };
        unsafe { NSEntityDescription::from_retained_ptr(ptr, "managed object ID entity") }
    }

    /// Wraps `NSManagedObjectID.is_temporary(...)`.
    pub fn is_temporary(&self) -> bool {
        unsafe { ffi::cd_managed_object_id_get_temporary(self.as_ptr()) != 0 }
    }

    /// Wraps `NSManagedObjectID.uri_representation(...)`.
    pub fn uri_representation(&self) -> Result<String, CoreDataError> {
        let ptr = unsafe { ffi::cd_managed_object_id_get_uri(self.as_ptr()) };
        unsafe { take_string(ptr) }
            .ok_or_else(|| CoreDataError::bridge(-1, "managed object ID URI was nil"))
    }
}
