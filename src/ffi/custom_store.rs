use core::ffi::{c_char, c_void};

extern "C" {
    pub fn cd_atomic_store_cache_node_new(
        object_id: *mut c_void,
        out_node: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_atomic_store_cache_node_get_object_id(node: *mut c_void) -> *mut c_void;

    pub fn cd_incremental_store_node_new(
        object_id: *mut c_void,
        values_json: *const c_char,
        version: u64,
        out_node: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_incremental_store_node_get_object_id(node: *mut c_void) -> *mut c_void;
    pub fn cd_incremental_store_node_get_version(node: *mut c_void) -> u64;
    pub fn cd_incremental_store_node_update(
        node: *mut c_void,
        values_json: *const c_char,
        version: u64,
        out_error: *mut *mut c_char,
    ) -> i32;
}
