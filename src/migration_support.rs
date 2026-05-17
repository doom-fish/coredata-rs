#![allow(dead_code)]

use core::ffi::c_void;

use crate::error::CoreDataError;
use crate::ffi;
use crate::model::NSManagedObjectModel;
use crate::private::{
    collect_array, cstring_from_str, error_from_status, impl_object_wrapper, json_cstring,
    parse_json_ptr, take_string,
};
use crate::store::NSPersistentContainer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum NSEntityMappingType {
    Undefined,
    Custom,
    Add,
    Remove,
    Copy,
    Transform,
    Unknown(u64),
}

impl NSEntityMappingType {
    const fn from_raw(raw: u64) -> Self {
        match raw {
            0 => Self::Undefined,
            1 => Self::Custom,
            2 => Self::Add,
            3 => Self::Remove,
            4 => Self::Copy,
            5 => Self::Transform,
            other => Self::Unknown(other),
        }
    }

    const fn as_raw(self) -> u64 {
        match self {
            Self::Undefined => 0,
            Self::Custom => 1,
            Self::Add => 2,
            Self::Remove => 3,
            Self::Copy => 4,
            Self::Transform => 5,
            Self::Unknown(raw) => raw,
        }
    }
}

impl_object_wrapper!(NSEntityMapping);
impl_object_wrapper!(NSPropertyMapping);
impl_object_wrapper!(NSEntityMigrationPolicy);
impl_object_wrapper!(NSManagedObjectModelReference);
impl_object_wrapper!(NSMigrationStage);
impl_object_wrapper!(NSLightweightMigrationStage);
impl_object_wrapper!(NSCustomMigrationStage);
impl_object_wrapper!(NSStagedMigrationManager);

fn clone_retained_wrapper<T>(ptr: *mut c_void, context: &str) -> Result<T, CoreDataError>
where
    T: crate::private::FromRetainedPtr,
{
    let retained = unsafe { ffi::cd_retain_object(ptr) };
    unsafe { T::from_retained_ptr(retained, context) }
}

