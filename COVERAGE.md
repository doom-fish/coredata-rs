# Core Data coverage matrix (v0.2.0)

This audit covers the v0.2.0 target areas requested for `coredata-rs`: persistent containers, store coordinators, contexts, managed objects, entity/relationship metadata, fetch requests, predicates, history, CloudKit mirroring, batch operations, and validation.

Rows below are grouped by API family so the matrix stays reviewable. `✅` means the family is exposed in the safe Rust API and bridged through Swift. `🟡` means the area is partially covered and the note calls out the missing surface. `⏭️` marks surfaces intentionally deferred because they are entitlement-only, app-model-specific, or outside the requested logical-area target for this release.

## PersistentContainer

| API row | Status | Notes |
| --- | --- | --- |
| `NSPersistentStoreDescription` construction, URL, type, configuration, read-only, timeout, SQLite pragmas, and option dictionaries | ✅ implemented | Covered by `src/persistent_container.rs` and `swift-bridge/.../PersistentContainer.swift`. |
| `NSPersistentContainer` construction, `loadPersistentStores`, `viewContext`, `newBackgroundContext`, default directory, model/coordinator access, and store-description round-tripping | ✅ implemented | Async loading is bridged synchronously via a semaphore-backed tracker. |
| `performBackgroundTask(_:)` and app-specific background task helpers | 🟡 partial | Context creation is covered; closure-based background task dispatch is not yet exposed. |

## ManagedObjectContext

| API row | Status | Notes |
| --- | --- | --- |
| Context construction, concurrency type, naming, save/insert/delete, `perform`, `performAndWait`, and coordinator attachment | ✅ implemented | Existing 0.1.x surface retained. |
| Parent context, merge flags, transaction author, inserted/updated/deleted/registered object snapshots, refresh/reset/rollback/process-pending-changes, permanent IDs, and history merge helpers | ✅ implemented | Added in `src/managed_object_context.rs`. |
| Undo manager, merge notifications, remote-context save merging, and object-trump/store-trump policy helpers | 🟡 partial | Not exposed in v0.2.0. |

## ManagedObject

| API row | Status | Notes |
| --- | --- | --- |
| Object construction, entity lookup, keyed value set/get, and context lookup | ✅ implemented | Existing wrapper extended in `src/managed_object.rs`. |
| Inserted/updated/deleted/fault/change-state inspection, committed/changed-value snapshots, relationship-fault inspection, object IDs, and URI representations | ✅ implemented | Exposed through new object/object-id wrappers. |
| Reference-object helpers, KVC collection mutation helpers, and notification plumbing | 🟡 partial | Deferred for a later release. |

## EntityDescription

| API row | Status | Notes |
| --- | --- | --- |
| Entity construction, naming, managed-object class naming, attribute/relationship attachment, and entity enumeration | ✅ implemented | Existing builder surface retained. |
| Entity lookup by name, insert-new-object helper, managed-object-model lookup, abstract flag, user info, version-hash metadata, renaming identifiers, uniqueness constraints, destination-relationship lookup, and kind checks | ✅ implemented | Added in `src/entity_description.rs`. |
| Fetched properties, subentity graphs, indexes/constraints beyond uniqueness, and migration-policy helpers | 🟡 partial | Not yet exposed. |

## RelationshipDescription

| API row | Status | Notes |
| --- | --- | --- |
| Relationship construction, naming, optional/transient flags, destination entity, inverse relationship, counts, and delete rule | ✅ implemented | Existing 0.1.x surface retained. |
| Ordered/to-many inspection and version-hash access | ✅ implemented | Added in `src/relationship_description.rs`. |
| Derived attributes, fetched properties, and index metadata outside relationship basics | 🟡 partial | Not exposed in v0.2.0. |

## FetchRequest

| API row | Status | Notes |
| --- | --- | --- |
| Request construction, predicate assignment, sort descriptors, fetch limit, fetch offset, and managed-object execution | ✅ implemented | Existing 0.1.x surface retained. |
| Entity assignment, entity-name inspection, result type, subentity/property/fault flags, prefetch key paths, pending-change flag, distinct-results flag, batch size, and refetch-refresh flag | ✅ implemented | Added in `src/fetch_request.rs`. |
| `propertiesToFetch`, grouping/aggregation, asynchronous fetch requests, and expression descriptions | 🟡 partial | Deferred. |

## NSPredicate

