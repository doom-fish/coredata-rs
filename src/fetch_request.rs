use crate::context::NSManagedObjectContext;
use crate::error::CoreDataError;
use crate::ffi;
use crate::managed_object::NSManagedObjectID;
use crate::private::{collect_array, error_from_status, json_cstring, parse_json_ptr, take_string};
use crate::query::NSFetchRequest;
use crate::schema::NSEntityDescription;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
/// Mirrors the corresponding Core Data `FetchRequestResultType` value.
pub enum FetchRequestResultType {
    /// Mirrors `FetchRequestResultType::ManagedObject`.
    ManagedObject,
    /// Mirrors `FetchRequestResultType::ManagedObjectId`.
    ManagedObjectId,
    /// Mirrors `FetchRequestResultType::Dictionary`.
    Dictionary,
    /// Mirrors `FetchRequestResultType::Count`.
    Count,
    /// Mirrors `FetchRequestResultType::Unknown`.
    Unknown(u64),
}

impl FetchRequestResultType {
    const fn from_raw(raw: u64) -> Self {
        match raw {
            0 => Self::ManagedObject,
            1 => Self::ManagedObjectId,
            2 => Self::Dictionary,
            4 => Self::Count,
            other => Self::Unknown(other),
        }
    }

    const fn as_raw(self) -> u64 {
        match self {
            Self::ManagedObject => 0,
            Self::ManagedObjectId => 1,
            Self::Dictionary => 2,
            Self::Count => 4,
            Self::Unknown(raw) => raw,
        }
    }
}

impl NSFetchRequest {
    /// Wraps `NSFetchRequest.entity(...)`.
    pub fn entity(&self) -> Result<Option<NSEntityDescription>, CoreDataError> {
        let ptr = unsafe { ffi::cd_fetch_request_get_entity(self.as_ptr()) };
        if ptr.is_null() {
            return Ok(None);
        }
        Ok(Some(unsafe {
            NSEntityDescription::from_retained_ptr(ptr, "fetch request entity")?
        }))
    }

