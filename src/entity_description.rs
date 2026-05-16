use std::collections::BTreeMap;

use crate::context::NSManagedObject;
use crate::context::NSManagedObjectContext;
use crate::error::CoreDataError;
use crate::ffi;
use crate::model::NSManagedObjectModel;
use crate::private::{
    collect_array, cstring_from_str, error_from_status, json_cstring, parse_json_ptr, take_string,
};
use crate::schema::{NSAttributeDescription, NSEntityDescription, NSRelationshipDescription};
use crate::value::{Value, ValuePayload};

impl NSEntityDescription {
    pub fn entity_for_name(
        name: &str,
        context: &NSManagedObjectContext,
    ) -> Result<Option<Self>, CoreDataError> {
        let name = cstring_from_str(name, "entity description lookup name")?;
        let mut out_entity = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_entity_description_entity_for_name(
                name.as_ptr(),
                context.as_ptr(),
                &mut out_entity,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        if out_entity.is_null() {
            return Ok(None);
        }
        Ok(Some(unsafe {
            Self::from_retained_ptr(out_entity, "entity description lookup")?
        }))
    }

    pub fn insert_new_object_for_name(
        name: &str,
        context: &NSManagedObjectContext,
    ) -> Result<NSManagedObject, CoreDataError> {
        let name = cstring_from_str(name, "entity insertion name")?;
        let mut out_object = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_entity_description_insert_new_object_for_name(
                name.as_ptr(),
                context.as_ptr(),
                &mut out_object,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { NSManagedObject::from_retained_ptr(out_object, "inserted managed object") }
    }

    pub fn managed_object_model(&self) -> Result<NSManagedObjectModel, CoreDataError> {
        let ptr = unsafe { ffi::cd_entity_description_get_managed_object_model(self.as_ptr()) };
        unsafe { NSManagedObjectModel::from_retained_ptr(ptr, "entity description model") }
    }

    pub fn is_abstract(&self) -> bool {
        unsafe { ffi::cd_entity_description_get_abstract(self.as_ptr()) != 0 }
    }

    pub fn set_abstract(&self, abstract_flag: bool) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_entity_description_set_abstract(
                self.as_ptr(),
                i32::from(abstract_flag),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn user_info(&self) -> Result<BTreeMap<String, String>, CoreDataError> {
        let mut out_json = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_entity_description_get_user_info_json(
                self.as_ptr(),
                &mut out_json,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { parse_json_ptr(out_json, "entity description user info") }
    }

    pub fn set_user_info(&self, user_info: &BTreeMap<String, String>) -> Result<(), CoreDataError> {
        let user_info_json = json_cstring(user_info, "entity description user info")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_entity_description_set_user_info_json(
                self.as_ptr(),
                user_info_json.as_ptr(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn version_hash(&self) -> Result<String, CoreDataError> {
        let ptr = unsafe { ffi::cd_entity_description_get_version_hash(self.as_ptr()) };
        unsafe { take_string(ptr) }
            .ok_or_else(|| CoreDataError::bridge(-1, "entity description version hash was nil"))
    }

    pub fn version_hash_modifier(&self) -> Option<String> {
        let ptr = unsafe { ffi::cd_entity_description_get_version_hash_modifier(self.as_ptr()) };
        unsafe { take_string(ptr) }
    }

    pub fn set_version_hash_modifier(&self, modifier: Option<&str>) -> Result<(), CoreDataError> {
        let modifier = modifier
            .map(|value| cstring_from_str(value, "entity version hash modifier"))
            .transpose()?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_entity_description_set_version_hash_modifier(
                self.as_ptr(),
                modifier
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

    pub fn renaming_identifier(&self) -> Option<String> {
        let ptr = unsafe { ffi::cd_entity_description_get_renaming_identifier(self.as_ptr()) };
        unsafe { take_string(ptr) }
    }

    pub fn set_renaming_identifier(&self, identifier: Option<&str>) -> Result<(), CoreDataError> {
        let identifier = identifier
            .map(|value| cstring_from_str(value, "entity renaming identifier"))
            .transpose()?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_entity_description_set_renaming_identifier(
                self.as_ptr(),
                identifier
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

    pub fn uniqueness_constraints(&self) -> Result<Vec<Vec<String>>, CoreDataError> {
        let mut out_json = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_entity_description_get_uniqueness_constraints_json(
                self.as_ptr(),
                &mut out_json,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { parse_json_ptr(out_json, "entity uniqueness constraints") }
    }

    pub fn set_uniqueness_constraints(
        &self,
        constraints: &[Vec<String>],
    ) -> Result<(), CoreDataError> {
        let constraints_json = json_cstring(constraints, "entity uniqueness constraints")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_entity_description_set_uniqueness_constraints_json(
                self.as_ptr(),
                constraints_json.as_ptr(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn relationships_with_destination_entity(
        &self,
        destination_entity: &NSEntityDescription,
    ) -> Result<Vec<NSRelationshipDescription>, CoreDataError> {
        let array_ptr = unsafe {
            ffi::cd_entity_description_relationships_with_destination_entity(
                self.as_ptr(),
                destination_entity.as_ptr(),
            )
        };
        collect_array(array_ptr, "entity relationships with destination entity")
    }

    pub fn is_kind_of_entity(&self, other_entity: &NSEntityDescription) -> bool {
        unsafe {
            ffi::cd_entity_description_is_kind_of_entity(self.as_ptr(), other_entity.as_ptr()) != 0
        }
    }
}

impl NSAttributeDescription {
    pub fn attribute_value_class_name(&self) -> Option<String> {
        let ptr =
            unsafe { ffi::cd_attribute_description_get_attribute_value_class_name(self.as_ptr()) };
        unsafe { take_string(ptr) }
    }

    pub fn set_attribute_value_class_name(
        &self,
        class_name: Option<&str>,
    ) -> Result<(), CoreDataError> {
        let class_name = class_name
            .map(|value| cstring_from_str(value, "attribute value class name"))
            .transpose()?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_attribute_description_set_attribute_value_class_name(
                self.as_ptr(),
                class_name
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

    pub fn default_value(&self) -> Result<Value, CoreDataError> {
        let mut out_json = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_attribute_description_get_default_value_json(
                self.as_ptr(),
                &mut out_json,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        let payload: ValuePayload = unsafe { parse_json_ptr(out_json, "attribute default value")? };
        payload.try_into()
    }

    pub fn set_default_value(&self, value: impl Into<Value>) -> Result<(), CoreDataError> {
        let payload = ValuePayload::from(value.into());
        let value_json = json_cstring(&payload, "attribute default value")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_attribute_description_set_default_value_json(
                self.as_ptr(),
                value_json.as_ptr(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn value_transformer_name(&self) -> Option<String> {
        let ptr =
            unsafe { ffi::cd_attribute_description_get_value_transformer_name(self.as_ptr()) };
        unsafe { take_string(ptr) }
    }

    pub fn set_value_transformer_name(&self, name: Option<&str>) -> Result<(), CoreDataError> {
        let name = name
            .map(|value| cstring_from_str(value, "attribute value transformer name"))
            .transpose()?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_attribute_description_set_value_transformer_name(
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

    pub fn allows_external_binary_data_storage(&self) -> bool {
        unsafe {
            ffi::cd_attribute_description_get_allows_external_binary_data_storage(self.as_ptr())
                != 0
        }
    }

    pub fn set_allows_external_binary_data_storage(
        &self,
        allows_external_storage: bool,
    ) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_attribute_description_set_allows_external_binary_data_storage(
                self.as_ptr(),
                i32::from(allows_external_storage),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn preserves_value_in_history_on_deletion(&self) -> bool {
        unsafe {
            ffi::cd_attribute_description_get_preserves_value_in_history_on_deletion(self.as_ptr())
                != 0
        }
    }

    pub fn set_preserves_value_in_history_on_deletion(
        &self,
        preserves_value: bool,
    ) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_attribute_description_set_preserves_value_in_history_on_deletion(
                self.as_ptr(),
                i32::from(preserves_value),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn allows_cloud_encryption(&self) -> bool {
        unsafe { ffi::cd_attribute_description_get_allows_cloud_encryption(self.as_ptr()) != 0 }
    }

    pub fn set_allows_cloud_encryption(
        &self,
        allows_encryption: bool,
    ) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_attribute_description_set_allows_cloud_encryption(
                self.as_ptr(),
                i32::from(allows_encryption),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }
}
