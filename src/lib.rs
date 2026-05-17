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
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub mod async_api;
pub mod cloudkit_mirroring;
pub mod constants;
pub mod context;
pub mod custom_store;
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
pub mod migration_support;
pub mod model;
pub mod model_metadata;
pub mod ns_predicate;
pub mod persistent_container;
pub mod persistent_store_coordinator;
pub mod persistent_store_request;
mod private;
pub mod query;
pub mod query_generation;
pub mod relationship_description;
pub mod schema;
pub mod spotlight;
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
pub use constants::{
    context_notification_names, context_user_info_keys, coredata_version_number, error_domains,
    error_user_info_keys, persistent_store_metadata_keys, persistent_store_notification_names,
    persistent_store_option_keys, persistent_store_user_info_keys,
};
pub use context::{NSManagedObject, NSManagedObjectContext, NSManagedObjectContextConcurrencyType};
pub use custom_store::{
    NSAtomicStore, NSAtomicStoreCacheNode, NSIncrementalStore, NSIncrementalStoreNode,
};
pub use error::{CoreDataError, COREDATA_BRIDGE_ERROR_DOMAIN};
pub use fetch_request::FetchRequestResultType;
pub use fetched_results_controller::{
    FetchedResultsIndexPath, NSFetchedResultsChangeType, NSFetchedResultsController,
    NSFetchedResultsControllerDelegate, NSFetchedResultsSectionInfo,
};
pub use history::{
    NSPersistentHistoryChange, NSPersistentHistoryChangeRequest, NSPersistentHistoryResult,
    NSPersistentHistoryToken, NSPersistentHistoryTransaction, PersistentHistoryChangeType,
    PersistentHistoryResultType,
};
pub use managed_object::{NSManagedObjectID, NSSnapshotEventType};
pub use merge_policy::{MergePolicyType, NSConstraintConflict, NSMergeConflict, NSMergePolicy};
pub use migration::{migration_expression_keys, NSMappingModel, NSMigrationManager};
pub use migration_support::{
    NSCustomMigrationStage, NSEntityMapping, NSEntityMappingType, NSEntityMigrationPolicy,
    NSLightweightMigrationStage, NSManagedObjectModelReference, NSMigrationStage,
    NSPropertyMapping, NSStagedMigrationManager,
};
pub use model::NSManagedObjectModel;
pub use model_metadata::{
    NSCompositeAttributeDescription, NSDerivedAttributeDescription, NSExpressionDescription,
    NSFetchIndexDescription, NSFetchIndexElementDescription, NSFetchIndexElementType,
    NSFetchedPropertyDescription, NSPropertyDescription,
};
pub use persistent_container::{option_keys, NSPersistentStoreDescription};
pub use persistent_store_coordinator::NSPersistentStore;
pub use persistent_store_request::{
    NSAsynchronousFetchRequest, NSAsynchronousFetchResult, NSFetchRequestExpression,
    NSFetchRequestResult, NSPersistentStoreAsynchronousResult, NSPersistentStoreRequest,
    NSPersistentStoreRequestType, NSPersistentStoreResult, NSSaveChangesRequest,
};
pub use query::{NSFetchRequest, NSPredicate, SortDescriptor};
pub use query_generation::NSQueryGenerationToken;
pub use schema::{
    AttributeType, DeleteRule, NSAttributeDescription, NSEntityDescription,
    NSRelationshipDescription,
};
pub use spotlight::NSCoreDataCoreSpotlightDelegate;
pub use store::{
    store_types, NSPersistentContainer, NSPersistentStoreCoordinator, PersistentStoreOptions,
};
pub use validation::{validation_error_codes, ValidationRule};
pub use value::Value;