impl NSEntityMapping {
    pub fn new() -> Result<Self, CoreDataError> {
        let mut out_mapping = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe { ffi::cd_entity_mapping_new(&mut out_mapping, &mut out_error) };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_mapping, "entity mapping") }
    }

    pub fn name(&self) -> Result<String, CoreDataError> {
        let ptr = unsafe { ffi::cd_entity_mapping_get_name(self.as_ptr()) };
        unsafe { take_string(ptr) }
            .ok_or_else(|| CoreDataError::bridge(-1, "entity mapping name was nil"))
    }

    pub fn set_name(&self, name: &str) -> Result<(), CoreDataError> {
        let name = cstring_from_str(name, "entity mapping name")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_entity_mapping_set_name(self.as_ptr(), name.as_ptr(), &mut out_error)
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn mapping_type(&self) -> NSEntityMappingType {
        NSEntityMappingType::from_raw(unsafe {
            ffi::cd_entity_mapping_get_mapping_type(self.as_ptr())
        })
    }

    pub fn set_mapping_type(&self, mapping_type: NSEntityMappingType) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_entity_mapping_set_mapping_type(
                self.as_ptr(),
                mapping_type.as_raw(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn source_entity_name(&self) -> Option<String> {
        let ptr = unsafe { ffi::cd_entity_mapping_get_source_entity_name(self.as_ptr()) };
        unsafe { take_string(ptr) }
    }

    pub fn set_source_entity_name(&self, name: Option<&str>) -> Result<(), CoreDataError> {
        let name = name
            .map(|value| cstring_from_str(value, "entity mapping source entity name"))
            .transpose()?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_entity_mapping_set_source_entity_name(
                self.as_ptr(),
                name.as_ref()
                    .map_or(core::ptr::null(), |value| value.as_ptr()),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn destination_entity_name(&self) -> Option<String> {
        let ptr = unsafe { ffi::cd_entity_mapping_get_destination_entity_name(self.as_ptr()) };
        unsafe { take_string(ptr) }
    }

    pub fn set_destination_entity_name(&self, name: Option<&str>) -> Result<(), CoreDataError> {
        let name = name
            .map(|value| cstring_from_str(value, "entity mapping destination entity name"))
            .transpose()?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_entity_mapping_set_destination_entity_name(
                self.as_ptr(),
                name.as_ref()
                    .map_or(core::ptr::null(), |value| value.as_ptr()),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }
}

impl NSPropertyMapping {
    pub fn new() -> Result<Self, CoreDataError> {
        let mut out_mapping = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe { ffi::cd_property_mapping_new(&mut out_mapping, &mut out_error) };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_mapping, "property mapping") }
    }

    pub fn name(&self) -> Option<String> {
        let ptr = unsafe { ffi::cd_property_mapping_get_name(self.as_ptr()) };
        unsafe { take_string(ptr) }
    }

    pub fn set_name(&self, name: Option<&str>) -> Result<(), CoreDataError> {
        let name = name
            .map(|value| cstring_from_str(value, "property mapping name"))
            .transpose()?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_property_mapping_set_name(
                self.as_ptr(),
                name.as_ref()
                    .map_or(core::ptr::null(), |value| value.as_ptr()),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }
}

impl NSManagedObjectModelReference {
    pub fn new_with_model(
        model: &NSManagedObjectModel,
        version_checksum: &str,
    ) -> Result<Self, CoreDataError> {
        let version_checksum =
            cstring_from_str(version_checksum, "model reference version checksum")?;
        let mut out_reference = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_model_reference_new_with_model(
                model.as_ptr(),
                version_checksum.as_ptr(),
                &mut out_reference,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_reference, "managed object model reference") }
    }

    pub fn resolved_model(&self) -> Result<NSManagedObjectModel, CoreDataError> {
        let ptr =
            unsafe { ffi::cd_managed_object_model_reference_get_resolved_model(self.as_ptr()) };
        unsafe {
            NSManagedObjectModel::from_retained_ptr(
                ptr,
                "managed object model reference resolved model",
            )
        }
    }

    pub fn version_checksum(&self) -> Result<String, CoreDataError> {
        let ptr =
            unsafe { ffi::cd_managed_object_model_reference_get_version_checksum(self.as_ptr()) };
        unsafe { take_string(ptr) }.ok_or_else(|| {
            CoreDataError::bridge(
                -1,
                "managed object model reference version checksum was nil",
            )
        })
    }
}

impl NSMigrationStage {
    pub fn label(&self) -> Option<String> {
        let ptr = unsafe { ffi::cd_migration_stage_get_label(self.as_ptr()) };
        unsafe { take_string(ptr) }
    }

    pub fn set_label(&self, label: Option<&str>) -> Result<(), CoreDataError> {
        let label = label
            .map(|value| cstring_from_str(value, "migration stage label"))
            .transpose()?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_migration_stage_set_label(
                self.as_ptr(),
                label
                    .as_ref()
                    .map_or(core::ptr::null(), |value| value.as_ptr()),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }
}

impl NSLightweightMigrationStage {
    pub fn new(version_checksums: &[&str]) -> Result<Self, CoreDataError> {
        let payload = version_checksums
            .iter()
            .map(|checksum| (*checksum).to_string())
            .collect::<Vec<_>>();
        let version_checksums_json =
            json_cstring(&payload, "lightweight migration stage version checksums")?;
        let mut out_stage = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_lightweight_migration_stage_new(
                version_checksums_json.as_ptr(),
                &mut out_stage,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_stage, "lightweight migration stage") }
    }

    pub fn version_checksums(&self) -> Result<Vec<String>, CoreDataError> {
        let mut out_json = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_lightweight_migration_stage_get_version_checksums_json(
                self.as_ptr(),
                &mut out_json,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { parse_json_ptr(out_json, "lightweight migration stage version checksums") }
    }

    pub fn as_migration_stage(&self) -> Result<NSMigrationStage, CoreDataError> {
        clone_retained_wrapper(self.as_ptr(), "migration stage")
    }
}

impl NSCustomMigrationStage {
    pub fn new(
        current_model: &NSManagedObjectModelReference,
        next_model: &NSManagedObjectModelReference,
    ) -> Result<Self, CoreDataError> {
        let mut out_stage = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_custom_migration_stage_new(
                current_model.as_ptr(),
                next_model.as_ptr(),
                &mut out_stage,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_stage, "custom migration stage") }
    }

    pub fn current_model(&self) -> Result<NSManagedObjectModelReference, CoreDataError> {
        let ptr = unsafe { ffi::cd_custom_migration_stage_get_current_model(self.as_ptr()) };
        unsafe {
            NSManagedObjectModelReference::from_retained_ptr(
                ptr,
                "custom migration stage current model",
            )
        }
    }

    pub fn next_model(&self) -> Result<NSManagedObjectModelReference, CoreDataError> {
        let ptr = unsafe { ffi::cd_custom_migration_stage_get_next_model(self.as_ptr()) };
        unsafe {
            NSManagedObjectModelReference::from_retained_ptr(
                ptr,
                "custom migration stage next model",
            )
        }
    }

    pub fn as_migration_stage(&self) -> Result<NSMigrationStage, CoreDataError> {
        clone_retained_wrapper(self.as_ptr(), "migration stage")
    }
}

impl NSStagedMigrationManager {
    pub fn new(stages: &[&NSMigrationStage]) -> Result<Self, CoreDataError> {
        let raw_stages = stages
            .iter()
            .map(|stage| stage.as_ptr())
            .collect::<Vec<_>>();
        let count = i32::try_from(raw_stages.len()).map_err(|_| {
            CoreDataError::bridge(-1, "staged migration manager stage count overflow")
        })?;
        let mut out_manager = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_staged_migration_manager_new(
                raw_stages.as_ptr(),
                count,
                &mut out_manager,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_manager, "staged migration manager") }
    }

    pub fn stages(&self) -> Result<Vec<NSMigrationStage>, CoreDataError> {
        let array_ptr = unsafe { ffi::cd_staged_migration_manager_get_stages(self.as_ptr()) };
        collect_array(array_ptr, "staged migration manager stages")
    }

    pub fn container(&self) -> Result<Option<NSPersistentContainer>, CoreDataError> {
        let ptr = unsafe { ffi::cd_staged_migration_manager_get_container(self.as_ptr()) };
        if ptr.is_null() {
            return Ok(None);
        }
        Ok(Some(unsafe {
            NSPersistentContainer::from_retained_ptr(ptr, "staged migration manager container")?
        }))
    }
}
