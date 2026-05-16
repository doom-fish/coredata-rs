use core::ffi::c_void;

extern "C" {
    pub fn cd_relationship_description_get_to_many(relationship: *mut c_void) -> i32;
    pub fn cd_relationship_description_get_ordered(relationship: *mut c_void) -> i32;
    pub fn cd_relationship_description_set_ordered(
        relationship: *mut c_void,
        ordered: i32,
        out_error: *mut *mut core::ffi::c_char,
    ) -> i32;
    pub fn cd_relationship_description_get_version_hash(
        relationship: *mut c_void,
    ) -> *mut core::ffi::c_char;
}
