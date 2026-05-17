#![allow(missing_docs, non_camel_case_types)]

use core::ffi::{c_char, c_void};

extern "C" {
    pub fn cd_string_free(s: *mut c_char);
    pub fn cd_retain_object(ptr: *mut c_void) -> *mut c_void;
    pub fn cd_release_object(ptr: *mut c_void);

    pub fn cd_array_count(array: *mut c_void) -> i32;
    pub fn cd_array_get_object(array: *mut c_void, index: i32) -> *mut c_void;

    pub fn cd_managed_object_model_new(
        out_model: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_managed_object_model_from_url(
        path: *const c_char,
        out_model: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_managed_object_model_add_entity(
        model: *mut c_void,
        entity: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_managed_object_model_entities(model: *mut c_void) -> *mut c_void;
    pub fn cd_managed_object_model_get_version_checksum(model: *mut c_void) -> *mut c_char;

    pub fn cd_entity_description_new(
        out_entity: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_entity_description_get_name(entity: *mut c_void) -> *mut c_char;
    pub fn cd_entity_description_set_name(
        entity: *mut c_void,
        name: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_entity_description_get_managed_object_class_name(entity: *mut c_void) -> *mut c_char;
    pub fn cd_entity_description_set_managed_object_class_name(
        entity: *mut c_void,
        name: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_entity_description_add_attribute(
        entity: *mut c_void,
        attribute: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_entity_description_add_relationship(
        entity: *mut c_void,
        relationship: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_entity_description_attributes(entity: *mut c_void) -> *mut c_void;
    pub fn cd_entity_description_relationships(entity: *mut c_void) -> *mut c_void;

    pub fn cd_attribute_description_new(
        out_attribute: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_attribute_description_get_name(attribute: *mut c_void) -> *mut c_char;
    pub fn cd_attribute_description_set_name(
        attribute: *mut c_void,
        name: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_attribute_description_get_optional(attribute: *mut c_void) -> i32;
    pub fn cd_attribute_description_set_optional(
        attribute: *mut c_void,
        optional: i32,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_attribute_description_get_transient(attribute: *mut c_void) -> i32;
    pub fn cd_attribute_description_set_transient(
        attribute: *mut c_void,
        transient: i32,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_attribute_description_get_attribute_type(attribute: *mut c_void) -> u64;
    pub fn cd_attribute_description_set_attribute_type(
        attribute: *mut c_void,
        attribute_type: u64,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cd_relationship_description_new(
        out_relationship: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_relationship_description_get_name(relationship: *mut c_void) -> *mut c_char;
    pub fn cd_relationship_description_set_name(
        relationship: *mut c_void,
        name: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_relationship_description_get_optional(relationship: *mut c_void) -> i32;
    pub fn cd_relationship_description_set_optional(
        relationship: *mut c_void,
        optional: i32,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_relationship_description_get_transient(relationship: *mut c_void) -> i32;
    pub fn cd_relationship_description_set_transient(
        relationship: *mut c_void,
        transient: i32,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_relationship_description_get_destination_entity(
        relationship: *mut c_void,
    ) -> *mut c_void;
    pub fn cd_relationship_description_set_destination_entity(
        relationship: *mut c_void,
        entity: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_relationship_description_get_inverse_relationship(
        relationship: *mut c_void,
    ) -> *mut c_void;
    pub fn cd_relationship_description_set_inverse_relationship(
        relationship: *mut c_void,
        inverse_relationship: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_relationship_description_get_min_count(relationship: *mut c_void) -> u64;
    pub fn cd_relationship_description_set_min_count(
        relationship: *mut c_void,
        min_count: u64,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_relationship_description_get_max_count(relationship: *mut c_void) -> u64;
    pub fn cd_relationship_description_set_max_count(
        relationship: *mut c_void,
        max_count: u64,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_relationship_description_get_delete_rule(relationship: *mut c_void) -> u64;
    pub fn cd_relationship_description_set_delete_rule(
        relationship: *mut c_void,
        delete_rule: u64,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cd_persistent_store_coordinator_new(
        model: *mut c_void,
        out_coordinator: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_store_coordinator_add_persistent_store(
        coordinator: *mut c_void,
        store_type: *const c_char,
        configuration: *const c_char,
        url: *const c_char,
        options_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cd_persistent_container_new(
        name: *const c_char,
        model: *mut c_void,
        out_container: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_container_load_persistent_stores(
        container: *mut c_void,
        timeout_seconds: i32,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_persistent_container_view_context(container: *mut c_void) -> *mut c_void;
    pub fn cd_persistent_container_new_background_context(container: *mut c_void) -> *mut c_void;

    pub fn cd_managed_object_context_new(
        concurrency_type: i32,
        out_context: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_managed_object_context_set_persistent_store_coordinator(
        context: *mut c_void,
        coordinator: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_managed_object_context_has_changes(context: *mut c_void) -> i32;
    pub fn cd_managed_object_context_save(context: *mut c_void, out_error: *mut *mut c_char)
        -> i32;
    pub fn cd_managed_object_context_insert_object(
        context: *mut c_void,
        object: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_managed_object_context_delete_object(context: *mut c_void, object: *mut c_void);
    pub fn cd_managed_object_context_perform(
        context: *mut c_void,
        callback: VoidCallback,
        refcon: *mut c_void,
    );
    pub fn cd_managed_object_context_perform_and_wait(
        context: *mut c_void,
        callback: VoidCallback,
        refcon: *mut c_void,
    );
    pub fn cd_managed_object_context_execute_fetch_request(
        context: *mut c_void,
        request: *mut c_void,
        out_array: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cd_managed_object_new(
        entity: *mut c_void,
        context: *mut c_void,
        out_object: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_managed_object_entity(object: *mut c_void) -> *mut c_void;
    pub fn cd_managed_object_set_value_json(
        object: *mut c_void,
        key: *const c_char,
        value_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_managed_object_get_value_json(
        object: *mut c_void,
        key: *const c_char,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cd_fetch_request_new(
        entity_name: *const c_char,
        out_request: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_fetch_request_set_predicate(request: *mut c_void, predicate: *mut c_void);
    pub fn cd_fetch_request_set_sort_descriptors_json(
        request: *mut c_void,
        descriptors_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cd_fetch_request_set_fetch_limit(request: *mut c_void, fetch_limit: u64);
    pub fn cd_fetch_request_set_fetch_offset(request: *mut c_void, fetch_offset: u64);

    pub fn cd_predicate_new_with_format(
        format: *const c_char,
        arguments_json: *const c_char,
        out_predicate: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
}

pub type VoidCallback = unsafe extern "C" fn(refcon: *mut c_void);

#[cfg(feature = "async")]
#[path = "ffi/async_api.rs"]
pub mod async_api;

#[path = "ffi/batch_operation.rs"]
mod batch_operation;
#[path = "ffi/cloudkit_mirroring.rs"]
mod cloudkit_mirroring;
#[path = "ffi/constants.rs"]
mod constants;
#[path = "ffi/custom_store.rs"]
mod custom_store;
#[path = "ffi/entity_description.rs"]
mod entity_description;
#[path = "ffi/fetch_request.rs"]
mod fetch_request;
#[path = "ffi/fetched_results_controller.rs"]
mod fetched_results_controller;
#[path = "ffi/history.rs"]
mod history;
#[path = "ffi/managed_object.rs"]
mod managed_object;
#[path = "ffi/managed_object_context.rs"]
mod managed_object_context;
#[path = "ffi/merge_policy.rs"]
mod merge_policy;
#[path = "ffi/migration.rs"]
mod migration;
#[path = "ffi/migration_support.rs"]
mod migration_support;
#[path = "ffi/model_metadata.rs"]
mod model_metadata;
#[path = "ffi/ns_predicate.rs"]
mod ns_predicate;
#[path = "ffi/persistent_container.rs"]
mod persistent_container;
#[path = "ffi/persistent_store_coordinator.rs"]
mod persistent_store_coordinator;
#[path = "ffi/persistent_store_request.rs"]
mod persistent_store_request;
#[path = "ffi/query_generation.rs"]
mod query_generation;
#[path = "ffi/relationship_description.rs"]
mod relationship_description;
#[path = "ffi/validation.rs"]
mod validation;

pub use batch_operation::*;
pub use cloudkit_mirroring::*;
pub use constants::*;
pub use custom_store::*;
pub use entity_description::*;
pub use fetch_request::*;
pub use fetched_results_controller::*;
pub use history::*;
pub use managed_object::*;
pub use managed_object_context::*;
pub use merge_policy::*;
pub use migration::*;
pub use migration_support::*;
pub use model_metadata::*;
pub use ns_predicate::*;
pub use persistent_container::*;
pub use persistent_store_coordinator::*;
pub use persistent_store_request::*;
pub use query_generation::*;
pub use relationship_description::*;
pub use validation::*;

pub mod status {
    pub const OK: i32 = 0;
    pub const INVALID_ARGUMENT: i32 = -1;
    pub const FAILURE: i32 = -2;
    pub const TIMED_OUT: i32 = -3;
}
