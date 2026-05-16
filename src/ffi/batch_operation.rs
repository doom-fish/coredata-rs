use core::ffi::{c_char, c_void};

extern "C" {
    pub fn cd_batch_delete_request_new_with_fetch_request(
        fetch_request: *mut c_void,
        out_request: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_batch_delete_request_new_with_object_ids(
        object_ids: *const *mut c_void,
        count: i32,
        out_request: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_batch_delete_request_get_result_type(request: *mut c_void) -> u64;
    pub fn cd_batch_delete_request_set_result_type(request: *mut c_void, result_type: u64);
    pub fn cd_managed_object_context_execute_batch_delete_request(
        context: *mut c_void,
        request: *mut c_void,
        out_result: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_batch_delete_result_get_result_type(result: *mut c_void) -> u64;
    pub fn cd_batch_delete_result_get_status(result: *mut c_void) -> i32;
    pub fn cd_batch_delete_result_get_count(result: *mut c_void) -> u64;
    pub fn cd_batch_delete_result_get_object_ids(result: *mut c_void) -> *mut c_void;

    pub fn cd_batch_insert_request_new_with_entity_name(
        entity_name: *const c_char,
        objects_json: *const c_char,
        out_request: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_batch_insert_request_get_entity_name(request: *mut c_void) -> *mut c_char;
    pub fn cd_batch_insert_request_get_result_type(request: *mut c_void) -> u64;
    pub fn cd_batch_insert_request_set_result_type(request: *mut c_void, result_type: u64);
    pub fn cd_managed_object_context_execute_batch_insert_request(
        context: *mut c_void,
        request: *mut c_void,
        out_result: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_batch_insert_result_get_result_type(result: *mut c_void) -> u64;
    pub fn cd_batch_insert_result_get_status(result: *mut c_void) -> i32;
    pub fn cd_batch_insert_result_get_count(result: *mut c_void) -> u64;
    pub fn cd_batch_insert_result_get_object_ids(result: *mut c_void) -> *mut c_void;

    pub fn cd_batch_update_request_new_with_entity_name(
        entity_name: *const c_char,
        out_request: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_batch_update_request_get_entity_name(request: *mut c_void) -> *mut c_char;
    pub fn cd_batch_update_request_get_includes_subentities(request: *mut c_void) -> i32;
    pub fn cd_batch_update_request_set_includes_subentities(
        request: *mut c_void,
        includes_subentities: i32,
    );
    pub fn cd_batch_update_request_get_result_type(request: *mut c_void) -> u64;
    pub fn cd_batch_update_request_set_result_type(request: *mut c_void, result_type: u64);
    pub fn cd_batch_update_request_set_predicate(request: *mut c_void, predicate: *mut c_void);
    pub fn cd_batch_update_request_set_properties_to_update_json(
        request: *mut c_void,
        properties_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_managed_object_context_execute_batch_update_request(
        context: *mut c_void,
        request: *mut c_void,
        out_result: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_batch_update_result_get_result_type(result: *mut c_void) -> u64;
    pub fn cd_batch_update_result_get_status(result: *mut c_void) -> i32;
    pub fn cd_batch_update_result_get_count(result: *mut c_void) -> u64;
    pub fn cd_batch_update_result_get_object_ids(result: *mut c_void) -> *mut c_void;
}
