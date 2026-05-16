use core::ffi::{c_char, c_void};

extern "C" {
    pub fn cd_persistent_store_description_new(
        path: *const c_char,
        out_description: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_store_description_get_type(description: *mut c_void) -> *mut c_char;
    pub fn cd_persistent_store_description_set_type(
        description: *mut c_void,
        store_type: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_store_description_get_configuration(
        description: *mut c_void,
    ) -> *mut c_char;
    pub fn cd_persistent_store_description_set_configuration(
        description: *mut c_void,
        configuration: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_store_description_get_url(description: *mut c_void) -> *mut c_char;
    pub fn cd_persistent_store_description_set_url(
        description: *mut c_void,
        path: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_store_description_get_read_only(description: *mut c_void) -> i32;
    pub fn cd_persistent_store_description_set_read_only(
        description: *mut c_void,
        read_only: i32,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_store_description_get_timeout(description: *mut c_void) -> f64;
    pub fn cd_persistent_store_description_set_timeout(
        description: *mut c_void,
        timeout: f64,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_store_description_sqlite_pragmas_json(
        description: *mut c_void,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_store_description_set_sqlite_pragma(
        description: *mut c_void,
        name: *const c_char,
        value: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_store_description_get_should_add_asynchronously(
        description: *mut c_void,
    ) -> i32;
    pub fn cd_persistent_store_description_set_should_add_asynchronously(
        description: *mut c_void,
        asynchronous: i32,
    );
    pub fn cd_persistent_store_description_get_should_migrate_automatically(
        description: *mut c_void,
    ) -> i32;
    pub fn cd_persistent_store_description_set_should_migrate_automatically(
        description: *mut c_void,
        automatically_migrate: i32,
    );
    pub fn cd_persistent_store_description_get_should_infer_mapping_model_automatically(
        description: *mut c_void,
    ) -> i32;
    pub fn cd_persistent_store_description_set_should_infer_mapping_model_automatically(
        description: *mut c_void,
        automatically_infer: i32,
    );
    pub fn cd_persistent_store_description_set_option_json(
        description: *mut c_void,
        key: *const c_char,
        value_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cd_persistent_container_default_directory() -> *mut c_char;
    pub fn cd_persistent_container_get_name(container: *mut c_void) -> *mut c_char;
    pub fn cd_persistent_container_managed_object_model(container: *mut c_void) -> *mut c_void;
    pub fn cd_persistent_container_persistent_store_coordinator(
        container: *mut c_void,
    ) -> *mut c_void;
    pub fn cd_persistent_container_persistent_store_descriptions(
        container: *mut c_void,
    ) -> *mut c_void;
    pub fn cd_persistent_container_set_persistent_store_descriptions(
        container: *mut c_void,
        descriptions: *const *mut c_void,
        count: i32,
        out_error: *mut *mut c_char,
    ) -> i32;
}
