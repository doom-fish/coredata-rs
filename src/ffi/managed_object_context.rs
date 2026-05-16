use core::ffi::{c_char, c_void};

extern "C" {
    pub fn cd_managed_object_context_get_name(context: *mut c_void) -> *mut c_char;
    pub fn cd_managed_object_context_set_name(
        context: *mut c_void,
        name: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_managed_object_context_get_parent_context(context: *mut c_void) -> *mut c_void;
    pub fn cd_managed_object_context_set_parent_context(
        context: *mut c_void,
        parent_context: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_managed_object_context_get_concurrency_type(context: *mut c_void) -> i32;
    pub fn cd_managed_object_context_get_inserted_objects(context: *mut c_void) -> *mut c_void;
    pub fn cd_managed_object_context_get_updated_objects(context: *mut c_void) -> *mut c_void;
    pub fn cd_managed_object_context_get_deleted_objects(context: *mut c_void) -> *mut c_void;
    pub fn cd_managed_object_context_get_registered_objects(context: *mut c_void) -> *mut c_void;
    pub fn cd_managed_object_context_object_registered_for_id(
        context: *mut c_void,
        object_id: *mut c_void,
    ) -> *mut c_void;
    pub fn cd_managed_object_context_object_with_id(
        context: *mut c_void,
        object_id: *mut c_void,
    ) -> *mut c_void;
    pub fn cd_managed_object_context_existing_object_with_id(
        context: *mut c_void,
        object_id: *mut c_void,
        out_object: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_managed_object_context_count_for_fetch_request(
        context: *mut c_void,
        request: *mut c_void,
        out_count: *mut u64,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_managed_object_context_refresh_object(
        context: *mut c_void,
        object: *mut c_void,
        merge_changes: i32,
    );
    pub fn cd_managed_object_context_process_pending_changes(context: *mut c_void);
    pub fn cd_managed_object_context_reset(context: *mut c_void);
    pub fn cd_managed_object_context_rollback(context: *mut c_void);
    pub fn cd_managed_object_context_refresh_all_objects(context: *mut c_void);
    pub fn cd_managed_object_context_get_automatically_merges_changes_from_parent(
        context: *mut c_void,
    ) -> i32;
    pub fn cd_managed_object_context_set_automatically_merges_changes_from_parent(
        context: *mut c_void,
        automatically_merges: i32,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_managed_object_context_get_transaction_author(context: *mut c_void) -> *mut c_char;
    pub fn cd_managed_object_context_set_transaction_author(
        context: *mut c_void,
        author: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_managed_object_context_obtain_permanent_ids(
        context: *mut c_void,
        objects: *const *mut c_void,
        count: i32,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_managed_object_context_merge_changes_from_history_transaction(
        context: *mut c_void,
        transaction: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
}
