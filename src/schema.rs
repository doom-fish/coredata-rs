use crate::error::CoreDataError;
use crate::ffi;
use crate::private::{
    collect_array, cstring_from_str, error_from_status, impl_object_wrapper, take_string,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
/// Mirrors the corresponding Core Data `AttributeType` value.
pub enum AttributeType {
    /// Mirrors `AttributeType::Undefined`.
    Undefined,
    /// Mirrors `AttributeType::Integer16`.
    Integer16,
    /// Mirrors `AttributeType::Integer32`.
    Integer32,
    /// Mirrors `AttributeType::Integer64`.
    Integer64,
    /// Mirrors `AttributeType::Decimal`.
    Decimal,
    /// Mirrors `AttributeType::Double`.
    Double,
    /// Mirrors `AttributeType::Float`.
    Float,
    /// Mirrors `AttributeType::String`.
    String,
    /// Mirrors `AttributeType::Boolean`.
    Boolean,
    /// Mirrors `AttributeType::Date`.
    Date,
    /// Mirrors `AttributeType::BinaryData`.
    BinaryData,
    /// Mirrors `AttributeType::Uuid`.
    Uuid,
    /// Mirrors `AttributeType::Uri`.
    Uri,
    /// Mirrors `AttributeType::Transformable`.
    Transformable,
    /// Mirrors `AttributeType::ObjectId`.
    ObjectId,
    /// Mirrors `AttributeType::Unknown`.
    Unknown(u64),
}

impl AttributeType {
    pub(crate) const fn from_raw(raw: u64) -> Self {
        match raw {
            0 => Self::Undefined,
            100 => Self::Integer16,
            200 => Self::Integer32,
            300 => Self::Integer64,
            400 => Self::Decimal,
            500 => Self::Double,
            600 => Self::Float,
            700 => Self::String,
            800 => Self::Boolean,
            900 => Self::Date,
            1000 => Self::BinaryData,
            1100 => Self::Uuid,
            1200 => Self::Uri,
            1800 => Self::Transformable,
            2000 => Self::ObjectId,
            other => Self::Unknown(other),
        }
    }

    pub(crate) const fn as_raw(self) -> u64 {
        match self {
            Self::Undefined => 0,
            Self::Integer16 => 100,
            Self::Integer32 => 200,
            Self::Integer64 => 300,
            Self::Decimal => 400,
            Self::Double => 500,
            Self::Float => 600,
            Self::String => 700,
            Self::Boolean => 800,
            Self::Date => 900,
            Self::BinaryData => 1000,
            Self::Uuid => 1100,
            Self::Uri => 1200,
            Self::Transformable => 1800,
            Self::ObjectId => 2000,
            Self::Unknown(raw) => raw,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
/// Mirrors the corresponding Core Data `DeleteRule` value.
pub enum DeleteRule {
    /// Mirrors `DeleteRule::NoAction`.
    NoAction,
    /// Mirrors `DeleteRule::Nullify`.
    Nullify,
    /// Mirrors `DeleteRule::Cascade`.
    Cascade,
    /// Mirrors `DeleteRule::Deny`.
    Deny,
    /// Mirrors `DeleteRule::Unknown`.
    Unknown(u64),
}

impl DeleteRule {
    pub(crate) const fn from_raw(raw: u64) -> Self {
        match raw {
            0 => Self::NoAction,
            1 => Self::Nullify,
            2 => Self::Cascade,
            3 => Self::Deny,
            other => Self::Unknown(other),
        }
    }

    pub(crate) const fn as_raw(self) -> u64 {
        match self {
            Self::NoAction => 0,
            Self::Nullify => 1,
            Self::Cascade => 2,
            Self::Deny => 3,
            Self::Unknown(raw) => raw,
        }
    }
}

impl_object_wrapper!(NSEntityDescription);
impl_object_wrapper!(NSAttributeDescription);
impl_object_wrapper!(NSRelationshipDescription);

impl NSEntityDescription {
    /// Wraps `NSEntityDescription.init(...)`.
    pub fn new() -> Result<Self, CoreDataError> {
        let mut out_entity = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe { ffi::cd_entity_description_new(&mut out_entity, &mut out_error) };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_entity, "entity description") }
    }

    /// Wraps `NSEntityDescription.named(...)`.
    pub fn named(name: impl AsRef<str>) -> Result<Self, CoreDataError> {
        let entity = Self::new()?;
        entity.set_name(name)?;
        Ok(entity)
    }

    /// Wraps `NSEntityDescription.name(...)`.
    pub fn name(&self) -> Result<String, CoreDataError> {
        let ptr = unsafe { ffi::cd_entity_description_get_name(self.as_ptr()) };
        unsafe { take_string(ptr) }
            .ok_or_else(|| CoreDataError::bridge(-1, "entity description name was nil"))
    }

    /// Mirrors `NSEntityDescription.name`.
    pub fn set_name(&self, name: impl AsRef<str>) -> Result<(), CoreDataError> {
        let name = cstring_from_str(name.as_ref(), "entity name")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_entity_description_set_name(self.as_ptr(), name.as_ptr(), &mut out_error)
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSEntityDescription.managed_object_class_name(...)`.
    pub fn managed_object_class_name(&self) -> Result<String, CoreDataError> {
        let ptr =
            unsafe { ffi::cd_entity_description_get_managed_object_class_name(self.as_ptr()) };
        unsafe { take_string(ptr) }
            .ok_or_else(|| CoreDataError::bridge(-1, "managed object class name was nil"))
    }

    /// Mirrors `NSEntityDescription.managed_object_class_name`.
    pub fn set_managed_object_class_name(
        &self,
        name: impl AsRef<str>,
    ) -> Result<(), CoreDataError> {
        let name = cstring_from_str(name.as_ref(), "managed object class name")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_entity_description_set_managed_object_class_name(
                self.as_ptr(),
                name.as_ptr(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSEntityDescription.add_attribute(...)`.
    pub fn add_attribute(&self, attribute: &NSAttributeDescription) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_entity_description_add_attribute(
                self.as_ptr(),
                attribute.as_ptr(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSEntityDescription.add_relationship(...)`.
    pub fn add_relationship(
        &self,
        relationship: &NSRelationshipDescription,
    ) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_entity_description_add_relationship(
                self.as_ptr(),
                relationship.as_ptr(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSEntityDescription.attributes(...)`.
    pub fn attributes(&self) -> Result<Vec<NSAttributeDescription>, CoreDataError> {
        let array_ptr = unsafe { ffi::cd_entity_description_attributes(self.as_ptr()) };
        collect_array(array_ptr, "entity attributes")
    }

    /// Wraps `NSEntityDescription.relationships(...)`.
    pub fn relationships(&self) -> Result<Vec<NSRelationshipDescription>, CoreDataError> {
        let array_ptr = unsafe { ffi::cd_entity_description_relationships(self.as_ptr()) };
        collect_array(array_ptr, "entity relationships")
    }
}

impl NSAttributeDescription {
    /// Wraps `NSAttributeDescription.init(...)`.
    pub fn new(
        name: impl AsRef<str>,
        attribute_type: AttributeType,
    ) -> Result<Self, CoreDataError> {
        let mut out_attribute = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status =
            unsafe { ffi::cd_attribute_description_new(&mut out_attribute, &mut out_error) };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        let attribute = unsafe { Self::from_retained_ptr(out_attribute, "attribute description") }?;
        attribute.set_name(name)?;
        attribute.set_attribute_type(attribute_type)?;
        Ok(attribute)
    }

    /// Wraps `NSAttributeDescription.name(...)`.
    pub fn name(&self) -> Result<String, CoreDataError> {
        let ptr = unsafe { ffi::cd_attribute_description_get_name(self.as_ptr()) };
        unsafe { take_string(ptr) }
            .ok_or_else(|| CoreDataError::bridge(-1, "attribute description name was nil"))
    }

    /// Mirrors `NSAttributeDescription.name`.
    pub fn set_name(&self, name: impl AsRef<str>) -> Result<(), CoreDataError> {
        let name = cstring_from_str(name.as_ref(), "attribute name")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_attribute_description_set_name(self.as_ptr(), name.as_ptr(), &mut out_error)
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSAttributeDescription.is_optional(...)`.
    pub fn is_optional(&self) -> bool {
        unsafe { ffi::cd_attribute_description_get_optional(self.as_ptr()) != 0 }
    }

    /// Mirrors `NSAttributeDescription.optional`.
    pub fn set_optional(&self, optional: bool) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_attribute_description_set_optional(
                self.as_ptr(),
                i32::from(optional),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSAttributeDescription.is_transient(...)`.
    pub fn is_transient(&self) -> bool {
        unsafe { ffi::cd_attribute_description_get_transient(self.as_ptr()) != 0 }
    }

    /// Mirrors `NSAttributeDescription.transient`.
    pub fn set_transient(&self, transient: bool) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_attribute_description_set_transient(
                self.as_ptr(),
                i32::from(transient),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSAttributeDescription.attribute_type(...)`.
    pub fn attribute_type(&self) -> AttributeType {
        AttributeType::from_raw(unsafe {
            ffi::cd_attribute_description_get_attribute_type(self.as_ptr())
        })
    }

    /// Mirrors `NSAttributeDescription.attribute_type`.
    pub fn set_attribute_type(&self, attribute_type: AttributeType) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_attribute_description_set_attribute_type(
                self.as_ptr(),
                attribute_type.as_raw(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }
}

impl NSRelationshipDescription {
    /// Wraps `NSRelationshipDescription.init(...)`.
    pub fn new(name: impl AsRef<str>) -> Result<Self, CoreDataError> {
        let mut out_relationship = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status =
            unsafe { ffi::cd_relationship_description_new(&mut out_relationship, &mut out_error) };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        let relationship =
            unsafe { Self::from_retained_ptr(out_relationship, "relationship description") }?;
        relationship.set_name(name)?;
        Ok(relationship)
    }

    /// Wraps `NSRelationshipDescription.name(...)`.
    pub fn name(&self) -> Result<String, CoreDataError> {
        let ptr = unsafe { ffi::cd_relationship_description_get_name(self.as_ptr()) };
        unsafe { take_string(ptr) }
            .ok_or_else(|| CoreDataError::bridge(-1, "relationship description name was nil"))
    }

    /// Mirrors `NSRelationshipDescription.name`.
    pub fn set_name(&self, name: impl AsRef<str>) -> Result<(), CoreDataError> {
        let name = cstring_from_str(name.as_ref(), "relationship name")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_relationship_description_set_name(self.as_ptr(), name.as_ptr(), &mut out_error)
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSRelationshipDescription.is_optional(...)`.
    pub fn is_optional(&self) -> bool {
        unsafe { ffi::cd_relationship_description_get_optional(self.as_ptr()) != 0 }
    }

    /// Mirrors `NSRelationshipDescription.optional`.
    pub fn set_optional(&self, optional: bool) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_relationship_description_set_optional(
                self.as_ptr(),
                i32::from(optional),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSRelationshipDescription.is_transient(...)`.
    pub fn is_transient(&self) -> bool {
        unsafe { ffi::cd_relationship_description_get_transient(self.as_ptr()) != 0 }
    }

    /// Mirrors `NSRelationshipDescription.transient`.
    pub fn set_transient(&self, transient: bool) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_relationship_description_set_transient(
                self.as_ptr(),
                i32::from(transient),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSRelationshipDescription.destination_entity(...)`.
    pub fn destination_entity(&self) -> Result<Option<NSEntityDescription>, CoreDataError> {
        let ptr = unsafe { ffi::cd_relationship_description_get_destination_entity(self.as_ptr()) };
        if ptr.is_null() {
            return Ok(None);
        }
        Ok(Some(unsafe {
            NSEntityDescription::from_retained_ptr(ptr, "relationship destination entity")?
        }))
    }

    /// Mirrors `NSRelationshipDescription.destination_entity`.
    pub fn set_destination_entity(
        &self,
        entity: Option<&NSEntityDescription>,
    ) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_relationship_description_set_destination_entity(
                self.as_ptr(),
                entity.map_or(core::ptr::null_mut(), NSEntityDescription::as_ptr),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSRelationshipDescription.inverse_relationship(...)`.
    pub fn inverse_relationship(&self) -> Result<Option<NSRelationshipDescription>, CoreDataError> {
        let ptr =
            unsafe { ffi::cd_relationship_description_get_inverse_relationship(self.as_ptr()) };
        if ptr.is_null() {
            return Ok(None);
        }
        Ok(Some(unsafe {
            NSRelationshipDescription::from_retained_ptr(ptr, "relationship inverse relationship")?
        }))
    }

    /// Mirrors `NSRelationshipDescription.inverse_relationship`.
    pub fn set_inverse_relationship(
        &self,
        inverse_relationship: Option<&NSRelationshipDescription>,
    ) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_relationship_description_set_inverse_relationship(
                self.as_ptr(),
                inverse_relationship
                    .map_or(core::ptr::null_mut(), NSRelationshipDescription::as_ptr),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSRelationshipDescription.min_count(...)`.
    pub fn min_count(&self) -> usize {
        unsafe { ffi::cd_relationship_description_get_min_count(self.as_ptr()) as usize }
    }

    /// Mirrors `NSRelationshipDescription.min_count`.
    pub fn set_min_count(&self, min_count: usize) -> Result<(), CoreDataError> {
        let min_count = u64::try_from(min_count)
            .map_err(|_| CoreDataError::bridge(-1, "relationship min_count overflow"))?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_relationship_description_set_min_count(self.as_ptr(), min_count, &mut out_error)
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSRelationshipDescription.max_count(...)`.
    pub fn max_count(&self) -> usize {
        unsafe { ffi::cd_relationship_description_get_max_count(self.as_ptr()) as usize }
    }

    /// Mirrors `NSRelationshipDescription.max_count`.
    pub fn set_max_count(&self, max_count: usize) -> Result<(), CoreDataError> {
        let max_count = u64::try_from(max_count)
            .map_err(|_| CoreDataError::bridge(-1, "relationship max_count overflow"))?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_relationship_description_set_max_count(self.as_ptr(), max_count, &mut out_error)
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSRelationshipDescription.delete_rule(...)`.
    pub fn delete_rule(&self) -> DeleteRule {
        DeleteRule::from_raw(unsafe {
            ffi::cd_relationship_description_get_delete_rule(self.as_ptr())
        })
    }

    /// Mirrors `NSRelationshipDescription.delete_rule`.
    pub fn set_delete_rule(&self, delete_rule: DeleteRule) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_relationship_description_set_delete_rule(
                self.as_ptr(),
                delete_rule.as_raw(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attribute_type_round_trips_known_values() {
        let cases = [
            (0, AttributeType::Undefined),
            (100, AttributeType::Integer16),
            (200, AttributeType::Integer32),
            (700, AttributeType::String),
            (1_800, AttributeType::Transformable),
            (2_000, AttributeType::ObjectId),
        ];

        for (raw, attribute_type) in cases {
            assert_eq!(AttributeType::from_raw(raw), attribute_type);
            assert_eq!(attribute_type.as_raw(), raw);
        }
    }

    #[test]
    fn attribute_type_unknown_round_trips() {
        let attribute_type = AttributeType::from_raw(777);

        assert_eq!(attribute_type, AttributeType::Unknown(777));
        assert_eq!(attribute_type.as_raw(), 777);
    }

    #[test]
    fn delete_rule_round_trips_known_values() {
        let cases = [
            (0, DeleteRule::NoAction),
            (1, DeleteRule::Nullify),
            (2, DeleteRule::Cascade),
            (3, DeleteRule::Deny),
        ];

        for (raw, delete_rule) in cases {
            assert_eq!(DeleteRule::from_raw(raw), delete_rule);
            assert_eq!(delete_rule.as_raw(), raw);
        }
    }

    #[test]
    fn delete_rule_unknown_round_trips() {
        let delete_rule = DeleteRule::from_raw(9);

        assert_eq!(delete_rule, DeleteRule::Unknown(9));
        assert_eq!(delete_rule.as_raw(), 9);
    }
}
