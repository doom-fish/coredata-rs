use std::collections::BTreeMap;
use std::path::Path;

use crate::context::NSManagedObjectContext;
use crate::error::CoreDataError;
use crate::ffi;
use crate::model::NSManagedObjectModel;
use crate::private::{
    cstring_from_str, error_from_status, impl_object_wrapper, json_cstring, opt_cstring_ptr,
    parse_json_ptr, path_cstring,
};
use crate::store::PersistentStoreOptions;
use crate::value::ValuePayload;

/// Core Data items for migration expression keys.
pub mod migration_expression_keys {
    /// Mirrors `NSMigrationManagerKey`.
    pub const MANAGER: &str = "NSMigrationManagerKey";
    /// Mirrors `NSMigrationSourceObjectKey`.
    pub const SOURCE_OBJECT: &str = "NSMigrationSourceObjectKey";
    /// Mirrors `NSMigrationDestinationObjectKey`.
    pub const DESTINATION_OBJECT: &str = "NSMigrationDestinationObjectKey";
    /// Mirrors `NSMigrationEntityMappingKey`.
    pub const ENTITY_MAPPING: &str = "NSMigrationEntityMappingKey";
    /// Mirrors `NSMigrationPropertyMappingKey`.
    pub const PROPERTY_MAPPING: &str = "NSMigrationPropertyMappingKey";
    /// Mirrors `NSMigrationEntityPolicyKey`.
    pub const ENTITY_POLICY: &str = "NSMigrationEntityPolicyKey";
}

impl_object_wrapper!(NSMappingModel);
impl_object_wrapper!(NSMigrationManager);

fn encode_options_json(
    options: Option<&PersistentStoreOptions>,
    context: &str,
) -> Result<Option<std::ffi::CString>, CoreDataError> {
    let payload = options.map(|options| {
        options
            .iter()
            .map(|(key, value)| (key.clone(), ValuePayload::from(value)))
            .collect::<BTreeMap<_, _>>()
    });
    payload
        .as_ref()
        .map(|payload| json_cstring(payload, context))
        .transpose()
}

impl NSMappingModel {
    /// Wraps `NSMappingModel.inferred(...)`.
    pub fn inferred(
        source_model: &NSManagedObjectModel,
        destination_model: &NSManagedObjectModel,
    ) -> Result<Self, CoreDataError> {
        let mut out_model = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_mapping_model_inferred(
                source_model.as_ptr(),
                destination_model.as_ptr(),
                &mut out_model,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_model, "mapping model") }
    }

    /// Wraps `NSMappingModel.init(...)`.
    pub fn from_url(path: impl AsRef<Path>) -> Result<Option<Self>, CoreDataError> {
        let path = path_cstring(path.as_ref(), "mapping model URL")?;
        let mut out_model = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_mapping_model_from_url(path.as_ptr(), &mut out_model, &mut out_error)
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        if out_model.is_null() {
            return Ok(None);
        }
        Ok(Some(unsafe {
            Self::from_retained_ptr(out_model, "mapping model")?
        }))
    }

    /// Wraps `NSMappingModel.entity_mapping_names(...)`.
    pub fn entity_mapping_names(&self) -> Result<Vec<String>, CoreDataError> {
        let mut out_json = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_mapping_model_get_entity_mapping_names_json(
                self.as_ptr(),
                &mut out_json,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { parse_json_ptr(out_json, "mapping model entity mapping names") }
    }
}

