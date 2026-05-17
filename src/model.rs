use std::path::Path;

use crate::error::CoreDataError;
use crate::ffi;
use crate::private::{
    collect_array, error_from_status, impl_object_wrapper, path_cstring, take_string,
};
use crate::schema::NSEntityDescription;

impl_object_wrapper!(NSManagedObjectModel);

impl NSManagedObjectModel {
    pub fn new() -> Result<Self, CoreDataError> {
        let mut out_model = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe { ffi::cd_managed_object_model_new(&mut out_model, &mut out_error) };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_model, "managed object model") }
    }

    pub fn from_url(path: impl AsRef<Path>) -> Result<Self, CoreDataError> {
        let path = path_cstring(path.as_ref(), "managed object model URL")?;
        let mut out_model = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_model_from_url(path.as_ptr(), &mut out_model, &mut out_error)
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_model, "managed object model") }
    }

    pub fn add_entity(&self, entity: &NSEntityDescription) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_model_add_entity(self.as_ptr(), entity.as_ptr(), &mut out_error)
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn entities(&self) -> Result<Vec<NSEntityDescription>, CoreDataError> {
        let array_ptr = unsafe { ffi::cd_managed_object_model_entities(self.as_ptr()) };
        collect_array(array_ptr, "managed object model entities")
    }

    pub fn entity_named(&self, name: &str) -> Result<Option<NSEntityDescription>, CoreDataError> {
        let entities = self.entities()?;
        entities
            .into_iter()
            .find_map(|entity| match entity.name() {
                Ok(entity_name) if entity_name == name => Some(Ok(entity)),
                Ok(_) => None,
                Err(error) => Some(Err(error)),
            })
            .transpose()
    }

    pub fn version_checksum(&self) -> Option<String> {
        let ptr = unsafe { ffi::cd_managed_object_model_get_version_checksum(self.as_ptr()) };
        unsafe { take_string(ptr) }
    }
}
