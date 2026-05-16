use core::ffi::{c_char, c_void};

extern "C" {
    pub fn cd_mapping_model_inferred(
        source_model: *mut c_void,
        destination_model: *mut c_void,
        out_model: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_mapping_model_from_url(
        path: *const c_char,
        out_model: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_mapping_model_get_entity_mapping_names_json(
        model: *mut c_void,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cd_migration_manager_new(
        source_model: *mut c_void,
        destination_model: *mut c_void,
        out_manager: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_migration_manager_get_source_model(manager: *mut c_void) -> *mut c_void;
    pub fn cd_migration_manager_get_destination_model(manager: *mut c_void) -> *mut c_void;
    pub fn cd_migration_manager_get_source_context(manager: *mut c_void) -> *mut c_void;
    pub fn cd_migration_manager_get_destination_context(manager: *mut c_void) -> *mut c_void;
    pub fn cd_migration_manager_get_uses_store_specific_migration_manager(
        manager: *mut c_void,
    ) -> i32;
    pub fn cd_migration_manager_set_uses_store_specific_migration_manager(
        manager: *mut c_void,
        uses_store_specific_migration_manager: i32,
    );
    pub fn cd_migration_manager_get_migration_progress(manager: *mut c_void) -> f32;
    pub fn cd_migration_manager_migrate_store(
        manager: *mut c_void,
        source_url: *const c_char,
        source_store_type: *const c_char,
        source_options_json: *const c_char,
        mapping_model: *mut c_void,
        destination_url: *const c_char,
        destination_store_type: *const c_char,
        destination_options_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
}
