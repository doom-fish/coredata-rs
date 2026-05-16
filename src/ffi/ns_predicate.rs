use core::ffi::{c_char, c_void};

extern "C" {
    pub fn cd_predicate_new_with_value(
        value: i32,
        out_predicate: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_predicate_get_format(predicate: *mut c_void) -> *mut c_char;
    pub fn cd_predicate_with_substitution_variables(
        predicate: *mut c_void,
        variables_json: *const c_char,
        out_predicate: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_predicate_evaluate_with_object_json(
        predicate: *mut c_void,
        object_json: *const c_char,
        substitution_variables_json: *const c_char,
        out_result: *mut i32,
        out_error: *mut *mut c_char,
    ) -> i32;
}
