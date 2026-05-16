use core::ffi::{c_char, c_void};

extern "C" {
    pub fn cd_persistent_history_change_request_fetch_after_token(
        token: *mut c_void,
        out_request: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_history_change_request_fetch_after_date(
        timestamp: f64,
        out_request: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_history_change_request_fetch_after_transaction(
        transaction: *mut c_void,
        out_request: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_history_change_request_delete_before_token(
        token: *mut c_void,
        out_request: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_history_change_request_delete_before_date(
        timestamp: f64,
        out_request: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_history_change_request_delete_before_transaction(
        transaction: *mut c_void,
        out_request: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_history_change_request_get_result_type(request: *mut c_void) -> i64;
    pub fn cd_persistent_history_change_request_set_result_type(
        request: *mut c_void,
        result_type: i64,
    );
    pub fn cd_managed_object_context_execute_persistent_history_change_request(
        context: *mut c_void,
        request: *mut c_void,
        out_result: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_history_result_get_result_type(result: *mut c_void) -> i64;
    pub fn cd_persistent_history_result_get_status(result: *mut c_void) -> i32;
    pub fn cd_persistent_history_result_get_count(result: *mut c_void) -> u64;
    pub fn cd_persistent_history_result_get_object_ids(result: *mut c_void) -> *mut c_void;
    pub fn cd_persistent_history_result_get_transactions(result: *mut c_void) -> *mut c_void;
    pub fn cd_persistent_history_result_get_changes(result: *mut c_void) -> *mut c_void;

    pub fn cd_persistent_history_transaction_get_timestamp(transaction: *mut c_void) -> f64;
    pub fn cd_persistent_history_transaction_get_changes(transaction: *mut c_void) -> *mut c_void;
    pub fn cd_persistent_history_transaction_get_transaction_number(
        transaction: *mut c_void,
    ) -> i64;
    pub fn cd_persistent_history_transaction_get_store_id(transaction: *mut c_void) -> *mut c_char;
    pub fn cd_persistent_history_transaction_get_bundle_id(transaction: *mut c_void)
        -> *mut c_char;
    pub fn cd_persistent_history_transaction_get_process_id(
        transaction: *mut c_void,
    ) -> *mut c_char;
    pub fn cd_persistent_history_transaction_get_context_name(
        transaction: *mut c_void,
    ) -> *mut c_char;
    pub fn cd_persistent_history_transaction_get_author(transaction: *mut c_void) -> *mut c_char;
    pub fn cd_persistent_history_transaction_get_token(transaction: *mut c_void) -> *mut c_void;

    pub fn cd_persistent_history_change_get_change_id(change: *mut c_void) -> i64;
    pub fn cd_persistent_history_change_get_change_type(change: *mut c_void) -> i64;
    pub fn cd_persistent_history_change_get_changed_object_id(change: *mut c_void) -> *mut c_void;
    pub fn cd_persistent_history_change_get_tombstone_json(
        change: *mut c_void,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_history_change_get_updated_properties_json(
        change: *mut c_void,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_history_change_get_transaction(change: *mut c_void) -> *mut c_void;
}
