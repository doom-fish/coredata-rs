use core::ffi::{c_char, c_void};

extern "C" {
    pub fn cd_persistent_store_request_get_affected_stores(request: *mut c_void) -> *mut c_void;
    pub fn cd_persistent_store_request_set_affected_stores(
        request: *mut c_void,
        stores: *const *mut c_void,
        count: i32,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_store_request_get_request_type(request: *mut c_void) -> u64;

    pub fn cd_asynchronous_fetch_request_new(
        fetch_request: *mut c_void,
        out_request: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_asynchronous_fetch_request_get_fetch_request(request: *mut c_void) -> *mut c_void;
    pub fn cd_asynchronous_fetch_request_get_estimated_result_count(request: *mut c_void) -> i64;
    pub fn cd_asynchronous_fetch_request_set_estimated_result_count(
        request: *mut c_void,
        estimated_result_count: i64,
    );
    pub fn cd_managed_object_context_execute_asynchronous_fetch_request(
        context: *mut c_void,
        request: *mut c_void,
        out_result: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cd_persistent_store_async_result_get_managed_object_context(
        result: *mut c_void,
    ) -> *mut c_void;
    pub fn cd_persistent_store_async_result_get_operation_error_json(
        result: *mut c_void,
    ) -> *mut c_char;
    pub fn cd_persistent_store_async_result_get_progress_fraction_completed(
        result: *mut c_void,
    ) -> f64;
    pub fn cd_persistent_store_async_result_has_progress(result: *mut c_void) -> i32;

    pub fn cd_asynchronous_fetch_result_get_fetch_request(result: *mut c_void) -> *mut c_void;
    pub fn cd_asynchronous_fetch_result_get_final_result_count(result: *mut c_void) -> u64;

    pub fn cd_save_changes_request_new(
        inserted_objects: *const *mut c_void,
        inserted_count: i32,
        updated_objects: *const *mut c_void,
        updated_count: i32,
        deleted_objects: *const *mut c_void,
        deleted_count: i32,
        locked_objects: *const *mut c_void,
        locked_count: i32,
        out_request: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_save_changes_request_get_inserted_objects(request: *mut c_void) -> *mut c_void;
    pub fn cd_save_changes_request_get_updated_objects(request: *mut c_void) -> *mut c_void;
    pub fn cd_save_changes_request_get_deleted_objects(request: *mut c_void) -> *mut c_void;
    pub fn cd_save_changes_request_get_locked_objects(request: *mut c_void) -> *mut c_void;

    pub fn cd_fetch_request_expression_new(
        fetch_request: *mut c_void,
        context: *mut c_void,
        count_only: i32,
        out_expression: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_fetch_request_expression_get_count_only_request(expression: *mut c_void) -> i32;
}