impl NSMigrationManager {
    /// Wraps `NSMigrationManager.init(...)`.
    pub fn new(
        source_model: &NSManagedObjectModel,
        destination_model: &NSManagedObjectModel,
    ) -> Result<Self, CoreDataError> {
        let mut out_manager = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_migration_manager_new(
                source_model.as_ptr(),
                destination_model.as_ptr(),
                &mut out_manager,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_manager, "migration manager") }
    }

    /// Wraps `NSMigrationManager.source_model(...)`.
    pub fn source_model(&self) -> Result<NSManagedObjectModel, CoreDataError> {
        let ptr = unsafe { ffi::cd_migration_manager_get_source_model(self.as_ptr()) };
        unsafe { NSManagedObjectModel::from_retained_ptr(ptr, "migration manager source model") }
    }

    /// Wraps `NSMigrationManager.destination_model(...)`.
    pub fn destination_model(&self) -> Result<NSManagedObjectModel, CoreDataError> {
        let ptr = unsafe { ffi::cd_migration_manager_get_destination_model(self.as_ptr()) };
        unsafe {
            NSManagedObjectModel::from_retained_ptr(ptr, "migration manager destination model")
        }
    }

    /// Wraps `NSMigrationManager.source_context(...)`.
    pub fn source_context(&self) -> Result<NSManagedObjectContext, CoreDataError> {
        let ptr = unsafe { ffi::cd_migration_manager_get_source_context(self.as_ptr()) };
        unsafe {
            NSManagedObjectContext::from_retained_ptr(ptr, "migration manager source context")
        }
    }

    /// Wraps `NSMigrationManager.destination_context(...)`.
    pub fn destination_context(&self) -> Result<NSManagedObjectContext, CoreDataError> {
        let ptr = unsafe { ffi::cd_migration_manager_get_destination_context(self.as_ptr()) };
        unsafe {
            NSManagedObjectContext::from_retained_ptr(ptr, "migration manager destination context")
        }
    }

    /// Wraps `NSMigrationManager.uses_store_specific_migration_manager(...)`.
    pub fn uses_store_specific_migration_manager(&self) -> bool {
        unsafe {
            ffi::cd_migration_manager_get_uses_store_specific_migration_manager(self.as_ptr()) != 0
        }
    }

    /// Mirrors `NSMigrationManager.uses_store_specific_migration_manager`.
    pub fn set_uses_store_specific_migration_manager(
        &self,
        uses_store_specific_migration_manager: bool,
    ) {
        unsafe {
            ffi::cd_migration_manager_set_uses_store_specific_migration_manager(
                self.as_ptr(),
                i32::from(uses_store_specific_migration_manager),
            );
        }
    }

    /// Wraps `NSMigrationManager.migration_progress(...)`.
    pub fn migration_progress(&self) -> f32 {
        unsafe { ffi::cd_migration_manager_get_migration_progress(self.as_ptr()) }
    }

    #[allow(clippy::too_many_arguments)]
    /// Wraps `NSMigrationManager.migrate_store(...)`.
    pub fn migrate_store<S, D>(
        &self,
        source_url: S,
        source_store_type: &str,
        source_options: Option<&PersistentStoreOptions>,
        mapping_model: Option<&NSMappingModel>,
        destination_url: D,
        destination_store_type: &str,
        destination_options: Option<&PersistentStoreOptions>,
    ) -> Result<(), CoreDataError>
    where
        S: AsRef<Path>,
        D: AsRef<Path>,
    {
        let source_url = path_cstring(source_url.as_ref(), "migration source URL")?;
        let source_store_type = cstring_from_str(source_store_type, "migration source store type")?;
        let source_options = encode_options_json(source_options, "migration source options")?;
        let destination_url = path_cstring(destination_url.as_ref(), "migration destination URL")?;
        let destination_store_type =
            cstring_from_str(destination_store_type, "migration destination store type")?;
        let destination_options =
            encode_options_json(destination_options, "migration destination options")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_migration_manager_migrate_store(
                self.as_ptr(),
                source_url.as_ptr(),
                source_store_type.as_ptr(),
                opt_cstring_ptr(&source_options),
                mapping_model.map_or(core::ptr::null_mut(), NSMappingModel::as_ptr),
                destination_url.as_ptr(),
                destination_store_type.as_ptr(),
                opt_cstring_ptr(&destination_options),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }
}
