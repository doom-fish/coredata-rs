# coredata-rs coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 180
VERIFIED: 161
GAPS: 0
EXEMPT: 19
COVERAGE_PCT: 100.0%

Audit notes:
- Scope is top-level CoreData.framework symbols only: interfaces, protocols, typedef enum/option types, exported constants, and C functions (none found).
- Obj-C category/extension declarations on existing classes were excluded because they do not introduce new top-level symbols.
- Duplicate exports were deduplicated by symbol name, and `NSPersistentStoreFileProtectionKey` was excluded because it is `API_UNAVAILABLE(macos)`.
- A symbol is marked VERIFIED when the crate exposes a public Rust wrapper or an explicit named helper for that SDK symbol; member-level completeness is tracked separately in `COVERAGE.md`.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `NSAttributeDescription` | Interface | `NSAttributeDescription.h` | `schema::NSAttributeDescription` |
| `NSAttributeType` | Type | `NSAttributeDescription.h` | `schema::AttributeType` |
| `NSBatchDeleteRequest` | Interface | `NSBatchDeleteRequest.h` | `batch_operation::NSBatchDeleteRequest` |
| `NSBatchDeleteRequestResultType` | Type | `NSPersistentStoreResult.h` | `batch_operation::BatchDeleteRequestResultType` |
| `NSBatchDeleteResult` | Interface | `NSPersistentStoreResult.h` | `batch_operation::NSBatchDeleteResult` |
| `NSBatchInsertRequest` | Interface | `NSBatchInsertRequest.h` | `batch_operation::NSBatchInsertRequest` |
| `NSBatchInsertRequestResultType` | Type | `NSPersistentStoreResult.h` | `batch_operation::BatchInsertRequestResultType` |
| `NSBatchInsertResult` | Interface | `NSPersistentStoreResult.h` | `batch_operation::NSBatchInsertResult` |
| `NSBinaryStoreType` | Constant | `NSPersistentStoreCoordinator.h` | `store::store_types::BINARY` |
| `NSDeleteRule` | Type | `NSRelationshipDescription.h` | `schema::DeleteRule` |
| `NSEntityDescription` | Interface | `NSEntityDescription.h` | `schema::NSEntityDescription` |
| `NSFetchRequest` | Interface | `NSFetchRequest.h` | `query::NSFetchRequest + fetch_request extensions` |
| `NSFetchRequestResultType` | Type | `NSFetchRequest.h` | `fetch_request::FetchRequestResultType` |
| `NSInMemoryStoreType` | Constant | `NSPersistentStoreCoordinator.h` | `store::store_types::IN_MEMORY` |
| `NSInferMappingModelAutomaticallyOption` | Constant | `NSPersistentStoreCoordinator.h` | `persistent_container::option_keys::INFER_MAPPING_MODEL_AUTOMATICALLY / NSPersistentStoreDescription::set_should_infer_mapping_model_automatically` |
| `NSManagedObject` | Interface | `NSManagedObject.h` | `context::NSManagedObject + managed_object extensions` |
| `NSManagedObjectContext` | Interface | `NSManagedObjectContext.h` | `context::NSManagedObjectContext + managed_object_context extensions` |
| `NSManagedObjectContextConcurrencyType` | Type | `NSManagedObjectContext.h` | `context::NSManagedObjectContextConcurrencyType` |
| `NSManagedObjectID` | Interface | `NSManagedObjectID.h` | `managed_object::NSManagedObjectID` |
| `NSManagedObjectModel` | Interface | `NSManagedObjectModel.h` | `model::NSManagedObjectModel` |
| `NSMigratePersistentStoresAutomaticallyOption` | Constant | `NSPersistentStoreCoordinator.h` | `persistent_container::option_keys::MIGRATE_PERSISTENT_STORES_AUTOMATICALLY / NSPersistentStoreDescription::set_should_migrate_automatically` |
| `NSPersistentCloudKitContainer` | Interface | `NSPersistentCloudKitContainer.h` | `cloudkit_mirroring::NSPersistentCloudKitContainer` |
| `NSPersistentCloudKitContainerOptions` | Interface | `NSPersistentCloudKitContainerOptions.h` | `cloudkit_mirroring::NSPersistentCloudKitContainerOptions` |
| `NSPersistentCloudKitContainerSchemaInitializationOptions` | Type | `NSPersistentCloudKitContainer.h` | `cloudkit_mirroring::CloudKitSchemaInitializationOptions` |
| `NSPersistentContainer` | Interface | `NSPersistentContainer.h` | `store::NSPersistentContainer` |
| `NSPersistentHistoryChange` | Interface | `NSPersistentHistoryChange.h` | `history::NSPersistentHistoryChange` |
| `NSPersistentHistoryChangeRequest` | Interface | `NSPersistentHistoryChangeRequest.h` | `history::NSPersistentHistoryChangeRequest` |
| `NSPersistentHistoryChangeType` | Type | `NSPersistentHistoryChange.h` | `history::PersistentHistoryChangeType` |
| `NSPersistentHistoryResult` | Interface | `NSPersistentStoreResult.h` | `history::NSPersistentHistoryResult` |
| `NSPersistentHistoryResultType` | Type | `NSPersistentStoreResult.h` | `history::PersistentHistoryResultType` |
| `NSPersistentHistoryToken` | Interface | `NSPersistentHistoryToken.h` | `history::NSPersistentHistoryToken` |
| `NSPersistentHistoryTrackingKey` | Constant | `NSPersistentStoreCoordinator.h` | `persistent_container::option_keys::PERSISTENT_HISTORY_TRACKING` |
| `NSPersistentHistoryTransaction` | Interface | `NSPersistentHistoryTransaction.h` | `history::NSPersistentHistoryTransaction` |
| `NSPersistentStore` | Interface | `NSPersistentStore.h` | `persistent_store_coordinator::NSPersistentStore` |
| `NSPersistentStoreCoordinator` | Interface | `NSPersistentStoreCoordinator.h` | `store::NSPersistentStoreCoordinator + persistent_store_coordinator extensions` |
| `NSPersistentStoreDescription` | Interface | `NSPersistentStoreDescription.h` | `persistent_container::NSPersistentStoreDescription` |
| `NSPersistentStoreRemoteChangeNotificationPostOptionKey` | Constant | `NSPersistentStoreCoordinator.h` | `persistent_container::option_keys::REMOTE_CHANGE_NOTIFICATION_POST` |
| `NSPersistentStoreTimeoutOption` | Constant | `NSPersistentStoreCoordinator.h` | `NSPersistentStoreDescription::{timeout,set_timeout}` |
| `NSReadOnlyPersistentStoreOption` | Constant | `NSPersistentStoreCoordinator.h` | `persistent_container::option_keys::READ_ONLY / NSPersistentStoreDescription::set_read_only` |
| `NSRelationshipDescription` | Interface | `NSRelationshipDescription.h` | `schema::NSRelationshipDescription` |
| `NSSQLitePragmasOption` | Constant | `NSPersistentStoreCoordinator.h` | `NSPersistentStoreDescription::{sqlite_pragmas,set_sqlite_pragma}` |
| `NSSQLiteStoreType` | Constant | `NSPersistentStoreCoordinator.h` | `store::store_types::SQLITE` |
| `NSBatchUpdateRequest` | Interface | `NSBatchUpdateRequest.h` | `batch_operation::NSBatchUpdateRequest` |
| `NSBatchUpdateRequestResultType` | Type | `NSPersistentStoreResult.h` | `batch_operation::BatchUpdateRequestResultType` |
| `NSBatchUpdateResult` | Interface | `NSPersistentStoreResult.h` | `batch_operation::NSBatchUpdateResult` |
| `NSErrorMergePolicy` | Constant | `NSMergePolicy.h` | `merge_policy::NSMergePolicy::error_policy` |
| `NSFetchedResultsChangeType` | Type | `NSFetchedResultsController.h` | `fetched_results_controller::NSFetchedResultsChangeType` |
| `NSFetchedResultsController` | Interface | `NSFetchedResultsController.h` | `fetched_results_controller::NSFetchedResultsController` |
| `NSFetchedResultsSectionInfo` | Protocol | `NSFetchedResultsController.h` | `fetched_results_controller::NSFetchedResultsSectionInfo` |
| `NSMappingModel` | Interface | `NSMappingModel.h` | `migration::NSMappingModel` |
| `NSMergeByPropertyObjectTrumpMergePolicy` | Constant | `NSMergePolicy.h` | `merge_policy::NSMergePolicy::merge_by_property_object_trump_policy` |
| `NSMergeByPropertyStoreTrumpMergePolicy` | Constant | `NSMergePolicy.h` | `merge_policy::NSMergePolicy::merge_by_property_store_trump_policy` |
| `NSMergePolicy` | Interface | `NSMergePolicy.h` | `merge_policy::NSMergePolicy` |
| `NSMergePolicyType` | Type | `NSMergePolicy.h` | `merge_policy::MergePolicyType` |
| `NSMigrationDestinationObjectKey` | Constant | `NSEntityMigrationPolicy.h` | `migration::migration_expression_keys::DESTINATION_OBJECT` |
| `NSMigrationEntityMappingKey` | Constant | `NSEntityMigrationPolicy.h` | `migration::migration_expression_keys::ENTITY_MAPPING` |
| `NSMigrationEntityPolicyKey` | Constant | `NSEntityMigrationPolicy.h` | `migration::migration_expression_keys::ENTITY_POLICY` |
| `NSMigrationManager` | Interface | `NSMigrationManager.h` | `migration::NSMigrationManager` |
| `NSMigrationManagerKey` | Constant | `NSEntityMigrationPolicy.h` | `migration::migration_expression_keys::MANAGER` |
| `NSMigrationPropertyMappingKey` | Constant | `NSEntityMigrationPolicy.h` | `migration::migration_expression_keys::PROPERTY_MAPPING` |
| `NSMigrationSourceObjectKey` | Constant | `NSEntityMigrationPolicy.h` | `migration::migration_expression_keys::SOURCE_OBJECT` |
| `NSOverwriteMergePolicy` | Constant | `NSMergePolicy.h` | `merge_policy::NSMergePolicy::overwrite_policy` |
| `NSPersistentCloudKitContainerEvent` | Interface | `NSPersistentCloudKitContainerEvent.h` | `cloudkit_mirroring::NSPersistentCloudKitContainerEvent` |
| `NSPersistentCloudKitContainerEventChangedNotification` | Constant | `NSPersistentCloudKitContainerEvent.h` | `cloudkit_mirroring::event_notification_names::CHANGED` |
| `NSPersistentCloudKitContainerEventRequest` | Interface | `NSPersistentCloudKitContainerEventRequest.h` | `cloudkit_mirroring::NSPersistentCloudKitContainerEventRequest` |
| `NSPersistentCloudKitContainerEventResult` | Interface | `NSPersistentStoreResult.h` | `cloudkit_mirroring::NSPersistentCloudKitContainerEventResult` |
| `NSPersistentCloudKitContainerEventResultType` | Type | `NSPersistentStoreResult.h` | `cloudkit_mirroring::NSPersistentCloudKitContainerEventResultType` |
| `NSPersistentCloudKitContainerEventType` | Type | `NSPersistentCloudKitContainerEvent.h` | `cloudkit_mirroring::NSPersistentCloudKitContainerEventType` |
| `NSPersistentCloudKitContainerEventUserInfoKey` | Constant | `NSPersistentCloudKitContainerEvent.h` | `cloudkit_mirroring::event_user_info_keys::EVENT` |
| `NSRollbackMergePolicy` | Constant | `NSMergePolicy.h` | `merge_policy::NSMergePolicy::rollback_policy` |


