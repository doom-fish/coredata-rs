use core::ffi::{c_char, c_void};

extern "C" {
    pub fn cd_property_description_new(
        out_property: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_property_description_get_name(property: *mut c_void) -> *mut c_char;
    pub fn cd_property_description_set_name(
        property: *mut c_void,
        name: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_property_description_get_optional(property: *mut c_void) -> i32;
    pub fn cd_property_description_set_optional(
        property: *mut c_void,
        optional: i32,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_property_description_get_transient(property: *mut c_void) -> i32;
    pub fn cd_property_description_set_transient(
        property: *mut c_void,
        transient: i32,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cd_fetched_property_description_new(
        out_property: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_fetched_property_description_get_fetch_request(property: *mut c_void) -> *mut c_void;
    pub fn cd_fetched_property_description_set_fetch_request(
        property: *mut c_void,
        fetch_request: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cd_expression_description_new(
        out_property: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_expression_description_get_result_type(property: *mut c_void) -> u64;
    pub fn cd_expression_description_set_result_type(
        property: *mut c_void,
        result_type: u64,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cd_derived_attribute_description_new(
        out_attribute: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cd_composite_attribute_description_new(
        out_attribute: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_composite_attribute_description_get_elements(attribute: *mut c_void) -> *mut c_void;
    pub fn cd_composite_attribute_description_set_elements(
        attribute: *mut c_void,
        elements: *const *mut c_void,
        count: i32,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cd_fetch_index_element_description_new(
        property: *mut c_void,
        collation_type: u64,
        out_element: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_fetch_index_element_description_get_property_name(
        element: *mut c_void,
    ) -> *mut c_char;
    pub fn cd_fetch_index_element_description_get_collation_type(element: *mut c_void) -> u64;
    pub fn cd_fetch_index_element_description_set_collation_type(
        element: *mut c_void,
        collation_type: u64,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_fetch_index_element_description_get_ascending(element: *mut c_void) -> i32;
    pub fn cd_fetch_index_element_description_set_ascending(
        element: *mut c_void,
        ascending: i32,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cd_fetch_index_description_new(
        name: *const c_char,
        elements: *const *mut c_void,
        count: i32,
        out_index: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_fetch_index_description_get_name(index: *mut c_void) -> *mut c_char;
    pub fn cd_fetch_index_description_set_name(
        index: *mut c_void,
        name: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_fetch_index_description_get_elements(index: *mut c_void) -> *mut c_void;
    pub fn cd_fetch_index_description_set_elements(
        index: *mut c_void,
        elements: *const *mut c_void,
        count: i32,
        out_error: *mut *mut c_char,
    ) -> i32;
}
