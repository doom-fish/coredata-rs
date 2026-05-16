use core::ffi::{c_char, c_void};

extern "C" {
    pub fn cd_managed_object_get_managed_object_context(object: *mut c_void) -> *mut c_void;
    pub fn cd_managed_object_get_object_id(object: *mut c_void) -> *mut c_void;
    pub fn cd_managed_object_get_inserted(object: *mut c_void) -> i32;
    pub fn cd_managed_object_get_updated(object: *mut c_void) -> i32;
    pub fn cd_managed_object_get_deleted(object: *mut c_void) -> i32;
    pub fn cd_managed_object_get_has_changes(object: *mut c_void) -> i32;
    pub fn cd_managed_object_get_has_persistent_changed_values(object: *mut c_void) -> i32;
    pub fn cd_managed_object_get_fault(object: *mut c_void) -> i32;
    pub fn cd_managed_object_has_fault_for_relationship_named(
        object: *mut c_void,
        relationship_name: *const c_char,
    ) -> i32;
    pub fn cd_managed_object_object_ids_for_relationship_named(
        object: *mut c_void,
        relationship_name: *const c_char,
    ) -> *mut c_void;
    pub fn cd_managed_object_committed_values_json(
        object: *mut c_void,
        keys_json: *const c_char,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_managed_object_changed_values_json(
        object: *mut c_void,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_managed_object_changed_values_for_current_event_json(
        object: *mut c_void,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cd_managed_object_id_get_entity(object_id: *mut c_void) -> *mut c_void;
    pub fn cd_managed_object_id_get_temporary(object_id: *mut c_void) -> i32;
    pub fn cd_managed_object_id_get_uri(object_id: *mut c_void) -> *mut c_char;
}
