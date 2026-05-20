use crate::context::NSManagedObject;
use crate::error::CoreDataError;
use crate::ffi;
use crate::private::{error_from_status, json_cstring, parse_json_ptr};
use crate::schema::{NSAttributeDescription, NSRelationshipDescription};
use crate::value::{Value, ValuePayload};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
/// Wraps `ValidationRule`.
pub struct ValidationRule {
    /// Mirrors `ValidationRule.predicate_format`.
    pub predicate_format: String,
    /// Mirrors `ValidationRule.warning`.
    pub warning: String,
}

impl ValidationRule {
    /// Wraps `ValidationRule.init(...)`.
    pub fn new(predicate_format: impl Into<String>, warning: impl Into<String>) -> Self {
        Self {
            predicate_format: predicate_format.into(),
            warning: warning.into(),
        }
    }
}

/// Core Data items for validation error codes.
pub mod validation_error_codes {
    /// Mirrors the corresponding Core Data constant.
    pub const MANAGED_OBJECT_VALIDATION: i64 = 1550;
    /// Mirrors the corresponding Core Data constant.
    pub const MANAGED_OBJECT_CONSTRAINT_VALIDATION: i64 = 1551;
    /// Mirrors the corresponding Core Data constant.
    pub const MULTIPLE_ERRORS: i64 = 1560;
    /// Mirrors the corresponding Core Data constant.
    pub const MISSING_MANDATORY_PROPERTY: i64 = 1570;
    /// Mirrors the corresponding Core Data constant.
    pub const RELATIONSHIP_LACKS_MINIMUM_COUNT: i64 = 1580;
    /// Mirrors the corresponding Core Data constant.
    pub const RELATIONSHIP_EXCEEDS_MAXIMUM_COUNT: i64 = 1590;
    /// Mirrors the corresponding Core Data constant.
    pub const RELATIONSHIP_DENIED_DELETE: i64 = 1600;
    /// Mirrors the corresponding Core Data constant.
    pub const NUMBER_TOO_LARGE: i64 = 1610;
    /// Mirrors the corresponding Core Data constant.
    pub const NUMBER_TOO_SMALL: i64 = 1620;
    /// Mirrors the corresponding Core Data constant.
    pub const DATE_TOO_LATE: i64 = 1630;
    /// Mirrors the corresponding Core Data constant.
    pub const DATE_TOO_SOON: i64 = 1640;
    /// Mirrors the corresponding Core Data constant.
    pub const INVALID_DATE: i64 = 1650;
    /// Mirrors the corresponding Core Data constant.
    pub const STRING_TOO_LONG: i64 = 1660;
    /// Mirrors the corresponding Core Data constant.
    pub const STRING_TOO_SHORT: i64 = 1670;
    /// Mirrors the corresponding Core Data constant.
    pub const STRING_PATTERN_MATCHING: i64 = 1680;
    /// Mirrors the corresponding Core Data constant.
    pub const INVALID_URI: i64 = 1690;
}

impl NSAttributeDescription {
    /// Wraps `NSAttributeDescription.validation_rules(...)`.
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

    /// Mirrors `NSAttributeDescription.validation_rules`.
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
    /// Wraps `NSRelationshipDescription.validation_rules(...)`.
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

    /// Mirrors `NSRelationshipDescription.validation_rules`.
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
    /// Wraps `NSManagedObject.validate_value(...)`.
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

    /// Wraps `NSManagedObject.validate_for_insert(...)`.
    pub fn validate_for_insert(&self) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status =
            unsafe { ffi::cd_managed_object_validate_for_insert(self.as_ptr(), &mut out_error) };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSManagedObject.validate_for_update(...)`.
    pub fn validate_for_update(&self) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status =
            unsafe { ffi::cd_managed_object_validate_for_update(self.as_ptr(), &mut out_error) };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSManagedObject.validate_for_delete(...)`.
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn validation_rule_new_sets_fields() {
        let rule = ValidationRule::new("name != nil", "Name is required");

        assert_eq!(rule.predicate_format, "name != nil");
        assert_eq!(rule.warning, "Name is required");
    }

    #[test]
    fn validation_rule_round_trips_through_serde() {
        let rule = ValidationRule::new("age > 0", "Age must be positive");
        let json = serde_json::to_string(&rule).expect("serialize validation rule");
        let decoded: ValidationRule =
            serde_json::from_str(&json).expect("deserialize validation rule");

        assert_eq!(decoded, rule);
    }

    #[test]
    fn validation_rule_serializes_camel_case_fields() {
        let value = serde_json::to_value(ValidationRule::new("score >= 10", "score warning"))
            .expect("serialize validation rule to value");

        assert_eq!(
            value,
            json!({
                "predicateFormat": "score >= 10",
                "warning": "score warning"
            })
        );
    }

    #[test]
    fn validation_error_codes_match_expected_values() {
        assert_eq!(validation_error_codes::MANAGED_OBJECT_VALIDATION, 1_550);
        assert_eq!(validation_error_codes::MISSING_MANDATORY_PROPERTY, 1_570);
        assert_eq!(validation_error_codes::STRING_PATTERN_MATCHING, 1_680);
        assert_eq!(validation_error_codes::INVALID_URI, 1_690);
    }
}
