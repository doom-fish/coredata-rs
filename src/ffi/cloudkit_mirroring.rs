use core::ffi::{c_char, c_void};

extern "C" {
    pub fn cd_persistent_cloudkit_container_options_new(
        container_identifier: *const c_char,
        out_options: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_cloudkit_container_options_get_container_identifier(
        options: *mut c_void,
    ) -> *mut c_char;
    pub fn cd_persistent_cloudkit_container_options_get_database_scope(options: *mut c_void)
        -> i64;
    pub fn cd_persistent_cloudkit_container_options_set_database_scope(
        options: *mut c_void,
        database_scope: i64,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_store_description_get_cloudkit_container_options(
        description: *mut c_void,
    ) -> *mut c_void;
    pub fn cd_persistent_store_description_set_cloudkit_container_options(
        description: *mut c_void,
        options: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cd_persistent_cloudkit_container_new(
        name: *const c_char,
        model: *mut c_void,
        out_container: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_cloudkit_container_get_name(container: *mut c_void) -> *mut c_char;
    pub fn cd_persistent_cloudkit_container_managed_object_model(
        container: *mut c_void,
    ) -> *mut c_void;
    pub fn cd_persistent_cloudkit_container_persistent_store_coordinator(
        container: *mut c_void,
    ) -> *mut c_void;
    pub fn cd_persistent_cloudkit_container_persistent_store_descriptions(
        container: *mut c_void,
    ) -> *mut c_void;
    pub fn cd_persistent_cloudkit_container_set_persistent_store_descriptions(
        container: *mut c_void,
        descriptions: *const *mut c_void,
        count: i32,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_cloudkit_container_load_persistent_stores(
        container: *mut c_void,
        timeout_seconds: i32,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_cloudkit_container_view_context(container: *mut c_void) -> *mut c_void;
    pub fn cd_persistent_cloudkit_container_new_background_context(
        container: *mut c_void,
    ) -> *mut c_void;
    pub fn cd_persistent_cloudkit_container_initialize_schema(
        container: *mut c_void,
        options_raw_value: u64,
        out_error: *mut *mut c_char,
    ) -> i32;
}
