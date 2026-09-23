use core::ffi::{c_char, c_void};

extern "C" {
    pub fn cd_query_generation_token_current() -> *mut c_void;
    pub fn cd_managed_object_context_get_query_generation_token(
        context: *mut c_void,
        out_token: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_managed_object_context_set_query_generation_from_token(
        context: *mut c_void,
        token: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
}
