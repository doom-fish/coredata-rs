use core::ffi::{c_char, c_void};

extern "C" {
    pub fn cd_attribute_description_get_validation_rules_json(
        attribute: *mut c_void,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_attribute_description_set_validation_rules_json(
        attribute: *mut c_void,
        rules_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_relationship_description_get_validation_rules_json(
        relationship: *mut c_void,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_relationship_description_set_validation_rules_json(
        relationship: *mut c_void,
        rules_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_managed_object_validate_value_json(
        object: *mut c_void,
        key: *const c_char,
        value_json: *const c_char,
        out_validated_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_managed_object_validate_for_insert(
        object: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_managed_object_validate_for_update(
        object: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_managed_object_validate_for_delete(
        object: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
}