## 🟢 VERIFIED (v0.2.2 additions)
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `NSAddedPersistentStoresKey` | Constant | `NSPersistentStoreCoordinator.h` | `constants::persistent_store_user_info_keys::ADDED_PERSISTENT_STORES` |
| `NSAffectedObjectsErrorKey` | Constant | `CoreDataErrors.h` | `constants::error_user_info_keys::AFFECTED_OBJECTS` |
| `NSAffectedStoresErrorKey` | Constant | `CoreDataErrors.h` | `constants::error_user_info_keys::AFFECTED_STORES` |
| `NSAsynchronousFetchRequest` | Interface | `NSFetchRequest.h` | `persistent_store_request::NSAsynchronousFetchRequest` |
| `NSAsynchronousFetchResult` | Interface | `NSPersistentStoreResult.h` | `persistent_store_request::NSAsynchronousFetchResult` |
| `NSAtomicStore` | Interface | `NSAtomicStore.h` | `custom_store::NSAtomicStore` |
| `NSAtomicStoreCacheNode` | Interface | `NSAtomicStoreCacheNode.h` | `custom_store::NSAtomicStoreCacheNode` |
| `NSBinaryStoreInsecureDecodingCompatibilityOption` | Constant | `NSPersistentStoreCoordinator.h` | `constants::persistent_store_option_keys::BINARY_STORE_INSECURE_DECODING_COMPATIBILITY` |
| `NSBinaryStoreSecureDecodingClasses` | Constant | `NSPersistentStoreCoordinator.h` | `constants::persistent_store_option_keys::BINARY_STORE_SECURE_DECODING_CLASSES` |
| `NSCompositeAttributeDescription` | Interface | `NSCompositeAttributeDescription.h` | `model_metadata::NSCompositeAttributeDescription` |
| `NSConstraintConflict` | Interface | `NSMergePolicy.h` | `merge_policy::NSConstraintConflict` |
| `NSCoreDataCoreSpotlightDelegate` | Interface | `NSCoreDataCoreSpotlightDelegate.h` | `spotlight::NSCoreDataCoreSpotlightDelegate` |
| `NSCoreDataCoreSpotlightDelegateIndexDidUpdateNotification` | Constant | `NSCoreDataCoreSpotlightDelegate.h` | `constants::persistent_store_notification_names::CORE_SPOTLIGHT_INDEX_DID_UPDATE` |
| `NSCoreDataCoreSpotlightExporter` | Constant | `NSPersistentStoreCoordinator.h` | `constants::persistent_store_option_keys::CORE_SPOTLIGHT_EXPORTER` |
| `NSCoreDataVersionNumber` | Constant | `CoreDataDefines.h` | `constants::coredata_version_number` |
| `NSCustomMigrationStage` | Interface | `NSCustomMigrationStage.h` | `migration_support::NSCustomMigrationStage` |
| `NSDeletedObjectIDsKey` | Constant | `NSManagedObjectContext.h` | `constants::context_user_info_keys::DELETED_OBJECT_IDS` |
| `NSDeletedObjectsKey` | Constant | `NSManagedObjectContext.h` | `constants::context_user_info_keys::DELETED_OBJECTS` |
| `NSDerivedAttributeDescription` | Interface | `NSDerivedAttributeDescription.h` | `model_metadata::NSDerivedAttributeDescription` |
| `NSDetailedErrorsKey` | Constant | `CoreDataErrors.h` | `constants::error_user_info_keys::DETAILED_ERRORS` |
| `NSEntityMapping` | Interface | `NSEntityMapping.h` | `migration_support::NSEntityMapping` |
| `NSEntityMappingType` | Type | `NSEntityMapping.h` | `migration_support::NSEntityMappingType` |
| `NSEntityMigrationPolicy` | Interface | `NSEntityMigrationPolicy.h` | `migration_support::NSEntityMigrationPolicy` |
| `NSExpressionDescription` | Interface | `NSExpressionDescription.h` | `model_metadata::NSExpressionDescription` |
| `NSFetchIndexDescription` | Interface | `NSFetchIndexDescription.h` | `model_metadata::NSFetchIndexDescription` |
| `NSFetchIndexElementDescription` | Interface | `NSFetchIndexElementDescription.h` | `model_metadata::NSFetchIndexElementDescription` |
| `NSFetchIndexElementType` | Type | `NSFetchIndexElementDescription.h` | `model_metadata::NSFetchIndexElementType` |
| `NSFetchRequestExpression` | Interface | `NSFetchRequestExpression.h` | `persistent_store_request::NSFetchRequestExpression` |
| `NSFetchRequestResult` | Protocol | `NSFetchRequest.h` | `persistent_store_request::NSFetchRequestResult` |
| `NSFetchedPropertyDescription` | Interface | `NSFetchedPropertyDescription.h` | `model_metadata::NSFetchedPropertyDescription` |
| `NSFetchedResultsControllerDelegate` | Protocol | `NSFetchedResultsController.h` | `fetched_results_controller::NSFetchedResultsControllerDelegate` |
| `NSIgnorePersistentStoreVersioningOption` | Constant | `NSPersistentStoreCoordinator.h` | `constants::persistent_store_option_keys::IGNORE_VERSIONING` |
| `NSIncrementalStore` | Interface | `NSIncrementalStore.h` | `custom_store::NSIncrementalStore` |
| `NSIncrementalStoreNode` | Interface | `NSIncrementalStoreNode.h` | `custom_store::NSIncrementalStoreNode` |
| `NSInsertedObjectIDsKey` | Constant | `NSManagedObjectContext.h` | `constants::context_user_info_keys::INSERTED_OBJECT_IDS` |
| `NSInsertedObjectsKey` | Constant | `NSManagedObjectContext.h` | `constants::context_user_info_keys::INSERTED_OBJECTS` |
| `NSInvalidatedAllObjectsKey` | Constant | `NSManagedObjectContext.h` | `constants::context_user_info_keys::INVALIDATED_ALL_OBJECTS` |
| `NSInvalidatedObjectIDsKey` | Constant | `NSManagedObjectContext.h` | `constants::context_user_info_keys::INVALIDATED_OBJECT_IDS` |
| `NSInvalidatedObjectsKey` | Constant | `NSManagedObjectContext.h` | `constants::context_user_info_keys::INVALIDATED_OBJECTS` |
| `NSLightweightMigrationStage` | Interface | `NSLightweightMigrationStage.h` | `migration_support::NSLightweightMigrationStage` |
| `NSManagedObjectContextDidMergeChangesObjectIDsNotification` | Constant | `NSManagedObjectContext.h` | `constants::context_notification_names::DID_MERGE_CHANGES_OBJECT_IDS` |
| `NSManagedObjectContextDidSaveNotification` | Constant | `NSManagedObjectContext.h` | `constants::context_notification_names::DID_SAVE` |
| `NSManagedObjectContextDidSaveObjectIDsNotification` | Constant | `NSManagedObjectContext.h` | `constants::context_notification_names::DID_SAVE_OBJECT_IDS` |
| `NSManagedObjectContextObjectsDidChangeNotification` | Constant | `NSManagedObjectContext.h` | `constants::context_notification_names::OBJECTS_DID_CHANGE` |
| `NSManagedObjectContextQueryGenerationKey` | Constant | `NSManagedObjectContext.h` | `constants::context_user_info_keys::QUERY_GENERATION` |
| `NSManagedObjectContextWillSaveNotification` | Constant | `NSManagedObjectContext.h` | `constants::context_notification_names::WILL_SAVE` |
| `NSManagedObjectModelReference` | Interface | `NSManagedObjectModelReference.h` | `migration_support::NSManagedObjectModelReference` |
| `NSMergeConflict` | Interface | `NSMergePolicy.h` | `merge_policy::NSMergeConflict` |
| `NSMigrationStage` | Interface | `NSMigrationStage.h` | `migration_support::NSMigrationStage` |
| `NSPersistentHistoryTokenKey` | Constant | `NSPersistentStoreCoordinator.h` | `constants::persistent_store_user_info_keys::PERSISTENT_HISTORY_TOKEN` |
| `NSPersistentStoreAsynchronousResult` | Interface | `NSPersistentStoreResult.h` | `persistent_store_request::NSPersistentStoreAsynchronousResult` |
| `NSPersistentStoreConnectionPoolMaxSizeKey` | Constant | `NSPersistentStoreCoordinator.h` | `constants::persistent_store_metadata_keys::STORE_CONNECTION_POOL_MAX_SIZE` |
| `NSPersistentStoreCoordinatorStoresDidChangeNotification` | Constant | `NSPersistentStoreCoordinator.h` | `constants::persistent_store_notification_names::STORES_DID_CHANGE` |
| `NSPersistentStoreCoordinatorStoresWillChangeNotification` | Constant | `NSPersistentStoreCoordinator.h` | `constants::persistent_store_notification_names::STORES_WILL_CHANGE` |
| `NSPersistentStoreCoordinatorWillRemoveStoreNotification` | Constant | `NSPersistentStoreCoordinator.h` | `constants::persistent_store_notification_names::WILL_REMOVE_STORE` |
| `NSPersistentStoreDeferredLightweightMigrationOptionKey` | Constant | `NSPersistentStoreCoordinator.h` | `constants::persistent_store_option_keys::DEFERRED_LIGHTWEIGHT_MIGRATION` |
| `NSPersistentStoreForceDestroyOption` | Constant | `NSPersistentStoreCoordinator.h` | `constants::persistent_store_option_keys::FORCE_DESTROY` |
| `NSPersistentStoreModelVersionChecksumKey` | Constant | `NSPersistentStoreCoordinator.h` | `constants::persistent_store_metadata_keys::STORE_MODEL_VERSION_CHECKSUM` |
| `NSPersistentStoreOSCompatibility` | Constant | `NSPersistentStoreCoordinator.h` | `constants::persistent_store_metadata_keys::STORE_OS_COMPATIBILITY` |
| `NSPersistentStoreRemoteChangeNotification` | Constant | `NSPersistentStoreCoordinator.h` | `constants::persistent_store_notification_names::REMOTE_CHANGE` |
| `NSPersistentStoreRequest` | Interface | `NSPersistentStoreRequest.h` | `persistent_store_request::NSPersistentStoreRequest` |
| `NSPersistentStoreRequestType` | Type | `NSPersistentStoreRequest.h` | `persistent_store_request::NSPersistentStoreRequestType` |
| `NSPersistentStoreResult` | Interface | `NSPersistentStoreResult.h` | `persistent_store_request::NSPersistentStoreResult` |
| `NSPersistentStoreSaveConflictsErrorKey` | Constant | `CoreDataErrors.h` | `constants::error_user_info_keys::PERSISTENT_STORE_SAVE_CONFLICTS` |
| `NSPersistentStoreStagedMigrationManagerOptionKey` | Constant | `NSPersistentStoreCoordinator.h` | `constants::persistent_store_option_keys::STAGED_MIGRATION_MANAGER` |
| `NSPersistentStoreURLKey` | Constant | `NSPersistentStoreCoordinator.h` | `constants::persistent_store_user_info_keys::PERSISTENT_STORE_URL` |
| `NSPropertyDescription` | Interface | `NSPropertyDescription.h` | `model_metadata::NSPropertyDescription` |
| `NSPropertyMapping` | Interface | `NSPropertyMapping.h` | `migration_support::NSPropertyMapping` |
| `NSQueryGenerationToken` | Interface | `NSQueryGenerationToken.h` | `query_generation::NSQueryGenerationToken` |
| `NSRefreshedObjectIDsKey` | Constant | `NSManagedObjectContext.h` | `constants::context_user_info_keys::REFRESHED_OBJECT_IDS` |
| `NSRefreshedObjectsKey` | Constant | `NSManagedObjectContext.h` | `constants::context_user_info_keys::REFRESHED_OBJECTS` |
| `NSRemovedPersistentStoresKey` | Constant | `NSPersistentStoreCoordinator.h` | `constants::persistent_store_user_info_keys::REMOVED_PERSISTENT_STORES` |
| `NSSQLiteAnalyzeOption` | Constant | `NSPersistentStoreCoordinator.h` | `constants::persistent_store_option_keys::SQLITE_ANALYZE` |
| `NSSQLiteErrorDomain` | Constant | `CoreDataErrors.h` | `constants::error_domains::SQLITE` |
| `NSSQLiteManualVacuumOption` | Constant | `NSPersistentStoreCoordinator.h` | `constants::persistent_store_option_keys::SQLITE_MANUAL_VACUUM` |
| `NSSaveChangesRequest` | Interface | `NSSaveChangesRequest.h` | `persistent_store_request::NSSaveChangesRequest` |
| `NSSnapshotEventType` | Type | `NSManagedObject.h` | `managed_object::NSSnapshotEventType` |
| `NSStagedMigrationManager` | Interface | `NSStagedMigrationManager.h` | `migration_support::NSStagedMigrationManager` |
| `NSStoreModelVersionHashesKey` | Constant | `NSPersistentStoreCoordinator.h` | `constants::persistent_store_metadata_keys::STORE_MODEL_VERSION_HASHES` |
| `NSStoreModelVersionIdentifiersKey` | Constant | `NSPersistentStoreCoordinator.h` | `constants::persistent_store_metadata_keys::STORE_MODEL_VERSION_IDENTIFIERS` |
| `NSStoreTypeKey` | Constant | `NSPersistentStoreCoordinator.h` | `constants::persistent_store_metadata_keys::STORE_TYPE` |
| `NSStoreUUIDKey` | Constant | `NSPersistentStoreCoordinator.h` | `constants::persistent_store_metadata_keys::STORE_UUID` |
| `NSUUIDChangedPersistentStoresKey` | Constant | `NSPersistentStoreCoordinator.h` | `constants::persistent_store_user_info_keys::UUID_CHANGED_PERSISTENT_STORES` |
| `NSUpdatedObjectIDsKey` | Constant | `NSManagedObjectContext.h` | `constants::context_user_info_keys::UPDATED_OBJECT_IDS` |
| `NSUpdatedObjectsKey` | Constant | `NSManagedObjectContext.h` | `constants::context_user_info_keys::UPDATED_OBJECTS` |
| `NSValidateXMLStoreOption` | Constant | `NSPersistentStoreCoordinator.h` | `constants::persistent_store_option_keys::VALIDATE_XML_STORE` |
| `NSValidationKeyErrorKey` | Constant | `CoreDataErrors.h` | `constants::error_user_info_keys::VALIDATION_KEY` |
| `NSValidationObjectErrorKey` | Constant | `CoreDataErrors.h` | `constants::error_user_info_keys::VALIDATION_OBJECT` |
| `NSValidationPredicateErrorKey` | Constant | `CoreDataErrors.h` | `constants::error_user_info_keys::VALIDATION_PREDICATE` |
| `NSValidationValueErrorKey` | Constant | `CoreDataErrors.h` | `constants::error_user_info_keys::VALIDATION_VALUE` |
| `NSXMLStoreType` | Constant | `NSPersistentStoreCoordinator.h` | `store::store_types::XML` |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| _None_ | — | — | All non-exempt CoreData.framework top-level symbols are now wrapped or named by the crate. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `NSBinaryExternalRecordType` | Constant | `NSPersistentStoreCoordinator.h` | Legacy Spotlight external-record integration intentionally skipped. | `API_DEPRECATED(macosx(10.6,10.13))` |
| `NSEntityNameInPathKey` | Constant | `NSPersistentStoreCoordinator.h` | Legacy Spotlight external-record integration intentionally skipped. | `API_DEPRECATED(macosx(10.6,10.13))` |
| `NSExternalRecordExtensionOption` | Constant | `NSPersistentStoreCoordinator.h` | Legacy Spotlight external-record integration intentionally skipped. | `API_DEPRECATED(macosx(10.6,10.13))` |
| `NSExternalRecordsDirectoryOption` | Constant | `NSPersistentStoreCoordinator.h` | Legacy Spotlight external-record integration intentionally skipped. | `API_DEPRECATED(macosx(10.6,10.13))` |
| `NSExternalRecordsFileFormatOption` | Constant | `NSPersistentStoreCoordinator.h` | Legacy Spotlight external-record integration intentionally skipped. | `API_DEPRECATED(macosx(10.6,10.13))` |
| `NSModelPathKey` | Constant | `NSPersistentStoreCoordinator.h` | Legacy Spotlight external-record integration intentionally skipped. | `API_DEPRECATED(macosx(10.6,10.13))` |
| `NSObjectURIKey` | Constant | `NSPersistentStoreCoordinator.h` | Legacy Spotlight external-record integration intentionally skipped. | `API_DEPRECATED(macosx(10.6,10.13))` |
| `NSPersistentStoreDidImportUbiquitousContentChangesNotification` | Constant | `NSPersistentStoreCoordinator.h` | Legacy iCloud ubiquity notification intentionally skipped. | `API_DEPRECATED(macosx(10.7,10.12))` |
| `NSPersistentStoreRebuildFromUbiquitousContentOption` | Constant | `NSPersistentStoreCoordinator.h` | Legacy iCloud ubiquity option intentionally skipped. | `API_DEPRECATED(macosx(10.9,10.12))` |
| `NSPersistentStoreRemoveUbiquitousMetadataOption` | Constant | `NSPersistentStoreCoordinator.h` | Legacy iCloud ubiquity option intentionally skipped. | `API_DEPRECATED(macosx(10.9,10.12))` |
| `NSPersistentStoreUbiquitousContainerIdentifierKey` | Constant | `NSPersistentStoreCoordinator.h` | Legacy iCloud ubiquity option intentionally skipped. | `API_DEPRECATED(macosx(10.9,10.12))` |
| `NSPersistentStoreUbiquitousContentNameKey` | Constant | `NSPersistentStoreCoordinator.h` | Legacy iCloud ubiquity option intentionally skipped. | `API_DEPRECATED(macosx(10.7,10.12))` |
| `NSPersistentStoreUbiquitousContentURLKey` | Constant | `NSPersistentStoreCoordinator.h` | Legacy iCloud ubiquity option intentionally skipped. | `API_DEPRECATED(macosx(10.7,10.12))` |
| `NSPersistentStoreUbiquitousPeerTokenOption` | Constant | `NSPersistentStoreCoordinator.h` | Legacy iCloud ubiquity option intentionally skipped. | `API_DEPRECATED(macosx(10.9,10.12))` |
| `NSPersistentStoreUbiquitousTransitionType` | Type | `NSPersistentStoreCoordinator.h` | Legacy iCloud ubiquity enum intentionally skipped. | `API_DEPRECATED(macosx(10.9,10.12))` |
| `NSPersistentStoreUbiquitousTransitionTypeKey` | Constant | `NSPersistentStoreCoordinator.h` | Legacy iCloud ubiquity option intentionally skipped. | `API_DEPRECATED(macosx(10.9,10.12))` |
| `NSStorePathKey` | Constant | `NSPersistentStoreCoordinator.h` | Legacy Spotlight external-record integration intentionally skipped. | `API_DEPRECATED(macosx(10.6,10.13))` |
| `NSStoreUUIDInPathKey` | Constant | `NSPersistentStoreCoordinator.h` | Legacy Spotlight external-record integration intentionally skipped. | `API_DEPRECATED(macosx(10.6,10.13))` |
| `NSXMLExternalRecordType` | Constant | `NSPersistentStoreCoordinator.h` | Legacy Spotlight external-record integration intentionally skipped. | `API_DEPRECATED(macosx(10.6,10.13))` |
