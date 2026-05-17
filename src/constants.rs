use crate::ffi;

pub mod context_notification_names {
    pub const WILL_SAVE: &str = "NSManagedObjectContextWillSaveNotification";
    pub const DID_SAVE: &str = "NSManagedObjectContextDidSaveNotification";
    pub const OBJECTS_DID_CHANGE: &str = "NSManagedObjectContextObjectsDidChangeNotification";
    pub const DID_SAVE_OBJECT_IDS: &str = "NSManagedObjectContextDidSaveObjectIDsNotification";
    pub const DID_MERGE_CHANGES_OBJECT_IDS: &str =
        "NSManagedObjectContextDidMergeChangesObjectIDsNotification";
}

pub mod context_user_info_keys {
    pub const INSERTED_OBJECTS: &str = "NSInsertedObjectsKey";
    pub const UPDATED_OBJECTS: &str = "NSUpdatedObjectsKey";
    pub const DELETED_OBJECTS: &str = "NSDeletedObjectsKey";
    pub const REFRESHED_OBJECTS: &str = "NSRefreshedObjectsKey";
    pub const INVALIDATED_OBJECTS: &str = "NSInvalidatedObjectsKey";
    pub const INVALIDATED_ALL_OBJECTS: &str = "NSInvalidatedAllObjectsKey";
    pub const INSERTED_OBJECT_IDS: &str = "NSInsertedObjectIDsKey";
    pub const UPDATED_OBJECT_IDS: &str = "NSUpdatedObjectIDsKey";
    pub const DELETED_OBJECT_IDS: &str = "NSDeletedObjectIDsKey";
    pub const REFRESHED_OBJECT_IDS: &str = "NSRefreshedObjectIDsKey";
    pub const INVALIDATED_OBJECT_IDS: &str = "NSInvalidatedObjectIDsKey";
    pub const QUERY_GENERATION: &str = "NSManagedObjectContextQueryGenerationKey";
}

pub mod persistent_store_notification_names {
    pub const STORES_WILL_CHANGE: &str = "NSPersistentStoreCoordinatorStoresWillChangeNotification";
    pub const STORES_DID_CHANGE: &str = "NSPersistentStoreCoordinatorStoresDidChangeNotification";
    pub const WILL_REMOVE_STORE: &str = "NSPersistentStoreCoordinatorWillRemoveStoreNotification";
    pub const REMOTE_CHANGE: &str = "NSPersistentStoreRemoteChangeNotification";
    pub const CORE_SPOTLIGHT_INDEX_DID_UPDATE: &str =
        "NSCoreDataCoreSpotlightDelegateIndexDidUpdateNotification";
}

pub mod persistent_store_user_info_keys {
    pub const ADDED_PERSISTENT_STORES: &str = "NSAddedPersistentStoresKey";
    pub const REMOVED_PERSISTENT_STORES: &str = "NSRemovedPersistentStoresKey";
    pub const UUID_CHANGED_PERSISTENT_STORES: &str = "NSUUIDChangedPersistentStoresKey";
    pub const PERSISTENT_STORE_URL: &str = "NSPersistentStoreURLKey";
    pub const PERSISTENT_HISTORY_TOKEN: &str = "NSPersistentHistoryTokenKey";
}

pub mod persistent_store_option_keys {
    pub const IGNORE_VERSIONING: &str = "NSIgnorePersistentStoreVersioningOption";
    pub const SQLITE_ANALYZE: &str = "NSSQLiteAnalyzeOption";
    pub const SQLITE_MANUAL_VACUUM: &str = "NSSQLiteManualVacuumOption";
    pub const FORCE_DESTROY: &str = "NSPersistentStoreForceDestroyOption";
    pub const DEFERRED_LIGHTWEIGHT_MIGRATION: &str =
        "NSPersistentStoreDeferredLightweightMigrationOptionKey";
    pub const STAGED_MIGRATION_MANAGER: &str = "NSPersistentStoreStagedMigrationManagerOptionKey";
    pub const VALIDATE_XML_STORE: &str = "NSValidateXMLStoreOption";
    pub const BINARY_STORE_SECURE_DECODING_CLASSES: &str = "NSBinaryStoreSecureDecodingClasses";
    pub const BINARY_STORE_INSECURE_DECODING_COMPATIBILITY: &str =
        "NSBinaryStoreInsecureDecodingCompatibilityOption";
    pub const CORE_SPOTLIGHT_EXPORTER: &str = "NSCoreDataCoreSpotlightExporter";
}

pub mod persistent_store_metadata_keys {
    pub const STORE_TYPE: &str = "NSStoreTypeKey";
    pub const STORE_UUID: &str = "NSStoreUUIDKey";
    pub const STORE_MODEL_VERSION_HASHES: &str = "NSStoreModelVersionHashesKey";
    pub const STORE_MODEL_VERSION_IDENTIFIERS: &str = "NSStoreModelVersionIdentifiersKey";
    pub const STORE_OS_COMPATIBILITY: &str = "NSPersistentStoreOSCompatibility";
    pub const STORE_CONNECTION_POOL_MAX_SIZE: &str = "NSPersistentStoreConnectionPoolMaxSizeKey";
    pub const STORE_MODEL_VERSION_CHECKSUM: &str = "NSPersistentStoreModelVersionChecksumKey";
}

pub mod error_user_info_keys {
    pub const DETAILED_ERRORS: &str = "NSDetailedErrorsKey";
    pub const VALIDATION_OBJECT: &str = "NSValidationObjectErrorKey";
    pub const VALIDATION_KEY: &str = "NSValidationKeyErrorKey";
    pub const VALIDATION_PREDICATE: &str = "NSValidationPredicateErrorKey";
    pub const VALIDATION_VALUE: &str = "NSValidationValueErrorKey";
    pub const AFFECTED_STORES: &str = "NSAffectedStoresErrorKey";
    pub const AFFECTED_OBJECTS: &str = "NSAffectedObjectsErrorKey";
    pub const PERSISTENT_STORE_SAVE_CONFLICTS: &str = "NSPersistentStoreSaveConflictsErrorKey";
}

pub mod error_domains {
    pub const SQLITE: &str = "NSSQLiteErrorDomain";
}

pub fn coredata_version_number() -> f64 {
    unsafe { ffi::cd_coredata_version_number() }
}
