use std::ffi::CString;
use std::path::{Path, PathBuf};

use crate::error::CoreDataError;
use crate::ffi;
use crate::history::NSPersistentHistoryToken;
use crate::model::NSManagedObjectModel;
use crate::persistent_container::NSPersistentStoreDescription;
use crate::private::{
    collect_array, cstring_from_str, error_from_status, impl_object_wrapper, json_cstring,
    opt_cstring_ptr, path_cstring, take_string,
};
use crate::store::{NSPersistentStoreCoordinator, PersistentStoreOptions};
use crate::value::ValuePayload;

impl_object_wrapper!(NSPersistentStore);

fn encode_options_json(
    options: Option<&PersistentStoreOptions>,
) -> Result<Option<CString>, CoreDataError> {
    let payload = options.map(|options| {
        options
            .iter()
            .map(|(key, value)| (key.clone(), ValuePayload::from(value)))
            .collect::<std::collections::BTreeMap<_, _>>()
    });
    payload
        .as_ref()
        .map(|payload| json_cstring(payload, "persistent store options"))
        .transpose()
}

impl NSPersistentStoreCoordinator {
    /// Wraps `NSPersistentStoreCoordinator.name(...)`.
    pub fn name(&self) -> Option<String> {
        let ptr = unsafe { ffi::cd_persistent_store_coordinator_get_name(self.as_ptr()) };
        unsafe { take_string(ptr) }
    }

