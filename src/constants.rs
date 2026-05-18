use crate::ffi;

/// Core Data items for context notification names.
pub mod context_notification_names {
    /// Mirrors `NSManagedObjectContextWillSaveNotification`.
    pub const WILL_SAVE: &str = "NSManagedObjectContextWillSaveNotification";
    /// Mirrors `NSManagedObjectContextDidSaveNotification`.
    pub const DID_SAVE: &str = "NSManagedObjectContextDidSaveNotification";
    /// Mirrors `NSManagedObjectContextObjectsDidChangeNotification`.
    pub const OBJECTS_DID_CHANGE: &str = "NSManagedObjectContextObjectsDidChangeNotification";
    /// Mirrors `NSManagedObjectContextDidSaveObjectIDsNotification`.
    pub const DID_SAVE_OBJECT_IDS: &str = "NSManagedObjectContextDidSaveObjectIDsNotification";
    /// Mirrors the corresponding Core Data constant.
    pub const DID_MERGE_CHANGES_OBJECT_IDS: &str =
        "NSManagedObjectContextDidMergeChangesObjectIDsNotification";
}

/// Core Data items for context user info keys.
pub mod context_user_info_keys {
    /// Mirrors `NSInsertedObjectsKey`.
    pub const INSERTED_OBJECTS: &str = "NSInsertedObjectsKey";
    /// Mirrors `NSUpdatedObjectsKey`.
    pub const UPDATED_OBJECTS: &str = "NSUpdatedObjectsKey";
    /// Mirrors `NSDeletedObjectsKey`.
    pub const DELETED_OBJECTS: &str = "NSDeletedObjectsKey";
    /// Mirrors `NSRefreshedObjectsKey`.
    pub const REFRESHED_OBJECTS: &str = "NSRefreshedObjectsKey";
    /// Mirrors `NSInvalidatedObjectsKey`.
    pub const INVALIDATED_OBJECTS: &str = "NSInvalidatedObjectsKey";
    /// Mirrors `NSInvalidatedAllObjectsKey`.
    pub const INVALIDATED_ALL_OBJECTS: &str = "NSInvalidatedAllObjectsKey";
    /// Mirrors `NSInsertedObjectIDsKey`.
    pub const INSERTED_OBJECT_IDS: &str = "NSInsertedObjectIDsKey";
    /// Mirrors `NSUpdatedObjectIDsKey`.
    pub const UPDATED_OBJECT_IDS: &str = "NSUpdatedObjectIDsKey";
    /// Mirrors `NSDeletedObjectIDsKey`.
    pub const DELETED_OBJECT_IDS: &str = "NSDeletedObjectIDsKey";
    /// Mirrors `NSRefreshedObjectIDsKey`.
    pub const REFRESHED_OBJECT_IDS: &str = "NSRefreshedObjectIDsKey";
    /// Mirrors `NSInvalidatedObjectIDsKey`.
    pub const INVALIDATED_OBJECT_IDS: &str = "NSInvalidatedObjectIDsKey";
    /// Mirrors `NSManagedObjectContextQueryGenerationKey`.
    pub const QUERY_GENERATION: &str = "NSManagedObjectContextQueryGenerationKey";
}

/// Core Data items for persistent store notification names.
pub mod persistent_store_notification_names {
    /// Mirrors `NSPersistentStoreCoordinatorStoresWillChangeNotification`.
    pub const STORES_WILL_CHANGE: &str = "NSPersistentStoreCoordinatorStoresWillChangeNotification";
    /// Mirrors `NSPersistentStoreCoordinatorStoresDidChangeNotification`.
    pub const STORES_DID_CHANGE: &str = "NSPersistentStoreCoordinatorStoresDidChangeNotification";
    /// Mirrors `NSPersistentStoreCoordinatorWillRemoveStoreNotification`.
    pub const WILL_REMOVE_STORE: &str = "NSPersistentStoreCoordinatorWillRemoveStoreNotification";
    /// Mirrors `NSPersistentStoreRemoteChangeNotification`.
    pub const REMOTE_CHANGE: &str = "NSPersistentStoreRemoteChangeNotification";
    /// Mirrors the corresponding Core Data constant.
    pub const CORE_SPOTLIGHT_INDEX_DID_UPDATE: &str =
        "NSCoreDataCoreSpotlightDelegateIndexDidUpdateNotification";
}

/// Core Data items for persistent store user info keys.
pub mod persistent_store_user_info_keys {
    /// Mirrors `NSAddedPersistentStoresKey`.
    pub const ADDED_PERSISTENT_STORES: &str = "NSAddedPersistentStoresKey";
    /// Mirrors `NSRemovedPersistentStoresKey`.
    pub const REMOVED_PERSISTENT_STORES: &str = "NSRemovedPersistentStoresKey";
    /// Mirrors `NSUUIDChangedPersistentStoresKey`.
    pub const UUID_CHANGED_PERSISTENT_STORES: &str = "NSUUIDChangedPersistentStoresKey";
    /// Mirrors `NSPersistentStoreURLKey`.
    pub const PERSISTENT_STORE_URL: &str = "NSPersistentStoreURLKey";
    /// Mirrors `NSPersistentHistoryTokenKey`.
    pub const PERSISTENT_HISTORY_TOKEN: &str = "NSPersistentHistoryTokenKey";
}

