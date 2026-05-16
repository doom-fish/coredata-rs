use core::ffi::{c_char, c_void};

extern "C" {
    pub fn cd_merge_policy_new(
        merge_type: u64,
        out_policy: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_merge_policy_error_policy() -> *mut c_void;
    pub fn cd_merge_policy_rollback_policy() -> *mut c_void;
    pub fn cd_merge_policy_overwrite_policy() -> *mut c_void;
    pub fn cd_merge_policy_merge_by_property_object_trump_policy() -> *mut c_void;
    pub fn cd_merge_policy_merge_by_property_store_trump_policy() -> *mut c_void;
    pub fn cd_merge_policy_get_merge_type(policy: *mut c_void) -> u64;

    pub fn cd_managed_object_context_get_merge_policy(context: *mut c_void) -> *mut c_void;
    pub fn cd_managed_object_context_set_merge_policy(
        context: *mut c_void,
        merge_policy: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
}