| API row | Status | Notes |
| --- | --- | --- |
| Format-string predicate construction | ✅ implemented | Existing 0.1.x surface retained. |
| `predicateWithValue`, predicate format inspection, substitution variables, and JSON-backed evaluation against dictionary-like objects | ✅ implemented | Added in `src/ns_predicate.rs`. |
| Compound predicates, expression trees, and editor/UI-facing predicate helpers | 🟡 partial | Not exposed. |

## History

| API row | Status | Notes |
| --- | --- | --- |
| `NSPersistentHistoryChangeRequest` fetch/delete builders (token/date/transaction) and result-type configuration | ✅ implemented | Added in `src/history.rs`. |
| Result, token, transaction, and change wrappers (`status`, `count`, object IDs, transactions, changes, timestamps, store/process metadata, tombstones, updated properties) | ✅ implemented | Bridged in Swift and wrapped in Rust. |
| Live history processing in automated smoke coverage | 🟡 partial | Request construction is smoke-tested; end-to-end history replay remains sensitive to store/runtime setup and is not part of the fast example suite. |

## CloudKitMirroring

| API row | Status | Notes |
| --- | --- | --- |
| `NSPersistentCloudKitContainerOptions` construction, container identifier, and database scope | ✅ implemented | Added in `src/cloudkit_mirroring.rs`. |
| CloudKit-backed store-description options plus `NSPersistentCloudKitContainer` construction, model/coordinator access, store-description round-tripping, background/view contexts, load, and schema initialization flags | ✅ implemented | Bridged through `CloudKitMirroring.swift`. |
| Event streams, CKShare/record metadata workflows, and live iCloud sync callbacks | ⏭️ skipped | Entitlement- and account-dependent; deferred until the crate can exercise them under a provisioned test environment. |

## BatchOperation

| API row | Status | Notes |
| --- | --- | --- |
| `NSBatchDeleteRequest` / `NSBatchDeleteResult` creation, result types, execution, status/count/object-ID inspection | ✅ implemented | Added in `src/batch_operation.rs`. |
| `NSBatchInsertRequest` / `NSBatchInsertResult` creation from entity-name + JSON rows, result types, execution, status/count/object-ID inspection | ✅ implemented | SQLite-backed smoke coverage included. |
| `NSBatchUpdateRequest` and dictionary/object result handling beyond insert/delete | 🟡 partial | Not yet exposed. |

## PersistentStoreCoordinator

| API row | Status | Notes |
| --- | --- | --- |
| Coordinator construction and `addPersistentStore` for in-memory/SQLite/binary stores | ✅ implemented | Existing 0.1.x surface retained. |
| Name, model lookup, store enumeration, add-with-description, remove, URL lookup, destroy, current history token, and persistent-store inspection/mutation (`url`, `identifier`, type, read-only`) | ✅ implemented | Added in `src/persistent_store_coordinator.rs`. |
| Replace/migrate metadata APIs and custom incremental-store subclasses | 🟡 partial | Deferred. |

## Validation

| API row | Status | Notes |
| --- | --- | --- |
| Validation-rule metadata on attributes and relationships | ✅ implemented | Exposed as `ValidationRule` plus rule get/set helpers. |
| Managed-object validation entry points (`validateValue`, `validateForInsert`, `validateForUpdate`, `validateForDelete`) | ✅ implemented | Bridged in `Validation.swift` and wrapped in `src/validation.rs`. |
| Rich NSError user-info decoding for multi-error validation failures | 🟡 partial | Errors are surfaced through `CoreDataError`, but nested Core Data validation detail dictionaries are not yet typed. |

## Deferred / skipped rows

| API row | Status | Reason |
| --- | --- | --- |
| `NSPersistentCloudKitContainer.Event` / live mirroring observation APIs | ⏭️ skipped | Requires provisioned iCloud entitlements and live CloudKit state. |
| CKShare / record-zone convenience APIs attached to Core Data mirroring | ⏭️ skipped | Entitlement- and account-dependent. |
| `NSBatchUpdateRequest` | ⏭️ skipped | Useful, but separate from the delete/insert subset targeted for v0.2.0. |
| `NSAsynchronousFetchRequest` and `NSFetchedResultsController` style higher-level fetch orchestration | ⏭️ skipped | Separate higher-level workflow surface not requested for this release. |
| Incremental stores, mapping models, migration managers, and other legacy/extensibility APIs outside the targeted logical areas | ⏭️ skipped | Large secondary feature set; reserved for a future major expansion. |
