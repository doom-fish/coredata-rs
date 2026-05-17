use core::ffi::{c_char, c_void};

extern "C" {
    pub fn cd_fetched_results_controller_new(
        fetch_request: *mut c_void,
        managed_object_context: *mut c_void,
        section_name_key_path: *const c_char,
        cache_name: *const c_char,
        out_controller: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_fetched_results_controller_perform_fetch(
        controller: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_fetched_results_controller_get_fetch_request(controller: *mut c_void) -> *mut c_void;
    pub fn cd_fetched_results_controller_get_managed_object_context(
        controller: *mut c_void,
    ) -> *mut c_void;
    pub fn cd_fetched_results_controller_get_section_name_key_path(
        controller: *mut c_void,
    ) -> *mut c_char;
    pub fn cd_fetched_results_controller_get_cache_name(controller: *mut c_void) -> *mut c_char;
    pub fn cd_fetched_results_controller_get_fetched_objects(
        controller: *mut c_void,
    ) -> *mut c_void;
    pub fn cd_fetched_results_controller_get_sections(controller: *mut c_void) -> *mut c_void;
    pub fn cd_fetched_results_controller_get_section_index_titles_json(
        controller: *mut c_void,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_fetched_results_controller_object_at_index_path(
        controller: *mut c_void,
        section: i64,
        item: i64,
        out_object: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_fetched_results_controller_index_path_for_object(
        controller: *mut c_void,
        object: *mut c_void,
        out_section: *mut i64,
        out_item: *mut i64,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_fetched_results_controller_delete_cache_with_name(name: *const c_char);

    pub fn cd_fetched_results_section_info_get_name(section_info: *mut c_void) -> *mut c_char;
    pub fn cd_fetched_results_section_info_get_index_title(
        section_info: *mut c_void,
    ) -> *mut c_char;
    pub fn cd_fetched_results_section_info_get_number_of_objects(section_info: *mut c_void) -> u64;
    pub fn cd_fetched_results_section_info_get_objects(section_info: *mut c_void) -> *mut c_void;
}
