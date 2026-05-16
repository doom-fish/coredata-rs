# coredata-rs coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 180
VERIFIED: 42
GAPS: 119
EXEMPT: 19
COVERAGE_PCT: 26.1%

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

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| `NSAddedPersistentStoresKey` | Constant | `NSPersistentStoreCoordinator.h` | Notification/userInfo constants are not wrapped. |
| `NSAffectedObjectsErrorKey` | Constant | `CoreDataErrors.h` | NSError user-info/domain constants are not typed by the crate. |
| `NSAffectedStoresErrorKey` | Constant | `CoreDataErrors.h` | NSError user-info/domain constants are not typed by the crate. |
| `NSAsynchronousFetchRequest` | Interface | `NSFetchRequest.h` | Async/base persistent-store request APIs are not wrapped. |
| `NSAsynchronousFetchResult` | Interface | `NSPersistentStoreResult.h` | Async/base persistent-store request APIs are not wrapped. |
| `NSAtomicStore` | Interface | `NSAtomicStore.h` | Custom/atomic store APIs are not wrapped. |
| `NSAtomicStoreCacheNode` | Interface | `NSAtomicStoreCacheNode.h` | Custom/atomic store APIs are not wrapped. |
| `NSBatchUpdateRequest` | Interface | `NSBatchUpdateRequest.h` | Batch delete/insert are wrapped, but batch update is not. |
| `NSBatchUpdateRequestResultType` | Type | `NSPersistentStoreResult.h` | Batch delete/insert are wrapped, but batch update is not. |
| `NSBatchUpdateResult` | Interface | `NSPersistentStoreResult.h` | Batch delete/insert are wrapped, but batch update is not. |
| `NSBinaryStoreInsecureDecodingCompatibilityOption` | Constant | `NSPersistentStoreCoordinator.h` | Metadata/option-key constant is not exposed as a named Rust constant. |
| `NSBinaryStoreSecureDecodingClasses` | Constant | `NSPersistentStoreCoordinator.h` | Metadata/option-key constant is not exposed as a named Rust constant. |
| `NSCompositeAttributeDescription` | Interface | `NSCompositeAttributeDescription.h` | Advanced model metadata APIs are not wrapped. |
| `NSConstraintConflict` | Interface | `NSMergePolicy.h` | Merge policy/conflict APIs are not wrapped. |
| `NSCoreDataCoreSpotlightDelegate` | Interface | `NSCoreDataCoreSpotlightDelegate.h` | Core Spotlight integration is not wrapped. |
| `NSCoreDataCoreSpotlightDelegateIndexDidUpdateNotification` | Constant | `NSCoreDataCoreSpotlightDelegate.h` | Core Spotlight integration is not wrapped. |
| `NSCoreDataCoreSpotlightExporter` | Constant | `NSPersistentStoreCoordinator.h` | Core Spotlight integration is not wrapped. |
| `NSCoreDataVersionNumber` | Constant | `CoreDataDefines.h` | Framework version constant is not exposed. |
| `NSCustomMigrationStage` | Interface | `NSCustomMigrationStage.h` | Migration/mapping APIs are not wrapped. |
| `NSDeletedObjectIDsKey` | Constant | `NSManagedObjectContext.h` | Notification/userInfo constants are not wrapped. |
| `NSDeletedObjectsKey` | Constant | `NSManagedObjectContext.h` | Notification/userInfo constants are not wrapped. |
| `NSDerivedAttributeDescription` | Interface | `NSDerivedAttributeDescription.h` | Advanced model metadata APIs are not wrapped. |
| `NSDetailedErrorsKey` | Constant | `CoreDataErrors.h` | NSError user-info/domain constants are not typed by the crate. |
| `NSEntityMapping` | Interface | `NSEntityMapping.h` | Migration/mapping APIs are not wrapped. |
| `NSEntityMappingType` | Type | `NSEntityMapping.h` | Migration/mapping APIs are not wrapped. |
| `NSEntityMigrationPolicy` | Interface | `NSEntityMigrationPolicy.h` | Migration/mapping APIs are not wrapped. |
| `NSErrorMergePolicy` | Constant | `NSMergePolicy.h` | Merge policy/conflict APIs are not wrapped. |
| `NSExpressionDescription` | Interface | `NSExpressionDescription.h` | Advanced model metadata APIs are not wrapped. |
| `NSFetchIndexDescription` | Interface | `NSFetchIndexDescription.h` | Advanced model metadata APIs are not wrapped. |
| `NSFetchIndexElementDescription` | Interface | `NSFetchIndexElementDescription.h` | Advanced model metadata APIs are not wrapped. |
| `NSFetchIndexElementType` | Type | `NSFetchIndexElementDescription.h` | Advanced model metadata APIs are not wrapped. |
| `NSFetchRequestExpression` | Interface | `NSFetchRequestExpression.h` | Async/base persistent-store request APIs are not wrapped. |
| `NSFetchRequestResult` | Protocol | `NSFetchRequest.h` | No public Rust analogue. |
| `NSFetchedPropertyDescription` | Interface | `NSFetchedPropertyDescription.h` | Advanced model metadata APIs are not wrapped. |
| `NSFetchedResultsChangeType` | Type | `NSFetchedResultsController.h` | Fetched-results-controller APIs are not wrapped. |
| `NSFetchedResultsController` | Interface | `NSFetchedResultsController.h` | Fetched-results-controller APIs are not wrapped. |
| `NSFetchedResultsControllerDelegate` | Protocol | `NSFetchedResultsController.h` | Fetched-results-controller APIs are not wrapped. |
| `NSFetchedResultsSectionInfo` | Protocol | `NSFetchedResultsController.h` | Fetched-results-controller APIs are not wrapped. |
| `NSIgnorePersistentStoreVersioningOption` | Constant | `NSPersistentStoreCoordinator.h` | No dedicated safe wrapper; only generic option passthrough exists (if any). |
| `NSIncrementalStore` | Interface | `NSIncrementalStore.h` | Custom/atomic store APIs are not wrapped. |
| `NSIncrementalStoreNode` | Interface | `NSIncrementalStoreNode.h` | Custom/atomic store APIs are not wrapped. |
| `NSInsertedObjectIDsKey` | Constant | `NSManagedObjectContext.h` | Notification/userInfo constants are not wrapped. |
| `NSInsertedObjectsKey` | Constant | `NSManagedObjectContext.h` | Notification/userInfo constants are not wrapped. |
| `NSInvalidatedAllObjectsKey` | Constant | `NSManagedObjectContext.h` | Notification/userInfo constants are not wrapped. |
| `NSInvalidatedObjectIDsKey` | Constant | `NSManagedObjectContext.h` | Notification/userInfo constants are not wrapped. |
| `NSInvalidatedObjectsKey` | Constant | `NSManagedObjectContext.h` | Notification/userInfo constants are not wrapped. |
| `NSLightweightMigrationStage` | Interface | `NSLightweightMigrationStage.h` | Migration/mapping APIs are not wrapped. |
| `NSManagedObjectContextDidMergeChangesObjectIDsNotification` | Constant | `NSManagedObjectContext.h` | Notification/userInfo constants are not wrapped. |
| `NSManagedObjectContextDidSaveNotification` | Constant | `NSManagedObjectContext.h` | Notification/userInfo constants are not wrapped. |
| `NSManagedObjectContextDidSaveObjectIDsNotification` | Constant | `NSManagedObjectContext.h` | Notification/userInfo constants are not wrapped. |
| `NSManagedObjectContextObjectsDidChangeNotification` | Constant | `NSManagedObjectContext.h` | Notification/userInfo constants are not wrapped. |
| `NSManagedObjectContextQueryGenerationKey` | Constant | `NSManagedObjectContext.h` | Query-generation APIs are not wrapped. |
| `NSManagedObjectContextWillSaveNotification` | Constant | `NSManagedObjectContext.h` | Notification/userInfo constants are not wrapped. |
| `NSManagedObjectModelReference` | Interface | `NSManagedObjectModelReference.h` | Migration/mapping APIs are not wrapped. |
| `NSMappingModel` | Interface | `NSMappingModel.h` | Migration/mapping APIs are not wrapped. |
| `NSMergeByPropertyObjectTrumpMergePolicy` | Constant | `NSMergePolicy.h` | Merge policy/conflict APIs are not wrapped. |
| `NSMergeByPropertyStoreTrumpMergePolicy` | Constant | `NSMergePolicy.h` | Merge policy/conflict APIs are not wrapped. |
| `NSMergeConflict` | Interface | `NSMergePolicy.h` | Merge policy/conflict APIs are not wrapped. |
| `NSMergePolicy` | Interface | `NSMergePolicy.h` | Merge policy/conflict APIs are not wrapped. |
| `NSMergePolicyType` | Type | `NSMergePolicy.h` | Merge policy/conflict APIs are not wrapped. |
| `NSMigrationDestinationObjectKey` | Constant | `NSEntityMigrationPolicy.h` | Migration helper constant is not wrapped. |
| `NSMigrationEntityMappingKey` | Constant | `NSEntityMigrationPolicy.h` | Migration helper constant is not wrapped. |
| `NSMigrationEntityPolicyKey` | Constant | `NSEntityMigrationPolicy.h` | Migration helper constant is not wrapped. |
| `NSMigrationManager` | Interface | `NSMigrationManager.h` | Migration/mapping APIs are not wrapped. |
| `NSMigrationManagerKey` | Constant | `NSEntityMigrationPolicy.h` | Migration helper constant is not wrapped. |
| `NSMigrationPropertyMappingKey` | Constant | `NSEntityMigrationPolicy.h` | Migration helper constant is not wrapped. |
| `NSMigrationSourceObjectKey` | Constant | `NSEntityMigrationPolicy.h` | Migration helper constant is not wrapped. |
| `NSMigrationStage` | Interface | `NSMigrationStage.h` | Migration/mapping APIs are not wrapped. |
| `NSOverwriteMergePolicy` | Constant | `NSMergePolicy.h` | Merge policy/conflict APIs are not wrapped. |
| `NSPersistentCloudKitContainerEvent` | Interface | `NSPersistentCloudKitContainerEvent.h` | CloudKit event observation/request APIs are not wrapped. |
| `NSPersistentCloudKitContainerEventChangedNotification` | Constant | `NSPersistentCloudKitContainerEvent.h` | CloudKit event observation/request APIs are not wrapped. |
| `NSPersistentCloudKitContainerEventRequest` | Interface | `NSPersistentCloudKitContainerEventRequest.h` | CloudKit event observation/request APIs are not wrapped. |
| `NSPersistentCloudKitContainerEventResult` | Interface | `NSPersistentStoreResult.h` | CloudKit event observation/request APIs are not wrapped. |
| `NSPersistentCloudKitContainerEventResultType` | Type | `NSPersistentStoreResult.h` | CloudKit event observation/request APIs are not wrapped. |
| `NSPersistentCloudKitContainerEventType` | Type | `NSPersistentCloudKitContainerEvent.h` | CloudKit event observation/request APIs are not wrapped. |
| `NSPersistentCloudKitContainerEventUserInfoKey` | Constant | `NSPersistentCloudKitContainerEvent.h` | CloudKit event observation/request APIs are not wrapped. |
| `NSPersistentHistoryTokenKey` | Constant | `NSPersistentStoreCoordinator.h` | Metadata/option-key constant is not exposed as a named Rust constant. |
| `NSPersistentStoreAsynchronousResult` | Interface | `NSPersistentStoreResult.h` | Async/base persistent-store request APIs are not wrapped. |
| `NSPersistentStoreConnectionPoolMaxSizeKey` | Constant | `NSPersistentStoreCoordinator.h` | Metadata/option-key constant is not exposed as a named Rust constant. |
| `NSPersistentStoreCoordinatorStoresDidChangeNotification` | Constant | `NSPersistentStoreCoordinator.h` | Notification/userInfo constants are not wrapped. |
| `NSPersistentStoreCoordinatorStoresWillChangeNotification` | Constant | `NSPersistentStoreCoordinator.h` | Notification/userInfo constants are not wrapped. |
| `NSPersistentStoreCoordinatorWillRemoveStoreNotification` | Constant | `NSPersistentStoreCoordinator.h` | Notification/userInfo constants are not wrapped. |
| `NSPersistentStoreDeferredLightweightMigrationOptionKey` | Constant | `NSPersistentStoreCoordinator.h` | No dedicated safe wrapper; only generic option passthrough exists (if any). |
| `NSPersistentStoreForceDestroyOption` | Constant | `NSPersistentStoreCoordinator.h` | No dedicated safe wrapper; only generic option passthrough exists (if any). |
| `NSPersistentStoreModelVersionChecksumKey` | Constant | `NSPersistentStoreCoordinator.h` | Metadata/option-key constant is not exposed as a named Rust constant. |
| `NSPersistentStoreOSCompatibility` | Constant | `NSPersistentStoreCoordinator.h` | Metadata/option-key constant is not exposed as a named Rust constant. |
| `NSPersistentStoreRemoteChangeNotification` | Constant | `NSPersistentStoreCoordinator.h` | Notification/userInfo constants are not wrapped. |
| `NSPersistentStoreRequest` | Interface | `NSPersistentStoreRequest.h` | Async/base persistent-store request APIs are not wrapped. |
| `NSPersistentStoreRequestType` | Type | `NSPersistentStoreRequest.h` | Async/base persistent-store request APIs are not wrapped. |
| `NSPersistentStoreResult` | Interface | `NSPersistentStoreResult.h` | Async/base persistent-store request APIs are not wrapped. |
| `NSPersistentStoreSaveConflictsErrorKey` | Constant | `CoreDataErrors.h` | NSError user-info/domain constants are not typed by the crate. |
| `NSPersistentStoreStagedMigrationManagerOptionKey` | Constant | `NSPersistentStoreCoordinator.h` | No dedicated safe wrapper; only generic option passthrough exists (if any). |
| `NSPersistentStoreURLKey` | Constant | `NSPersistentStoreCoordinator.h` | Metadata/option-key constant is not exposed as a named Rust constant. |
| `NSPropertyDescription` | Interface | `NSPropertyDescription.h` | Advanced model metadata APIs are not wrapped. |
| `NSPropertyMapping` | Interface | `NSPropertyMapping.h` | Migration/mapping APIs are not wrapped. |
| `NSQueryGenerationToken` | Interface | `NSQueryGenerationToken.h` | Query-generation APIs are not wrapped. |
| `NSRefreshedObjectIDsKey` | Constant | `NSManagedObjectContext.h` | Notification/userInfo constants are not wrapped. |
| `NSRefreshedObjectsKey` | Constant | `NSManagedObjectContext.h` | Notification/userInfo constants are not wrapped. |
| `NSRemovedPersistentStoresKey` | Constant | `NSPersistentStoreCoordinator.h` | Notification/userInfo constants are not wrapped. |
| `NSRollbackMergePolicy` | Constant | `NSMergePolicy.h` | Merge policy/conflict APIs are not wrapped. |
| `NSSQLiteAnalyzeOption` | Constant | `NSPersistentStoreCoordinator.h` | No dedicated safe wrapper; only generic option passthrough exists (if any). |
| `NSSQLiteErrorDomain` | Constant | `CoreDataErrors.h` | NSError user-info/domain constants are not typed by the crate. |
| `NSSQLiteManualVacuumOption` | Constant | `NSPersistentStoreCoordinator.h` | No dedicated safe wrapper; only generic option passthrough exists (if any). |
| `NSSaveChangesRequest` | Interface | `NSSaveChangesRequest.h` | No public Rust wrapper. |
| `NSSnapshotEventType` | Type | `NSManagedObject.h` | No public Rust analogue. |
| `NSStagedMigrationManager` | Interface | `NSStagedMigrationManager.h` | Migration/mapping APIs are not wrapped. |
| `NSStoreModelVersionHashesKey` | Constant | `NSPersistentStoreCoordinator.h` | Metadata/option-key constant is not exposed as a named Rust constant. |
| `NSStoreModelVersionIdentifiersKey` | Constant | `NSPersistentStoreCoordinator.h` | Metadata/option-key constant is not exposed as a named Rust constant. |
| `NSStoreTypeKey` | Constant | `NSPersistentStoreCoordinator.h` | Metadata/option-key constant is not exposed as a named Rust constant. |
| `NSStoreUUIDKey` | Constant | `NSPersistentStoreCoordinator.h` | Metadata/option-key constant is not exposed as a named Rust constant. |
| `NSUUIDChangedPersistentStoresKey` | Constant | `NSPersistentStoreCoordinator.h` | Notification/userInfo constants are not wrapped. |
| `NSUpdatedObjectIDsKey` | Constant | `NSManagedObjectContext.h` | Notification/userInfo constants are not wrapped. |
| `NSUpdatedObjectsKey` | Constant | `NSManagedObjectContext.h` | Notification/userInfo constants are not wrapped. |
| `NSValidateXMLStoreOption` | Constant | `NSPersistentStoreCoordinator.h` | XML-store helpers are not exposed by the crate. |
| `NSValidationKeyErrorKey` | Constant | `CoreDataErrors.h` | NSError user-info/domain constants are not typed by the crate. |
| `NSValidationObjectErrorKey` | Constant | `CoreDataErrors.h` | NSError user-info/domain constants are not typed by the crate. |
| `NSValidationPredicateErrorKey` | Constant | `CoreDataErrors.h` | NSError user-info/domain constants are not typed by the crate. |
| `NSValidationValueErrorKey` | Constant | `CoreDataErrors.h` | NSError user-info/domain constants are not typed by the crate. |
| `NSXMLStoreType` | Constant | `NSPersistentStoreCoordinator.h` | XML-store helpers are not exposed by the crate. |

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