/// Common imports.
pub mod prelude {
    pub use crate::batch_operation::{
        BatchDeleteRequestResultType, BatchInsertRequestResultType, BatchUpdateRequestResultType,
        NSBatchDeleteRequest, NSBatchDeleteResult, NSBatchInsertRequest, NSBatchInsertResult,
        NSBatchUpdateRequest, NSBatchUpdateResult,
    };
    pub use crate::cloudkit_mirroring::{
        event_notification_names, event_user_info_keys, CloudKitDatabaseScope,
        CloudKitSchemaInitializationOptions, NSPersistentCloudKitContainer,
        NSPersistentCloudKitContainerEvent, NSPersistentCloudKitContainerEventRequest,
        NSPersistentCloudKitContainerEventResult, NSPersistentCloudKitContainerEventResultType,
        NSPersistentCloudKitContainerEventType, NSPersistentCloudKitContainerOptions,
    };
    pub use crate::constants::{
        context_notification_names, context_user_info_keys, coredata_version_number, error_domains,
        error_user_info_keys, persistent_store_metadata_keys, persistent_store_notification_names,
        persistent_store_option_keys, persistent_store_user_info_keys,
    };
    pub use crate::context::{
        NSManagedObject, NSManagedObjectContext, NSManagedObjectContextConcurrencyType,
    };
    pub use crate::custom_store::{
        NSAtomicStore, NSAtomicStoreCacheNode, NSIncrementalStore, NSIncrementalStoreNode,
    };
    pub use crate::error::{CoreDataError, COREDATA_BRIDGE_ERROR_DOMAIN};
    pub use crate::fetch_request::FetchRequestResultType;
    pub use crate::fetched_results_controller::{
        FetchedResultsIndexPath, NSFetchedResultsChangeType, NSFetchedResultsController,
        NSFetchedResultsControllerDelegate, NSFetchedResultsSectionInfo,
    };
    pub use crate::history::{
        NSPersistentHistoryChange, NSPersistentHistoryChangeRequest, NSPersistentHistoryResult,
        NSPersistentHistoryToken, NSPersistentHistoryTransaction, PersistentHistoryChangeType,
        PersistentHistoryResultType,
    };
    pub use crate::managed_object::{NSManagedObjectID, NSSnapshotEventType};
    pub use crate::merge_policy::{
        MergePolicyType, NSConstraintConflict, NSMergeConflict, NSMergePolicy,
    };
    pub use crate::migration::{migration_expression_keys, NSMappingModel, NSMigrationManager};
    pub use crate::migration_support::{
        NSCustomMigrationStage, NSEntityMapping, NSEntityMappingType, NSEntityMigrationPolicy,
        NSLightweightMigrationStage, NSManagedObjectModelReference, NSMigrationStage,
        NSPropertyMapping, NSStagedMigrationManager,
    };
    pub use crate::model::NSManagedObjectModel;
    pub use crate::model_metadata::{
        NSCompositeAttributeDescription, NSDerivedAttributeDescription, NSExpressionDescription,
        NSFetchIndexDescription, NSFetchIndexElementDescription, NSFetchIndexElementType,
        NSFetchedPropertyDescription, NSPropertyDescription,
    };
    pub use crate::persistent_container::{option_keys, NSPersistentStoreDescription};
    pub use crate::persistent_store_coordinator::NSPersistentStore;
    pub use crate::persistent_store_request::{
        NSAsynchronousFetchRequest, NSAsynchronousFetchResult, NSFetchRequestExpression,
        NSFetchRequestResult, NSPersistentStoreAsynchronousResult, NSPersistentStoreRequest,
        NSPersistentStoreRequestType, NSPersistentStoreResult, NSSaveChangesRequest,
    };
    pub use crate::query::{NSFetchRequest, NSPredicate, SortDescriptor};
    pub use crate::query_generation::NSQueryGenerationToken;
    pub use crate::schema::{
        AttributeType, DeleteRule, NSAttributeDescription, NSEntityDescription,
        NSRelationshipDescription,
    };
    pub use crate::spotlight::NSCoreDataCoreSpotlightDelegate;
    pub use crate::store::{
        store_types, NSPersistentContainer, NSPersistentStoreCoordinator, PersistentStoreOptions,
    };
    pub use crate::validation::{validation_error_codes, ValidationRule};
    pub use crate::value::Value;
}