    /// Mirrors `NSFetchRequest.entity`.
    pub fn set_entity(&self, entity: Option<&NSEntityDescription>) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_fetch_request_set_entity(
                self.as_ptr(),
                entity.map_or(core::ptr::null_mut(), NSEntityDescription::as_ptr),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSFetchRequest.entity_name(...)`.
    pub fn entity_name(&self) -> Option<String> {
        let ptr = unsafe { ffi::cd_fetch_request_get_entity_name(self.as_ptr()) };
        unsafe { take_string(ptr) }
    }

    /// Wraps `NSFetchRequest.result_type(...)`.
    pub fn result_type(&self) -> FetchRequestResultType {
        FetchRequestResultType::from_raw(unsafe {
            ffi::cd_fetch_request_get_result_type(self.as_ptr())
        })
    }

    /// Mirrors `NSFetchRequest.result_type`.
    pub fn set_result_type(
        &self,
        result_type: FetchRequestResultType,
    ) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_fetch_request_set_result_type(
                self.as_ptr(),
                result_type.as_raw(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSFetchRequest.includes_subentities(...)`.
    pub fn includes_subentities(&self) -> bool {
        unsafe { ffi::cd_fetch_request_get_includes_subentities(self.as_ptr()) != 0 }
    }

    /// Mirrors `NSFetchRequest.includes_subentities`.
    pub fn set_includes_subentities(&self, includes_subentities: bool) {
        unsafe {
            ffi::cd_fetch_request_set_includes_subentities(
                self.as_ptr(),
                i32::from(includes_subentities),
            );
        }
    }

    /// Wraps `NSFetchRequest.includes_property_values(...)`.
    pub fn includes_property_values(&self) -> bool {
        unsafe { ffi::cd_fetch_request_get_includes_property_values(self.as_ptr()) != 0 }
    }

    /// Mirrors `NSFetchRequest.includes_property_values`.
    pub fn set_includes_property_values(&self, includes_property_values: bool) {
        unsafe {
            ffi::cd_fetch_request_set_includes_property_values(
                self.as_ptr(),
                i32::from(includes_property_values),
            );
        }
    }

    /// Wraps `NSFetchRequest.returns_objects_as_faults(...)`.
    pub fn returns_objects_as_faults(&self) -> bool {
        unsafe { ffi::cd_fetch_request_get_returns_objects_as_faults(self.as_ptr()) != 0 }
    }

    /// Mirrors `NSFetchRequest.returns_objects_as_faults`.
    pub fn set_returns_objects_as_faults(&self, returns_objects_as_faults: bool) {
        unsafe {
            ffi::cd_fetch_request_set_returns_objects_as_faults(
                self.as_ptr(),
                i32::from(returns_objects_as_faults),
            );
        }
    }

    /// Wraps `NSFetchRequest.relationship_key_paths_for_prefetching(...)`.
    pub fn relationship_key_paths_for_prefetching(&self) -> Result<Vec<String>, CoreDataError> {
        let mut out_json = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_fetch_request_get_relationship_key_paths_for_prefetching_json(
                self.as_ptr(),
                &mut out_json,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { parse_json_ptr(out_json, "fetch request relationship key paths") }
    }

    /// Mirrors `NSFetchRequest.relationship_key_paths_for_prefetching`.
    pub fn set_relationship_key_paths_for_prefetching(
        &self,
        key_paths: &[impl AsRef<str>],
    ) -> Result<(), CoreDataError> {
        let key_paths = key_paths
            .iter()
            .map(|key_path| key_path.as_ref().to_string())
            .collect::<Vec<_>>();
        let key_paths_json = json_cstring(&key_paths, "fetch request relationship key paths")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_fetch_request_set_relationship_key_paths_for_prefetching_json(
                self.as_ptr(),
                key_paths_json.as_ptr(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSFetchRequest.includes_pending_changes(...)`.
    pub fn includes_pending_changes(&self) -> bool {
        unsafe { ffi::cd_fetch_request_get_includes_pending_changes(self.as_ptr()) != 0 }
    }

    /// Mirrors `NSFetchRequest.includes_pending_changes`.
    pub fn set_includes_pending_changes(&self, includes_pending_changes: bool) {
        unsafe {
            ffi::cd_fetch_request_set_includes_pending_changes(
                self.as_ptr(),
                i32::from(includes_pending_changes),
            );
        }
    }

    /// Wraps `NSFetchRequest.returns_distinct_results(...)`.
    pub fn returns_distinct_results(&self) -> bool {
        unsafe { ffi::cd_fetch_request_get_returns_distinct_results(self.as_ptr()) != 0 }
    }

    /// Mirrors `NSFetchRequest.returns_distinct_results`.
    pub fn set_returns_distinct_results(&self, returns_distinct_results: bool) {
        unsafe {
            ffi::cd_fetch_request_set_returns_distinct_results(
                self.as_ptr(),
                i32::from(returns_distinct_results),
            );
        }
    }

    /// Wraps `NSFetchRequest.fetch_batch_size(...)`.
    pub fn fetch_batch_size(&self) -> usize {
        unsafe { ffi::cd_fetch_request_get_fetch_batch_size(self.as_ptr()) as usize }
    }

    /// Mirrors `NSFetchRequest.fetch_batch_size`.
    pub fn set_fetch_batch_size(&self, fetch_batch_size: usize) -> Result<(), CoreDataError> {
        let fetch_batch_size = u64::try_from(fetch_batch_size)
            .map_err(|_| CoreDataError::bridge(-1, "fetch batch size overflow"))?;
        unsafe { ffi::cd_fetch_request_set_fetch_batch_size(self.as_ptr(), fetch_batch_size) }
        Ok(())
    }

    /// Wraps `NSFetchRequest.should_refresh_refetched_objects(...)`.
    pub fn should_refresh_refetched_objects(&self) -> bool {
        unsafe { ffi::cd_fetch_request_get_should_refresh_refetched_objects(self.as_ptr()) != 0 }
    }

    /// Mirrors `NSFetchRequest.should_refresh_refetched_objects`.
    pub fn set_should_refresh_refetched_objects(&self, should_refresh: bool) {
        unsafe {
            ffi::cd_fetch_request_set_should_refresh_refetched_objects(
                self.as_ptr(),
                i32::from(should_refresh),
            );
        }
    }

    /// Wraps `NSFetchRequest.execute_object_ids(...)`.
    pub fn execute_object_ids(
        &self,
        context: &NSManagedObjectContext,
    ) -> Result<Vec<NSManagedObjectID>, CoreDataError> {
        let mut out_array = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_fetch_request_execute_object_ids(
                self.as_ptr(),
                context.as_ptr(),
                &mut out_array,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        collect_array(out_array, "fetch request object IDs")
    }
}
