use crate::error::CoreDataError;
use crate::ffi;
use crate::private::{error_from_status, take_string};
use crate::schema::NSRelationshipDescription;

impl NSRelationshipDescription {
    /// Wraps `NSRelationshipDescription.is_to_many(...)`.
    pub fn is_to_many(&self) -> bool {
        unsafe { ffi::cd_relationship_description_get_to_many(self.as_ptr()) != 0 }
    }

    /// Wraps `NSRelationshipDescription.is_ordered(...)`.
    pub fn is_ordered(&self) -> bool {
        unsafe { ffi::cd_relationship_description_get_ordered(self.as_ptr()) != 0 }
    }

    /// Mirrors `NSRelationshipDescription.ordered`.
    pub fn set_ordered(&self, ordered: bool) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_relationship_description_set_ordered(
                self.as_ptr(),
                i32::from(ordered),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSRelationshipDescription.version_hash(...)`.
    pub fn version_hash(&self) -> Result<String, CoreDataError> {
        let ptr = unsafe { ffi::cd_relationship_description_get_version_hash(self.as_ptr()) };
        unsafe { take_string(ptr) }.ok_or_else(|| {
            CoreDataError::bridge(-1, "relationship description version hash was nil")
        })
    }
}
