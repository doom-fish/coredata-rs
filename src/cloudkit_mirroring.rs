use crate::context::NSManagedObjectContext;
use crate::error::CoreDataError;
use crate::ffi;
use crate::model::NSManagedObjectModel;
use crate::persistent_container::NSPersistentStoreDescription;
use crate::private::{
    collect_array, cstring_from_str, error_from_status, impl_object_wrapper, take_string,
};
use crate::store::NSPersistentStoreCoordinator;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CloudKitDatabaseScope {
    Public,
    Private,
    Shared,
    Unknown(i64),
}

impl CloudKitDatabaseScope {
    const fn from_raw(raw: i64) -> Self {
        match raw {
            1 => Self::Public,
            2 => Self::Private,
            3 => Self::Shared,
            other => Self::Unknown(other),
        }
    }

    const fn as_raw(self) -> i64 {
        match self {
            Self::Public => 1,
            Self::Private => 2,
            Self::Shared => 3,
            Self::Unknown(raw) => raw,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CloudKitSchemaInitializationOptions(u64);

impl CloudKitSchemaInitializationOptions {
    pub const NONE: Self = Self(0);
    pub const DRY_RUN: Self = Self(1 << 1);
    pub const PRINT_SCHEMA: Self = Self(1 << 2);

    pub const fn bits(self) -> u64 {
        self.0
    }

    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

impl_object_wrapper!(NSPersistentCloudKitContainerOptions);
impl_object_wrapper!(NSPersistentCloudKitContainer);

impl NSPersistentCloudKitContainerOptions {
    pub fn new(container_identifier: &str) -> Result<Self, CoreDataError> {
        let container_identifier =
            cstring_from_str(container_identifier, "CloudKit container identifier")?;
        let mut out_options = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_cloudkit_container_options_new(
                container_identifier.as_ptr(),
                &mut out_options,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_options, "CloudKit container options") }
    }

    pub fn container_identifier(&self) -> Result<String, CoreDataError> {
        let ptr = unsafe {
            ffi::cd_persistent_cloudkit_container_options_get_container_identifier(self.as_ptr())
        };
        unsafe { take_string(ptr) }
            .ok_or_else(|| CoreDataError::bridge(-1, "CloudKit container identifier was nil"))
    }

    pub fn database_scope(&self) -> CloudKitDatabaseScope {
        CloudKitDatabaseScope::from_raw(unsafe {
            ffi::cd_persistent_cloudkit_container_options_get_database_scope(self.as_ptr())
        })
    }

    pub fn set_database_scope(
        &self,
        database_scope: CloudKitDatabaseScope,
    ) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_cloudkit_container_options_set_database_scope(
                self.as_ptr(),
                database_scope.as_raw(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }
}

impl NSPersistentStoreDescription {
    pub fn cloudkit_container_options(
        &self,
    ) -> Result<Option<NSPersistentCloudKitContainerOptions>, CoreDataError> {
        let ptr = unsafe {
            ffi::cd_persistent_store_description_get_cloudkit_container_options(self.as_ptr())
        };
        if ptr.is_null() {
            return Ok(None);
        }
        Ok(Some(unsafe {
            NSPersistentCloudKitContainerOptions::from_retained_ptr(
                ptr,
                "CloudKit container options",
            )?
        }))
    }

    pub fn set_cloudkit_container_options(
        &self,
        options: Option<&NSPersistentCloudKitContainerOptions>,
    ) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_store_description_set_cloudkit_container_options(
                self.as_ptr(),
                options.map_or(
                    core::ptr::null_mut(),
                    NSPersistentCloudKitContainerOptions::as_ptr,
                ),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }
}

impl NSPersistentCloudKitContainer {
    pub fn new(name: &str, model: &NSManagedObjectModel) -> Result<Self, CoreDataError> {
        let name = cstring_from_str(name, "CloudKit container name")?;
        let mut out_container = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_cloudkit_container_new(
                name.as_ptr(),
                model.as_ptr(),
                &mut out_container,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_container, "CloudKit container") }
    }

    pub fn name(&self) -> Result<String, CoreDataError> {
        let ptr = unsafe { ffi::cd_persistent_cloudkit_container_get_name(self.as_ptr()) };
        unsafe { take_string(ptr) }
            .ok_or_else(|| CoreDataError::bridge(-1, "CloudKit container name was nil"))
    }

    pub fn managed_object_model(&self) -> Result<NSManagedObjectModel, CoreDataError> {
        let ptr =
            unsafe { ffi::cd_persistent_cloudkit_container_managed_object_model(self.as_ptr()) };
        unsafe { NSManagedObjectModel::from_retained_ptr(ptr, "CloudKit container model") }
    }

    pub fn persistent_store_coordinator(
        &self,
    ) -> Result<NSPersistentStoreCoordinator, CoreDataError> {
        let ptr = unsafe {
            ffi::cd_persistent_cloudkit_container_persistent_store_coordinator(self.as_ptr())
        };
        unsafe {
            NSPersistentStoreCoordinator::from_retained_ptr(ptr, "CloudKit container coordinator")
        }
    }

    pub fn persistent_store_descriptions(
        &self,
    ) -> Result<Vec<NSPersistentStoreDescription>, CoreDataError> {
        let array_ptr = unsafe {
            ffi::cd_persistent_cloudkit_container_persistent_store_descriptions(self.as_ptr())
        };
        collect_array(
            array_ptr,
            "CloudKit container persistent store descriptions",
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
            CoreDataError::bridge(-1, "CloudKit container description count overflow")
        })?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_cloudkit_container_set_persistent_store_descriptions(
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

    pub fn load_persistent_stores_with_timeout(
        &self,
        timeout_seconds: i32,
    ) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_cloudkit_container_load_persistent_stores(
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

    pub fn load_persistent_stores(&self) -> Result<(), CoreDataError> {
        self.load_persistent_stores_with_timeout(30)
    }

    pub fn view_context(&self) -> Result<NSManagedObjectContext, CoreDataError> {
        let ptr = unsafe { ffi::cd_persistent_cloudkit_container_view_context(self.as_ptr()) };
        unsafe { NSManagedObjectContext::from_retained_ptr(ptr, "CloudKit container view context") }
    }

    pub fn new_background_context(&self) -> Result<NSManagedObjectContext, CoreDataError> {
        let ptr =
            unsafe { ffi::cd_persistent_cloudkit_container_new_background_context(self.as_ptr()) };
        unsafe {
            NSManagedObjectContext::from_retained_ptr(ptr, "CloudKit container background context")
        }
    }

    pub fn initialize_schema(
        &self,
        options: CloudKitSchemaInitializationOptions,
    ) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_cloudkit_container_initialize_schema(
                self.as_ptr(),
                options.bits(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }
}
