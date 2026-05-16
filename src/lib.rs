#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::cargo_common_metadata,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::doc_markdown,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    clippy::redundant_pub_crate,
    clippy::ref_option,
    clippy::return_self_not_must_use,
    clippy::struct_field_names,
    clippy::type_complexity,
    clippy::use_self
)]

pub mod batch_operation;
pub mod cloudkit_mirroring;
pub mod context;
pub mod entity_description;
pub mod error;
pub mod fetch_request;
pub mod fetched_results_controller;
pub mod ffi;
pub mod history;
pub mod managed_object;
pub mod managed_object_context;
pub mod merge_policy;
pub mod migration;
pub mod model;
pub mod ns_predicate;
pub mod persistent_container;
pub mod persistent_store_coordinator;
mod private;
pub mod query;
pub mod relationship_description;
pub mod schema;
pub mod store;
pub mod validation;
pub mod value;

pub use batch_operation::{
    BatchDeleteRequestResultType, BatchInsertRequestResultType, BatchUpdateRequestResultType,
    NSBatchDeleteRequest, NSBatchDeleteResult, NSBatchInsertRequest, NSBatchInsertResult,
    NSBatchUpdateRequest, NSBatchUpdateResult,
};
pub use cloudkit_mirroring::{
    event_notification_names, event_user_info_keys, CloudKitDatabaseScope,
    CloudKitSchemaInitializationOptions, NSPersistentCloudKitContainer,
    NSPersistentCloudKitContainerEvent, NSPersistentCloudKitContainerEventRequest,
    NSPersistentCloudKitContainerEventResult, NSPersistentCloudKitContainerEventResultType,
    NSPersistentCloudKitContainerEventType, NSPersistentCloudKitContainerOptions,
};
pub use context::{NSManagedObject, NSManagedObjectContext, NSManagedObjectContextConcurrencyType};
pub use error::{CoreDataError, COREDATA_BRIDGE_ERROR_DOMAIN};
pub use fetch_request::FetchRequestResultType;
pub use fetched_results_controller::{
    FetchedResultsIndexPath, NSFetchedResultsChangeType, NSFetchedResultsController,
    NSFetchedResultsSectionInfo,
};
pub use history::{
    NSPersistentHistoryChange, NSPersistentHistoryChangeRequest, NSPersistentHistoryResult,
    NSPersistentHistoryToken, NSPersistentHistoryTransaction, PersistentHistoryChangeType,
    PersistentHistoryResultType,
};
pub use managed_object::NSManagedObjectID;
pub use merge_policy::{MergePolicyType, NSMergePolicy};
pub use migration::{migration_expression_keys, NSMappingModel, NSMigrationManager};
pub use model::NSManagedObjectModel;
pub use persistent_container::{option_keys, NSPersistentStoreDescription};
pub use persistent_store_coordinator::NSPersistentStore;
pub use query::{NSFetchRequest, NSPredicate, SortDescriptor};
pub use schema::{
    AttributeType, DeleteRule, NSAttributeDescription, NSEntityDescription,
    NSRelationshipDescription,
};
pub use store::{
    store_types, NSPersistentContainer, NSPersistentStoreCoordinator, PersistentStoreOptions,
};
pub use validation::{validation_error_codes, ValidationRule};
pub use value::Value;

/// Common imports.
pub mod prelude {
    pub use crate::batch_operation::{
        BatchDeleteRequestResultType, BatchInsertRequestResultType,
        BatchUpdateRequestResultType, NSBatchDeleteRequest, NSBatchDeleteResult,
        NSBatchInsertRequest, NSBatchInsertResult, NSBatchUpdateRequest, NSBatchUpdateResult,
    };
    pub use crate::cloudkit_mirroring::{
        event_notification_names, event_user_info_keys, CloudKitDatabaseScope,
        CloudKitSchemaInitializationOptions, NSPersistentCloudKitContainer,
        NSPersistentCloudKitContainerEvent, NSPersistentCloudKitContainerEventRequest,
        NSPersistentCloudKitContainerEventResult, NSPersistentCloudKitContainerEventResultType,
        NSPersistentCloudKitContainerEventType, NSPersistentCloudKitContainerOptions,
    };
    pub use crate::context::{
        NSManagedObject, NSManagedObjectContext, NSManagedObjectContextConcurrencyType,
    };
    pub use crate::error::{CoreDataError, COREDATA_BRIDGE_ERROR_DOMAIN};
    pub use crate::fetch_request::FetchRequestResultType;
    pub use crate::fetched_results_controller::{
        FetchedResultsIndexPath, NSFetchedResultsChangeType, NSFetchedResultsController,
        NSFetchedResultsSectionInfo,
    };
    pub use crate::history::{
        NSPersistentHistoryChange, NSPersistentHistoryChangeRequest, NSPersistentHistoryResult,
        NSPersistentHistoryToken, NSPersistentHistoryTransaction, PersistentHistoryChangeType,
        PersistentHistoryResultType,
    };
    pub use crate::managed_object::NSManagedObjectID;
    pub use crate::merge_policy::{MergePolicyType, NSMergePolicy};
    pub use crate::migration::{migration_expression_keys, NSMappingModel, NSMigrationManager};
    pub use crate::model::NSManagedObjectModel;
    pub use crate::persistent_container::{option_keys, NSPersistentStoreDescription};
    pub use crate::persistent_store_coordinator::NSPersistentStore;
    pub use crate::query::{NSFetchRequest, NSPredicate, SortDescriptor};
    pub use crate::schema::{
        AttributeType, DeleteRule, NSAttributeDescription, NSEntityDescription,
        NSRelationshipDescription,
    };
    pub use crate::store::{
        store_types, NSPersistentContainer, NSPersistentStoreCoordinator, PersistentStoreOptions,
    };
    pub use crate::validation::{validation_error_codes, ValidationRule};
    pub use crate::value::Value;
}
