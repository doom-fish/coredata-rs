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

    pub fn cd_persistent_cloudkit_event_request_fetch_after_date(
        timestamp: f64,
        out_request: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_cloudkit_event_request_fetch_after_event(
        event: *mut c_void,
        out_request: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_cloudkit_event_request_fetch_request_for_events(
        out_fetch_request: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_cloudkit_event_request_get_result_type(request: *mut c_void) -> i64;
    pub fn cd_persistent_cloudkit_event_request_set_result_type(
        request: *mut c_void,
        result_type: i64,
    );
    pub fn cd_managed_object_context_execute_persistent_cloudkit_event_request(
        context: *mut c_void,
        request: *mut c_void,
        out_result: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_cloudkit_event_result_get_result_type(result: *mut c_void) -> i64;
    pub fn cd_persistent_cloudkit_event_result_get_events(result: *mut c_void) -> *mut c_void;
    pub fn cd_persistent_cloudkit_event_result_get_counts_json(
        result: *mut c_void,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_cloudkit_event_get_identifier(event: *mut c_void) -> *mut c_char;
    pub fn cd_persistent_cloudkit_event_get_store_identifier(event: *mut c_void) -> *mut c_char;
    pub fn cd_persistent_cloudkit_event_get_type(event: *mut c_void) -> i64;
    pub fn cd_persistent_cloudkit_event_get_start_timestamp(event: *mut c_void) -> f64;
    pub fn cd_persistent_cloudkit_event_get_end_timestamp(event: *mut c_void) -> f64;
    pub fn cd_persistent_cloudkit_event_has_end_date(event: *mut c_void) -> i32;
    pub fn cd_persistent_cloudkit_event_get_succeeded(event: *mut c_void) -> i32;
}
