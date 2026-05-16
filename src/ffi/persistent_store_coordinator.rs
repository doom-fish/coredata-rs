use core::ffi::{c_char, c_void};

extern "C" {
    pub fn cd_persistent_store_coordinator_get_name(coordinator: *mut c_void) -> *mut c_char;
    pub fn cd_persistent_store_coordinator_set_name(
        coordinator: *mut c_void,
        name: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_store_coordinator_managed_object_model(
        coordinator: *mut c_void,
    ) -> *mut c_void;
    pub fn cd_persistent_store_coordinator_persistent_stores(
        coordinator: *mut c_void,
    ) -> *mut c_void;
    pub fn cd_persistent_store_coordinator_add_persistent_store_with_description(
        coordinator: *mut c_void,
        description: *mut c_void,
        timeout_seconds: i32,
        out_store: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_store_coordinator_remove_persistent_store(
        coordinator: *mut c_void,
        store: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_store_coordinator_persistent_store_for_url(
        coordinator: *mut c_void,
        path: *const c_char,
    ) -> *mut c_void;
    pub fn cd_persistent_store_coordinator_url_for_persistent_store(
        coordinator: *mut c_void,
        store: *mut c_void,
    ) -> *mut c_char;
    pub fn cd_persistent_store_coordinator_current_persistent_history_token(
        coordinator: *mut c_void,
    ) -> *mut c_void;
    pub fn cd_persistent_store_coordinator_destroy_persistent_store(
        coordinator: *mut c_void,
        path: *const c_char,
        store_type: *const c_char,
        options_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cd_persistent_store_get_configuration_name(store: *mut c_void) -> *mut c_char;
    pub fn cd_persistent_store_get_url(store: *mut c_void) -> *mut c_char;
    pub fn cd_persistent_store_set_url(
        store: *mut c_void,
        path: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_store_get_identifier(store: *mut c_void) -> *mut c_char;
    pub fn cd_persistent_store_set_identifier(
        store: *mut c_void,
        identifier: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_store_get_type(store: *mut c_void) -> *mut c_char;
    pub fn cd_persistent_store_get_read_only(store: *mut c_void) -> i32;
    pub fn cd_persistent_store_set_read_only(
        store: *mut c_void,
        read_only: i32,
        out_error: *mut *mut c_char,
    ) -> i32;
}
