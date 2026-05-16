use crate::context::NSManagedObject;
use crate::error::CoreDataError;
use crate::ffi;
use crate::private::{error_from_status, json_cstring, parse_json_ptr};
use crate::schema::{NSAttributeDescription, NSRelationshipDescription};
use crate::value::{Value, ValuePayload};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationRule {
    pub predicate_format: String,
    pub warning: String,
}

impl ValidationRule {
    pub fn new(predicate_format: impl Into<String>, warning: impl Into<String>) -> Self {
        Self {
            predicate_format: predicate_format.into(),
            warning: warning.into(),
        }
    }
}

pub mod validation_error_codes {
    pub const MANAGED_OBJECT_VALIDATION: i64 = 1550;
    pub const MANAGED_OBJECT_CONSTRAINT_VALIDATION: i64 = 1551;
    pub const MULTIPLE_ERRORS: i64 = 1560;
    pub const MISSING_MANDATORY_PROPERTY: i64 = 1570;
    pub const RELATIONSHIP_LACKS_MINIMUM_COUNT: i64 = 1580;
    pub const RELATIONSHIP_EXCEEDS_MAXIMUM_COUNT: i64 = 1590;
    pub const RELATIONSHIP_DENIED_DELETE: i64 = 1600;
    pub const NUMBER_TOO_LARGE: i64 = 1610;
    pub const NUMBER_TOO_SMALL: i64 = 1620;
    pub const DATE_TOO_LATE: i64 = 1630;
    pub const DATE_TOO_SOON: i64 = 1640;
    pub const INVALID_DATE: i64 = 1650;
    pub const STRING_TOO_LONG: i64 = 1660;
    pub const STRING_TOO_SHORT: i64 = 1670;
    pub const STRING_PATTERN_MATCHING: i64 = 1680;
    pub const INVALID_URI: i64 = 1690;
}

impl NSAttributeDescription {
    pub fn validation_rules(&self) -> Result<Vec<ValidationRule>, CoreDataError> {
        let mut out_json = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_attribute_description_get_validation_rules_json(
                self.as_ptr(),
                &mut out_json,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { parse_json_ptr(out_json, "attribute validation rules") }
    }

    pub fn set_validation_rules(&self, rules: &[ValidationRule]) -> Result<(), CoreDataError> {
        let rules_json = json_cstring(rules, "attribute validation rules")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_attribute_description_set_validation_rules_json(
                self.as_ptr(),
                rules_json.as_ptr(),
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
    pub fn validation_rules(&self) -> Result<Vec<ValidationRule>, CoreDataError> {
        let mut out_json = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_relationship_description_get_validation_rules_json(
                self.as_ptr(),
                &mut out_json,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { parse_json_ptr(out_json, "relationship validation rules") }
    }

    pub fn set_validation_rules(&self, rules: &[ValidationRule]) -> Result<(), CoreDataError> {
        let rules_json = json_cstring(rules, "relationship validation rules")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_relationship_description_set_validation_rules_json(
                self.as_ptr(),
                rules_json.as_ptr(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }
}

impl NSManagedObject {
    pub fn validate_value(
        &self,
        key: &str,
        value: impl Into<Value>,
    ) -> Result<Value, CoreDataError> {
        let key = crate::private::cstring_from_str(key, "validation key")?;
        let payload = ValuePayload::from(value.into());
        let value_json = json_cstring(&payload, "validation value")?;
        let mut out_validated_json = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_validate_value_json(
                self.as_ptr(),
                key.as_ptr(),
                value_json.as_ptr(),
                &mut out_validated_json,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        let payload: ValuePayload =
            unsafe { parse_json_ptr(out_validated_json, "validated value")? };
        payload.try_into()
    }

    pub fn validate_for_insert(&self) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status =
            unsafe { ffi::cd_managed_object_validate_for_insert(self.as_ptr(), &mut out_error) };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn validate_for_update(&self) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status =
            unsafe { ffi::cd_managed_object_validate_for_update(self.as_ptr(), &mut out_error) };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn validate_for_delete(&self) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status =
            unsafe { ffi::cd_managed_object_validate_for_delete(self.as_ptr(), &mut out_error) };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }
}