    /// Mirrors `NSPersistentStoreCoordinator.name`.
    pub fn set_name(&self, name: Option<&str>) -> Result<(), CoreDataError> {
        let name = name
            .map(|value| cstring_from_str(value, "persistent store coordinator name"))
            .transpose()?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_store_coordinator_set_name(
                self.as_ptr(),
                opt_cstring_ptr(&name),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSPersistentStoreCoordinator.managed_object_model(...)`.
    pub fn managed_object_model(&self) -> Result<NSManagedObjectModel, CoreDataError> {
        let ptr =
            unsafe { ffi::cd_persistent_store_coordinator_managed_object_model(self.as_ptr()) };
        unsafe {
            NSManagedObjectModel::from_retained_ptr(ptr, "persistent store coordinator model")
        }
    }

    /// Wraps `NSPersistentStoreCoordinator.persistent_stores(...)`.
    pub fn persistent_stores(&self) -> Result<Vec<NSPersistentStore>, CoreDataError> {
        let array_ptr =
            unsafe { ffi::cd_persistent_store_coordinator_persistent_stores(self.as_ptr()) };
        collect_array(array_ptr, "persistent store coordinator stores")
    }

    /// Wraps `NSPersistentStoreCoordinator.add_persistent_store_with_description(...)`.
    pub fn add_persistent_store_with_description(
        &self,
        description: &NSPersistentStoreDescription,
        timeout_seconds: i32,
    ) -> Result<Option<NSPersistentStore>, CoreDataError> {
        let mut out_store = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_store_coordinator_add_persistent_store_with_description(
                self.as_ptr(),
                description.as_ptr(),
                timeout_seconds,
                &mut out_store,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        if out_store.is_null() {
            return Ok(None);
        }
        Ok(Some(unsafe {
            NSPersistentStore::from_retained_ptr(out_store, "persistent store")?
        }))
    }

    /// Wraps `NSPersistentStoreCoordinator.remove_persistent_store(...)`.
    pub fn remove_persistent_store(&self, store: &NSPersistentStore) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_store_coordinator_remove_persistent_store(
                self.as_ptr(),
                store.as_ptr(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSPersistentStoreCoordinator.persistent_store_for_url(...)`.
    pub fn persistent_store_for_url(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<Option<NSPersistentStore>, CoreDataError> {
        let path = path_cstring(path.as_ref(), "persistent store URL")?;
        let ptr = unsafe {
            ffi::cd_persistent_store_coordinator_persistent_store_for_url(
                self.as_ptr(),
                path.as_ptr(),
            )
        };
        if ptr.is_null() {
            return Ok(None);
        }
        Ok(Some(unsafe {
            NSPersistentStore::from_retained_ptr(ptr, "persistent store")?
        }))
    }

    /// Wraps `NSPersistentStoreCoordinator.url_for_persistent_store(...)`.
    pub fn url_for_persistent_store(
        &self,
        store: &NSPersistentStore,
    ) -> Result<PathBuf, CoreDataError> {
        let ptr = unsafe {
            ffi::cd_persistent_store_coordinator_url_for_persistent_store(
                self.as_ptr(),
                store.as_ptr(),
            )
        };
        unsafe { take_string(ptr) }
            .map(PathBuf::from)
            .ok_or_else(|| CoreDataError::bridge(-1, "persistent store URL was nil"))
    }

    /// Wraps `NSPersistentStoreCoordinator.current_persistent_history_token(...)`.
    pub fn current_persistent_history_token(
        &self,
    ) -> Result<Option<NSPersistentHistoryToken>, CoreDataError> {
        let ptr = unsafe {
            ffi::cd_persistent_store_coordinator_current_persistent_history_token(self.as_ptr())
        };
        if ptr.is_null() {
            return Ok(None);
        }
        Ok(Some(unsafe {
            NSPersistentHistoryToken::from_retained_ptr(ptr, "persistent history token")?
        }))
    }

    /// Wraps `NSPersistentStoreCoordinator.destroy_persistent_store(...)`.
    pub fn destroy_persistent_store(
        &self,
        path: impl AsRef<Path>,
        store_type: &str,
        options: Option<&PersistentStoreOptions>,
    ) -> Result<(), CoreDataError> {
        let path = path_cstring(path.as_ref(), "persistent store path")?;
        let store_type = cstring_from_str(store_type, "persistent store type")?;
        let options_json = encode_options_json(options)?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_store_coordinator_destroy_persistent_store(
                self.as_ptr(),
                path.as_ptr(),
                store_type.as_ptr(),
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

impl NSPersistentStore {
    /// Wraps `NSPersistentStore.configuration_name(...)`.
    pub fn configuration_name(&self) -> Result<String, CoreDataError> {
        let ptr = unsafe { ffi::cd_persistent_store_get_configuration_name(self.as_ptr()) };
        unsafe { take_string(ptr) }
            .ok_or_else(|| CoreDataError::bridge(-1, "persistent store configuration name was nil"))
    }

    /// Wraps `NSPersistentStore.url(...)`.
    pub fn url(&self) -> Option<PathBuf> {
        let ptr = unsafe { ffi::cd_persistent_store_get_url(self.as_ptr()) };
        unsafe { take_string(ptr) }.map(PathBuf::from)
    }

    /// Mirrors `NSPersistentStore.url`.
    pub fn set_url(&self, path: impl AsRef<Path>) -> Result<(), CoreDataError> {
        let path = path_cstring(path.as_ref(), "persistent store URL")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_store_set_url(self.as_ptr(), path.as_ptr(), &mut out_error)
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSPersistentStore.clear_url(...)`.
    pub fn clear_url(&self) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_store_set_url(self.as_ptr(), core::ptr::null(), &mut out_error)
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSPersistentStore.identifier(...)`.
    pub fn identifier(&self) -> Result<String, CoreDataError> {
        let ptr = unsafe { ffi::cd_persistent_store_get_identifier(self.as_ptr()) };
        unsafe { take_string(ptr) }
            .ok_or_else(|| CoreDataError::bridge(-1, "persistent store identifier was nil"))
    }

    /// Mirrors `NSPersistentStore.identifier`.
    pub fn set_identifier(&self, identifier: Option<&str>) -> Result<(), CoreDataError> {
        let identifier = identifier
            .map(|value| cstring_from_str(value, "persistent store identifier"))
            .transpose()?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_store_set_identifier(
                self.as_ptr(),
                opt_cstring_ptr(&identifier),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSPersistentStore.store_type(...)`.
    pub fn store_type(&self) -> Result<String, CoreDataError> {
        let ptr = unsafe { ffi::cd_persistent_store_get_type(self.as_ptr()) };
        unsafe { take_string(ptr) }
            .ok_or_else(|| CoreDataError::bridge(-1, "persistent store type was nil"))
    }

    /// Wraps `NSPersistentStore.is_read_only(...)`.
    pub fn is_read_only(&self) -> bool {
        unsafe { ffi::cd_persistent_store_get_read_only(self.as_ptr()) != 0 }
    }

    /// Mirrors `NSPersistentStore.read_only`.
    pub fn set_read_only(&self, read_only: bool) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_store_set_read_only(
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
}
