use core::ffi::{c_char, c_void};

extern "C" {
    pub fn cd_entity_description_entity_for_name(
        name: *const c_char,
        context: *mut c_void,
        out_entity: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_entity_description_insert_new_object_for_name(
        name: *const c_char,
        context: *mut c_void,
        out_object: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_entity_description_get_managed_object_model(entity: *mut c_void) -> *mut c_void;
    pub fn cd_entity_description_get_abstract(entity: *mut c_void) -> i32;
    pub fn cd_entity_description_set_abstract(
        entity: *mut c_void,
        abstract_flag: i32,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_entity_description_get_user_info_json(
        entity: *mut c_void,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_entity_description_set_user_info_json(
        entity: *mut c_void,
        user_info_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_entity_description_get_version_hash(entity: *mut c_void) -> *mut c_char;
    pub fn cd_entity_description_get_version_hash_modifier(entity: *mut c_void) -> *mut c_char;
    pub fn cd_entity_description_set_version_hash_modifier(
        entity: *mut c_void,
        modifier: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_entity_description_get_renaming_identifier(entity: *mut c_void) -> *mut c_char;
    pub fn cd_entity_description_set_renaming_identifier(
        entity: *mut c_void,
        identifier: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_entity_description_get_uniqueness_constraints_json(
        entity: *mut c_void,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_entity_description_set_uniqueness_constraints_json(
        entity: *mut c_void,
        constraints_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_entity_description_relationships_with_destination_entity(
        entity: *mut c_void,
        destination_entity: *mut c_void,
    ) -> *mut c_void;
    pub fn cd_entity_description_is_kind_of_entity(
        entity: *mut c_void,
        other_entity: *mut c_void,
    ) -> i32;

    pub fn cd_attribute_description_get_attribute_value_class_name(
        attribute: *mut c_void,
    ) -> *mut c_char;
    pub fn cd_attribute_description_set_attribute_value_class_name(
        attribute: *mut c_void,
        class_name: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_attribute_description_get_default_value_json(
        attribute: *mut c_void,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_attribute_description_set_default_value_json(
        attribute: *mut c_void,
        value_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_attribute_description_get_value_transformer_name(
        attribute: *mut c_void,
    ) -> *mut c_char;
    pub fn cd_attribute_description_set_value_transformer_name(
        attribute: *mut c_void,
        name: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_attribute_description_get_allows_external_binary_data_storage(
        attribute: *mut c_void,
    ) -> i32;
    pub fn cd_attribute_description_set_allows_external_binary_data_storage(
        attribute: *mut c_void,
        allows_external_storage: i32,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_attribute_description_get_preserves_value_in_history_on_deletion(
        attribute: *mut c_void,
    ) -> i32;
    pub fn cd_attribute_description_set_preserves_value_in_history_on_deletion(
        attribute: *mut c_void,
        preserves_value: i32,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_attribute_description_get_allows_cloud_encryption(attribute: *mut c_void) -> i32;
    pub fn cd_attribute_description_set_allows_cloud_encryption(
        attribute: *mut c_void,
        allows_encryption: i32,
        out_error: *mut *mut c_char,
    ) -> i32;
}