/// Core Data items for persistent store option keys.
pub mod persistent_store_option_keys {
    /// Mirrors `NSIgnorePersistentStoreVersioningOption`.
    pub const IGNORE_VERSIONING: &str = "NSIgnorePersistentStoreVersioningOption";
    /// Mirrors `NSSQLiteAnalyzeOption`.
    pub const SQLITE_ANALYZE: &str = "NSSQLiteAnalyzeOption";
    /// Mirrors `NSSQLiteManualVacuumOption`.
    pub const SQLITE_MANUAL_VACUUM: &str = "NSSQLiteManualVacuumOption";
    /// Mirrors `NSPersistentStoreForceDestroyOption`.
    pub const FORCE_DESTROY: &str = "NSPersistentStoreForceDestroyOption";
    /// Mirrors the corresponding Core Data constant.
    pub const DEFERRED_LIGHTWEIGHT_MIGRATION: &str =
        "NSPersistentStoreDeferredLightweightMigrationOptionKey";
    /// Mirrors `NSPersistentStoreStagedMigrationManagerOptionKey`.
    pub const STAGED_MIGRATION_MANAGER: &str = "NSPersistentStoreStagedMigrationManagerOptionKey";
    /// Mirrors `NSValidateXMLStoreOption`.
    pub const VALIDATE_XML_STORE: &str = "NSValidateXMLStoreOption";
    /// Mirrors `NSBinaryStoreSecureDecodingClasses`.
    pub const BINARY_STORE_SECURE_DECODING_CLASSES: &str = "NSBinaryStoreSecureDecodingClasses";
    /// Mirrors the corresponding Core Data constant.
    pub const BINARY_STORE_INSECURE_DECODING_COMPATIBILITY: &str =
        "NSBinaryStoreInsecureDecodingCompatibilityOption";
    /// Mirrors `NSCoreDataCoreSpotlightExporter`.
    pub const CORE_SPOTLIGHT_EXPORTER: &str = "NSCoreDataCoreSpotlightExporter";
}

/// Core Data items for persistent store metadata keys.
pub mod persistent_store_metadata_keys {
    /// Mirrors `NSStoreTypeKey`.
    pub const STORE_TYPE: &str = "NSStoreTypeKey";
    /// Mirrors `NSStoreUUIDKey`.
    pub const STORE_UUID: &str = "NSStoreUUIDKey";
    /// Mirrors `NSStoreModelVersionHashesKey`.
    pub const STORE_MODEL_VERSION_HASHES: &str = "NSStoreModelVersionHashesKey";
    /// Mirrors `NSStoreModelVersionIdentifiersKey`.
    pub const STORE_MODEL_VERSION_IDENTIFIERS: &str = "NSStoreModelVersionIdentifiersKey";
    /// Mirrors `NSPersistentStoreOSCompatibility`.
    pub const STORE_OS_COMPATIBILITY: &str = "NSPersistentStoreOSCompatibility";
    /// Mirrors `NSPersistentStoreConnectionPoolMaxSizeKey`.
    pub const STORE_CONNECTION_POOL_MAX_SIZE: &str = "NSPersistentStoreConnectionPoolMaxSizeKey";
    /// Mirrors `NSPersistentStoreModelVersionChecksumKey`.
    pub const STORE_MODEL_VERSION_CHECKSUM: &str = "NSPersistentStoreModelVersionChecksumKey";
}

/// Core Data items for error user info keys.
pub mod error_user_info_keys {
    /// Mirrors `NSDetailedErrorsKey`.
    pub const DETAILED_ERRORS: &str = "NSDetailedErrorsKey";
    /// Mirrors `NSValidationObjectErrorKey`.
    pub const VALIDATION_OBJECT: &str = "NSValidationObjectErrorKey";
    /// Mirrors `NSValidationKeyErrorKey`.
    pub const VALIDATION_KEY: &str = "NSValidationKeyErrorKey";
    /// Mirrors `NSValidationPredicateErrorKey`.
    pub const VALIDATION_PREDICATE: &str = "NSValidationPredicateErrorKey";
    /// Mirrors `NSValidationValueErrorKey`.
    pub const VALIDATION_VALUE: &str = "NSValidationValueErrorKey";
    /// Mirrors `NSAffectedStoresErrorKey`.
    pub const AFFECTED_STORES: &str = "NSAffectedStoresErrorKey";
    /// Mirrors `NSAffectedObjectsErrorKey`.
    pub const AFFECTED_OBJECTS: &str = "NSAffectedObjectsErrorKey";
    /// Mirrors `NSPersistentStoreSaveConflictsErrorKey`.
    pub const PERSISTENT_STORE_SAVE_CONFLICTS: &str = "NSPersistentStoreSaveConflictsErrorKey";
}

/// Core Data items for error domains.
pub mod error_domains {
    /// Mirrors `NSSQLiteErrorDomain`.
    pub const SQLITE: &str = "NSSQLiteErrorDomain";
}

/// Wraps `coredata_version_number(...)`.
pub fn coredata_version_number() -> f64 {
    unsafe { ffi::cd_coredata_version_number() }
}
