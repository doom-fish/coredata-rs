use crate::context::{NSManagedObject, NSManagedObjectContext};
use crate::error::CoreDataError;
use crate::ffi;
use crate::private::{cstring_from_str, error_from_status, impl_object_wrapper, json_cstring};
use crate::value::{Value, ValuePayload};

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
/// Wraps `SortDescriptor`.
pub struct SortDescriptor {
    /// Mirrors `SortDescriptor.key`.
    pub key: String,
    /// Mirrors `SortDescriptor.ascending`.
    pub ascending: bool,
}

impl SortDescriptor {
    /// Wraps `SortDescriptor.ascending(...)`.
    pub fn ascending(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            ascending: true,
        }
    }

    /// Wraps `SortDescriptor.descending(...)`.
    pub fn descending(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            ascending: false,
        }
    }
}

impl_object_wrapper!(NSPredicate);
impl_object_wrapper!(NSFetchRequest);

impl NSPredicate {
    /// Wraps `NSPredicate.init(...)`.
    pub fn from_format(
        format: impl AsRef<str>,
        arguments: &[Value],
    ) -> Result<Self, CoreDataError> {
        let format = cstring_from_str(format.as_ref(), "predicate format")?;
        let payload = arguments.iter().map(ValuePayload::from).collect::<Vec<_>>();
        let arguments_json = json_cstring(&payload, "predicate arguments")?;
        let mut out_predicate = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_predicate_new_with_format(
                format.as_ptr(),
                arguments_json.as_ptr(),
                &mut out_predicate,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_predicate, "predicate") }
    }
}

impl NSFetchRequest {
    /// Wraps `NSFetchRequest.init(...)`.
    pub fn new(entity_name: impl AsRef<str>) -> Result<Self, CoreDataError> {
        let entity_name = cstring_from_str(entity_name.as_ref(), "fetch request entity name")?;
        let mut out_request = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_fetch_request_new(entity_name.as_ptr(), &mut out_request, &mut out_error)
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_request, "fetch request") }
    }

    /// Mirrors `NSFetchRequest.predicate`.
    pub fn set_predicate(&self, predicate: Option<&NSPredicate>) {
        unsafe {
            ffi::cd_fetch_request_set_predicate(
                self.as_ptr(),
                predicate.map_or(core::ptr::null_mut(), NSPredicate::as_ptr),
            );
        }
    }

    /// Mirrors `NSFetchRequest.sort_descriptors`.
    pub fn set_sort_descriptors(
        &self,
        descriptors: &[SortDescriptor],
    ) -> Result<(), CoreDataError> {
        let descriptors_json = json_cstring(descriptors, "sort descriptors")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_fetch_request_set_sort_descriptors_json(
                self.as_ptr(),
                descriptors_json.as_ptr(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Mirrors `NSFetchRequest.fetch_limit`.
    pub fn set_fetch_limit(&self, fetch_limit: usize) -> Result<(), CoreDataError> {
        let fetch_limit = u64::try_from(fetch_limit)
            .map_err(|_| CoreDataError::bridge(-1, "fetch limit overflow"))?;
        unsafe {
            ffi::cd_fetch_request_set_fetch_limit(self.as_ptr(), fetch_limit);
        }
        Ok(())
    }

    /// Mirrors `NSFetchRequest.fetch_offset`.
    pub fn set_fetch_offset(&self, fetch_offset: usize) -> Result<(), CoreDataError> {
        let fetch_offset = u64::try_from(fetch_offset)
            .map_err(|_| CoreDataError::bridge(-1, "fetch offset overflow"))?;
        unsafe {
            ffi::cd_fetch_request_set_fetch_offset(self.as_ptr(), fetch_offset);
        }
        Ok(())
    }

    /// Wraps `NSFetchRequest.execute(...)`.
    pub fn execute(
        &self,
        context: &NSManagedObjectContext,
    ) -> Result<Vec<NSManagedObject>, CoreDataError> {
        context.fetch(self)
    }
}
