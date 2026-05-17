use std::collections::BTreeMap;
use std::path::Path;

use crate::context::NSManagedObjectContext;
use crate::error::CoreDataError;
use crate::ffi;
use crate::model::NSManagedObjectModel;
use crate::private::{
    cstring_from_str, error_from_status, impl_object_wrapper, json_cstring, opt_cstring_ptr,
    optional_cstring_from_str, path_cstring,
};
use crate::value::{Value, ValuePayload};

pub mod store_types {
    pub const SQLITE: &str = "SQLite";
    pub const XML: &str = "XML";
    pub const BINARY: &str = "Binary";
    pub const IN_MEMORY: &str = "InMemory";
}

pub type PersistentStoreOptions = BTreeMap<String, Value>;

impl_object_wrapper!(NSPersistentStoreCoordinator);
impl_object_wrapper!(NSPersistentContainer);

impl NSPersistentStoreCoordinator {
    pub fn new(model: &NSManagedObjectModel) -> Result<Self, CoreDataError> {
        let mut out_coordinator = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_store_coordinator_new(
                model.as_ptr(),
                &mut out_coordinator,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_coordinator, "persistent store coordinator") }
    }

    pub fn add_persistent_store<P>(
        &self,
        store_type: &str,
        configuration_name: Option<&str>,
        at: Option<P>,
        options: Option<&PersistentStoreOptions>,
    ) -> Result<(), CoreDataError>
    where
        P: AsRef<Path>,
    {
        let store_type = cstring_from_str(store_type, "persistent store type")?;
        let configuration_name =
            optional_cstring_from_str(configuration_name, "persistent store configuration")?;
        let store_url = at
            .map(|path| path_cstring(path.as_ref(), "persistent store URL"))
            .transpose()?;
        let options_payload = options.map(|options| {
            options
                .iter()
                .map(|(key, value)| (key.clone(), ValuePayload::from(value)))
                .collect::<BTreeMap<_, _>>()
        });
        let options_json = options_payload
            .as_ref()
            .map(|payload| json_cstring(payload, "persistent store options"))
            .transpose()?;

        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_store_coordinator_add_persistent_store(
                self.as_ptr(),
                store_type.as_ptr(),
                opt_cstring_ptr(&configuration_name),
                opt_cstring_ptr(&store_url),
                opt_cstring_ptr(&options_json),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }
}

impl NSPersistentContainer {
    pub fn new(name: impl AsRef<str>, model: &NSManagedObjectModel) -> Result<Self, CoreDataError> {
        let name = cstring_from_str(name.as_ref(), "persistent container name")?;
        let mut out_container = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_container_new(
                name.as_ptr(),
                model.as_ptr(),
                &mut out_container,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_container, "persistent container") }
    }

    pub fn load_persistent_stores(&self) -> Result<(), CoreDataError> {
        self.load_persistent_stores_with_timeout(30)
    }

    pub fn load_persistent_stores_with_timeout(
        &self,
        timeout_seconds: i32,
    ) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_container_load_persistent_stores(
                self.as_ptr(),
                timeout_seconds,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn view_context(&self) -> Result<NSManagedObjectContext, CoreDataError> {
        let ptr = unsafe { ffi::cd_persistent_container_view_context(self.as_ptr()) };
        unsafe {
            NSManagedObjectContext::from_retained_ptr(ptr, "persistent container view context")
        }
    }

    pub fn new_background_context(&self) -> Result<NSManagedObjectContext, CoreDataError> {
        let ptr = unsafe { ffi::cd_persistent_container_new_background_context(self.as_ptr()) };
        unsafe {
            NSManagedObjectContext::from_retained_ptr(
                ptr,
                "persistent container background context",
            )
        }
    }
}
