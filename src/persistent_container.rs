use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::error::CoreDataError;
use crate::ffi;
use crate::model::NSManagedObjectModel;
use crate::private::{
    collect_array, cstring_from_str, error_from_status, impl_object_wrapper, json_cstring,
    opt_cstring_ptr, parse_json_ptr, path_cstring, take_string,
};
use crate::store::NSPersistentContainer;
use crate::store::NSPersistentStoreCoordinator;
use crate::value::{Value, ValuePayload};

pub mod option_keys {
    pub const READ_ONLY: &str = "NSReadOnlyPersistentStoreOption";
    pub const MIGRATE_PERSISTENT_STORES_AUTOMATICALLY: &str =
        "NSMigratePersistentStoresAutomaticallyOption";
    pub const INFER_MAPPING_MODEL_AUTOMATICALLY: &str = "NSInferMappingModelAutomaticallyOption";
    pub const PERSISTENT_HISTORY_TRACKING: &str = "NSPersistentHistoryTrackingKey";
    pub const REMOTE_CHANGE_NOTIFICATION_POST: &str =
        "NSPersistentStoreRemoteChangeNotificationPostOptionKey";
}

impl_object_wrapper!(NSPersistentStoreDescription);

impl NSPersistentStoreDescription {
    pub fn new() -> Result<Self, CoreDataError> {
        let mut out_description = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_store_description_new(
                core::ptr::null(),
                &mut out_description,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_description, "persistent store description") }
    }

    pub fn with_url(path: impl AsRef<Path>) -> Result<Self, CoreDataError> {
        let path = path_cstring(path.as_ref(), "persistent store description URL")?;
        let mut out_description = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_store_description_new(
                path.as_ptr(),
                &mut out_description,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_description, "persistent store description") }
    }

    pub fn store_type(&self) -> Result<String, CoreDataError> {
        let ptr = unsafe { ffi::cd_persistent_store_description_get_type(self.as_ptr()) };
        unsafe { take_string(ptr) }
            .ok_or_else(|| CoreDataError::bridge(-1, "persistent store description type was nil"))
    }

    pub fn set_store_type(&self, store_type: &str) -> Result<(), CoreDataError> {
        let store_type = cstring_from_str(store_type, "persistent store description type")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_store_description_set_type(
                self.as_ptr(),
                store_type.as_ptr(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn configuration(&self) -> Option<String> {
        let ptr = unsafe { ffi::cd_persistent_store_description_get_configuration(self.as_ptr()) };
        unsafe { take_string(ptr) }
    }

    pub fn set_configuration(&self, configuration: Option<&str>) -> Result<(), CoreDataError> {
        let configuration = configuration
            .map(|value| cstring_from_str(value, "persistent store configuration"))
            .transpose()?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_store_description_set_configuration(
                self.as_ptr(),
                opt_cstring_ptr(&configuration),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn url(&self) -> Option<PathBuf> {
        let ptr = unsafe { ffi::cd_persistent_store_description_get_url(self.as_ptr()) };
        unsafe { take_string(ptr) }.map(PathBuf::from)
    }

    pub fn set_url(&self, path: impl AsRef<Path>) -> Result<(), CoreDataError> {
        let path = path_cstring(path.as_ref(), "persistent store description URL")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_store_description_set_url(
                self.as_ptr(),
                path.as_ptr(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn clear_url(&self) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_store_description_set_url(
                self.as_ptr(),
                core::ptr::null(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn is_read_only(&self) -> bool {
        unsafe { ffi::cd_persistent_store_description_get_read_only(self.as_ptr()) != 0 }
    }

    pub fn set_read_only(&self, read_only: bool) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_store_description_set_read_only(
                self.as_ptr(),
                i32::from(read_only),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn timeout(&self) -> f64 {
        unsafe { ffi::cd_persistent_store_description_get_timeout(self.as_ptr()) }
    }

    pub fn set_timeout(&self, timeout: f64) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_store_description_set_timeout(self.as_ptr(), timeout, &mut out_error)
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn sqlite_pragmas(&self) -> Result<BTreeMap<String, String>, CoreDataError> {
        let mut out_json = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_store_description_sqlite_pragmas_json(
                self.as_ptr(),
                &mut out_json,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { parse_json_ptr(out_json, "persistent store sqlite pragmas") }
    }

    pub fn set_sqlite_pragma(&self, name: &str, value: Option<&str>) -> Result<(), CoreDataError> {
        let name = cstring_from_str(name, "persistent store sqlite pragma name")?;
        let value = value
            .map(|value| cstring_from_str(value, "persistent store sqlite pragma value"))
            .transpose()?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_store_description_set_sqlite_pragma(
                self.as_ptr(),
                name.as_ptr(),
                opt_cstring_ptr(&value),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn should_add_asynchronously(&self) -> bool {
        unsafe {
            ffi::cd_persistent_store_description_get_should_add_asynchronously(self.as_ptr()) != 0
        }
    }

    pub fn set_should_add_asynchronously(&self, asynchronous: bool) {
        unsafe {
            ffi::cd_persistent_store_description_set_should_add_asynchronously(
                self.as_ptr(),
                i32::from(asynchronous),
            );
        }
    }

    pub fn should_migrate_automatically(&self) -> bool {
        unsafe {
            ffi::cd_persistent_store_description_get_should_migrate_automatically(self.as_ptr())
                != 0
        }
    }

    pub fn set_should_migrate_automatically(&self, automatically_migrate: bool) {
        unsafe {
            ffi::cd_persistent_store_description_set_should_migrate_automatically(
                self.as_ptr(),
                i32::from(automatically_migrate),
            );
        }
    }

    pub fn should_infer_mapping_model_automatically(&self) -> bool {
        unsafe {
            ffi::cd_persistent_store_description_get_should_infer_mapping_model_automatically(
                self.as_ptr(),
            ) != 0
        }
    }

    pub fn set_should_infer_mapping_model_automatically(&self, automatically_infer: bool) {
        unsafe {
            ffi::cd_persistent_store_description_set_should_infer_mapping_model_automatically(
                self.as_ptr(),
                i32::from(automatically_infer),
            );
        }
    }

    pub fn set_option(&self, key: &str, value: Option<Value>) -> Result<(), CoreDataError> {
        let key = cstring_from_str(key, "persistent store option key")?;
        let value_json = value
            .map(ValuePayload::from)
            .as_ref()
            .map(|payload| json_cstring(payload, "persistent store option value"))
            .transpose()?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_store_description_set_option_json(
                self.as_ptr(),
                key.as_ptr(),
                opt_cstring_ptr(&value_json),
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
    pub fn default_directory() -> Result<PathBuf, CoreDataError> {
        let ptr = unsafe { ffi::cd_persistent_container_default_directory() };
        unsafe { take_string(ptr) }
            .map(PathBuf::from)
            .ok_or_else(|| {
                CoreDataError::bridge(-1, "persistent container default directory was nil")
            })
    }

    pub fn name(&self) -> Result<String, CoreDataError> {
        let ptr = unsafe { ffi::cd_persistent_container_get_name(self.as_ptr()) };
        unsafe { take_string(ptr) }
            .ok_or_else(|| CoreDataError::bridge(-1, "persistent container name was nil"))
    }

    pub fn managed_object_model(&self) -> Result<NSManagedObjectModel, CoreDataError> {
        let ptr = unsafe { ffi::cd_persistent_container_managed_object_model(self.as_ptr()) };
        unsafe { NSManagedObjectModel::from_retained_ptr(ptr, "persistent container model") }
    }

    pub fn persistent_store_coordinator(
        &self,
    ) -> Result<NSPersistentStoreCoordinator, CoreDataError> {
        let ptr =
            unsafe { ffi::cd_persistent_container_persistent_store_coordinator(self.as_ptr()) };
        unsafe {
            NSPersistentStoreCoordinator::from_retained_ptr(
                ptr,
                "persistent container persistent store coordinator",
            )
        }
    }

    pub fn persistent_store_descriptions(
        &self,
    ) -> Result<Vec<NSPersistentStoreDescription>, CoreDataError> {
        let array_ptr =
            unsafe { ffi::cd_persistent_container_persistent_store_descriptions(self.as_ptr()) };
        collect_array(
            array_ptr,
            "persistent container persistent store descriptions",
        )
    }

    pub fn set_persistent_store_descriptions(
        &self,
        descriptions: &[&NSPersistentStoreDescription],
    ) -> Result<(), CoreDataError> {
        let raw_descriptions = descriptions
            .iter()
            .map(|description| description.as_ptr())
            .collect::<Vec<_>>();
        let count = i32::try_from(raw_descriptions.len()).map_err(|_| {
            CoreDataError::bridge(-1, "persistent container description count overflow")
        })?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_container_set_persistent_store_descriptions(
                self.as_ptr(),
                raw_descriptions.as_ptr(),
                count,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }
}
