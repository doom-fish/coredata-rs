use core::ffi::{c_char, c_void};

extern "C" {
    pub fn cd_entity_mapping_new(out_mapping: *mut *mut c_void, out_error: *mut *mut c_char)
        -> i32;
    pub fn cd_entity_mapping_get_name(mapping: *mut c_void) -> *mut c_char;
    pub fn cd_entity_mapping_set_name(
        mapping: *mut c_void,
        name: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_entity_mapping_get_mapping_type(mapping: *mut c_void) -> u64;
    pub fn cd_entity_mapping_set_mapping_type(
        mapping: *mut c_void,
        mapping_type: u64,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_entity_mapping_get_source_entity_name(mapping: *mut c_void) -> *mut c_char;
    pub fn cd_entity_mapping_set_source_entity_name(
        mapping: *mut c_void,
        name: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_entity_mapping_get_destination_entity_name(mapping: *mut c_void) -> *mut c_char;
    pub fn cd_entity_mapping_set_destination_entity_name(
        mapping: *mut c_void,
        name: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cd_property_mapping_new(
        out_mapping: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_property_mapping_get_name(mapping: *mut c_void) -> *mut c_char;
    pub fn cd_property_mapping_set_name(
        mapping: *mut c_void,
        name: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cd_managed_object_model_reference_new_with_model(
        model: *mut c_void,
        version_checksum: *const c_char,
        out_reference: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_managed_object_model_reference_get_resolved_model(
        reference: *mut c_void,
    ) -> *mut c_void;
    pub fn cd_managed_object_model_reference_get_version_checksum(
        reference: *mut c_void,
    ) -> *mut c_char;

    pub fn cd_migration_stage_get_label(stage: *mut c_void) -> *mut c_char;
    pub fn cd_migration_stage_set_label(
        stage: *mut c_void,
        label: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cd_lightweight_migration_stage_new(
        version_checksums_json: *const c_char,
        out_stage: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_lightweight_migration_stage_get_version_checksums_json(
        stage: *mut c_void,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cd_custom_migration_stage_new(
        current_model: *mut c_void,
        next_model: *mut c_void,
        out_stage: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_custom_migration_stage_get_current_model(stage: *mut c_void) -> *mut c_void;
    pub fn cd_custom_migration_stage_get_next_model(stage: *mut c_void) -> *mut c_void;

    pub fn cd_staged_migration_manager_new(
        stages: *const *mut c_void,
        count: i32,
        out_manager: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_staged_migration_manager_get_stages(manager: *mut c_void) -> *mut c_void;
    pub fn cd_staged_migration_manager_get_container(manager: *mut c_void) -> *mut c_void;
}
