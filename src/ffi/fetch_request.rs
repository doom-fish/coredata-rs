use core::ffi::{c_char, c_void};

extern "C" {
    pub fn cd_fetch_request_get_entity(request: *mut c_void) -> *mut c_void;
    pub fn cd_fetch_request_set_entity(
        request: *mut c_void,
        entity: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_fetch_request_get_entity_name(request: *mut c_void) -> *mut c_char;
    pub fn cd_fetch_request_get_result_type(request: *mut c_void) -> u64;
    pub fn cd_fetch_request_set_result_type(
        request: *mut c_void,
        result_type: u64,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_fetch_request_get_includes_subentities(request: *mut c_void) -> i32;
    pub fn cd_fetch_request_set_includes_subentities(
        request: *mut c_void,
        includes_subentities: i32,
    );
    pub fn cd_fetch_request_get_includes_property_values(request: *mut c_void) -> i32;
    pub fn cd_fetch_request_set_includes_property_values(
        request: *mut c_void,
        includes_property_values: i32,
    );
    pub fn cd_fetch_request_get_returns_objects_as_faults(request: *mut c_void) -> i32;
    pub fn cd_fetch_request_set_returns_objects_as_faults(
        request: *mut c_void,
        returns_objects_as_faults: i32,
    );
    pub fn cd_fetch_request_get_relationship_key_paths_for_prefetching_json(
        request: *mut c_void,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_fetch_request_set_relationship_key_paths_for_prefetching_json(
        request: *mut c_void,
        key_paths_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_fetch_request_get_includes_pending_changes(request: *mut c_void) -> i32;
    pub fn cd_fetch_request_set_includes_pending_changes(
        request: *mut c_void,
        includes_pending: i32,
    );
    pub fn cd_fetch_request_get_returns_distinct_results(request: *mut c_void) -> i32;
    pub fn cd_fetch_request_set_returns_distinct_results(request: *mut c_void, distinct: i32);
    pub fn cd_fetch_request_get_fetch_batch_size(request: *mut c_void) -> u64;
    pub fn cd_fetch_request_set_fetch_batch_size(request: *mut c_void, fetch_batch_size: u64);
    pub fn cd_fetch_request_get_should_refresh_refetched_objects(request: *mut c_void) -> i32;
    pub fn cd_fetch_request_set_should_refresh_refetched_objects(
        request: *mut c_void,
        should_refresh: i32,
    );
    pub fn cd_fetch_request_execute_object_ids(
        request: *mut c_void,
        context: *mut c_void,
        out_array: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
}
