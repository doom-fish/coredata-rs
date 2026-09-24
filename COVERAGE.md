# Core Data coverage matrix (revised for v0.4.0)

This matrix covers persistent containers, store coordinators, contexts, managed objects, schema/model metadata, fetch requests, fetched-results controllers, predicates, persistent-store request/result layers, history, CloudKit mirroring/events, batch operations, merge policies/conflicts, migrations/staged migration, validation, and named Core Data constants.

`COVERAGE_AUDIT.md` counts top-level `CoreData.framework` symbols against MacOSX26.2.sdk: 155 of 161 non-exempt symbols (96.3%) have a working wrapper. The six gaps are opaque types with no constructor or methods. That count says nothing about member coverage; this matrix tracks family-level depth: `✅` means the family has public Rust wrappers and Swift bridge coverage, `🟡` means part of the family is missing, and `⏭️` marks intentionally skipped entitlement- or legacy-driven areas.

## PersistentContainer

| API row | Status | Notes |
| --- | --- | --- |
| `NSPersistentStoreDescription` construction, URL, type, configuration, read-only, timeout, SQLite pragmas, named option keys, and option dictionaries | ✅ implemented | Covered by `src/persistent_container.rs`, `src/constants.rs`, and the Swift bridge. |
| `NSPersistentContainer` construction, `loadPersistentStores`, `viewContext`, `newBackgroundContext`, default directory, model/coordinator access, and store-description round-tripping | ✅ implemented | Async loading is bridged synchronously via a semaphore-backed tracker. |
| `performBackgroundTask(_:)` and app-specific background task helpers | 🟡 partial | Context creation is covered; closure-based background task dispatch is not yet exposed. |

## ManagedObjectContext

| API row | Status | Notes |
| --- | --- | --- |
| Context construction, concurrency type, naming, save/insert/delete, `perform`, `performAndWait`, and coordinator attachment | ✅ implemented | Every operation runs on the context's queue; a main-queue context used off the main thread returns a wrong-thread error. |
| Parent context, merge flags, transaction author, inserted/updated/deleted/registered object snapshots, refresh/reset/rollback/process-pending-changes, permanent IDs, merge-policy round-tripping, query-generation symbol coverage, and history merge helpers | ✅ implemented | Includes the `NSManagedObjectContextQueryGenerationKey` constant and `NSQueryGenerationToken` wrappers. |
| Undo manager and remote-context save-merging orchestration | 🟡 partial | Notification and user-info names are exposed as constants only. There is no API to observe notifications. |

## ManagedObject

| API row | Status | Notes |
| --- | --- | --- |
| Object construction, entity lookup, keyed value set/get, and context lookup | ✅ implemented | Objects are created in a context and use the entity with the same name in the context's model; `entity()` returns a read-only view. Values are read and written on the owning context's queue. Keys must name attributes of the entity; handles fail with an invalidated error after reset, rollback, or delete and save. |
| Inserted/updated/deleted/fault/change-state inspection, committed/changed-value snapshots, relationship-fault inspection, object IDs, URI representations, and snapshot-event type coverage | ✅ implemented | Exposed through object/object-id wrappers plus `managed_object::NSSnapshotEventType`. |
| Reference-object helpers and KVC collection mutation helpers | 🟡 partial | Still deferred for a later release. |

## Schema / ModelMetadata

| API row | Status | Notes |
| --- | --- | --- |
| Entity, attribute, and relationship construction; user info, renaming identifiers, uniqueness constraints, ordering, and validation rules | ✅ implemented | Covered by `schema`, `entity_description`, `relationship_description`, and `validation`. Relationships name their destination entity and inverse relationship; the model links them when the entities are added. Adding copies the argument, and children are returned as read-only views. |
| Property-description wrappers, fetched/expression/derived/composite properties, and fetch-index descriptions/elements | ✅ implemented | Added in `src/model_metadata.rs` and `swift-bridge/.../ModelMetadata.swift`. Fetch-index elements take an attribute. |
| Fetched-property requests, fetched properties and fetch indexes inside entities | 🟡 partial | Core Data only accepts a fetched-property request that has a resolved entity, and the crate builds requests from entity names, so `set_fetch_request` returns an error. Fetched properties and fetch indexes cannot be added to entities. |
| Subentity graph authoring, richer aggregation helpers, and app-specific schema convenience layers | 🟡 partial | The top-level SDK symbols are wrapped; deeper authoring helpers remain additive. |

## FetchRequest / PersistentStoreRequest

| API row | Status | Notes |
| --- | --- | --- |
| Fetch-request construction, predicate assignment, sort descriptors, limits/offsets, prefetch flags, batch sizing, and execution | ✅ implemented | Requests name their entity. Execution runs on a copy, so Core Data never stores its resolved entity in the caller's request. |
| Base persistent-store request/result wrappers, async fetch request/result wrappers, save-changes requests, fetch-request expressions, and request/result type enums | ✅ implemented | Added in `src/persistent_store_request.rs` and `PersistentStoreRequest.swift`. |
| `propertiesToFetch`, grouping/aggregation authoring, completion-block customization, and progress observation ergonomics | 🟡 partial | Symbol coverage is complete, but higher-level async orchestration remains intentionally thin. |

## FetchedResultsController

| API row | Status | Notes |
| --- | --- | --- |
| `NSFetchedResultsController` construction, `performFetch`, fetched objects, index-path lookup, section metadata, and cache deletion | ✅ implemented | Added in `src/fetched_results_controller.rs`. |
| Delegate callbacks (`controllerWillChangeContent`, object and section changes, `controllerDidChangeContent`) | ✅ implemented | `NSFetchedResultsController::set_delegate` returns a registration; callbacks run on the context's queue until it is dropped. The snapshot/difference delegate methods are not bridged. |

## NSPredicate

| API row | Status | Notes |
| --- | --- | --- |
| Format-string predicate construction | ✅ implemented | Parse errors and missing arguments return errors. Pass values as arguments, not by editing the format string. |
| Argument-based comparison predicates and `AND` / `OR` / `NOT` compounds | ✅ implemented | `NSPredicate::comparison`, `and`, `or`, `not`; values never pass through the format parser. |
| `predicateWithValue`, predicate format inspection, substitution variables, and JSON-backed evaluation against dictionary-like objects | ✅ implemented | Added in `src/ns_predicate.rs`. |
| Expression trees, `IN`/`BETWEEN`/aggregate operators, and editor/UI-facing predicate helpers | 🟡 partial | Not exposed. |

## History

| API row | Status | Notes |
| --- | --- | --- |
| `NSPersistentHistoryChangeRequest` fetch/delete builders (token/date/transaction) and result-type configuration | ✅ implemented | Added in `src/history.rs`. |
| Result, token, transaction, and change wrappers (`status`, `count`, object IDs, transactions, changes, timestamps, store/process metadata, tombstones, updated properties) | ✅ implemented | Bridged in Swift and wrapped in Rust. |
| Live history processing in automated smoke coverage | 🟡 partial | Request construction is smoke-tested; end-to-end history replay remains runtime-sensitive. |

## CloudKitMirroring

| API row | Status | Notes |
| --- | --- | --- |
| `NSPersistentCloudKitContainerOptions` construction, container identifier, database scope, and store-description attachment | ✅ implemented | Added in `src/cloudkit_mirroring.rs`. |
| CloudKit-backed store-description options plus `NSPersistentCloudKitContainer` construction, model/coordinator access, store-description round-tripping, background/view contexts, load, schema initialization flags, and event request/result/event wrappers | ✅ implemented | Includes event error decoding and record mutability helpers. |
| CKShare/record metadata workflows and live iCloud sync callbacks | ⏭️ skipped | Entitlement- and account-dependent; deferred until provisioned test coverage exists. |

## BatchOperation

| API row | Status | Notes |
| --- | --- | --- |
| `NSBatchDeleteRequest` / `NSBatchDeleteResult` creation, result types, execution, status/count/object-ID inspection, and fetch-request access | ✅ implemented | Added in `src/batch_operation.rs`. |
| `NSBatchInsertRequest` / `NSBatchInsertResult` creation from entity/entity-name + JSON rows, result types, execution, status/count/object-ID inspection | ✅ implemented | SQLite-backed smoke coverage included. |
| `NSBatchUpdateRequest` / `NSBatchUpdateResult` creation, predicate/subentity configuration, entity access, constant-value property updates, execution, status/count/object-ID inspection | ✅ implemented | SQLite-backed smoke coverage exercises update requests alongside insert/delete. |

## MergePolicy / Conflicts

| API row | Status | Notes |
| --- | --- | --- |
| Merge-policy type helpers, singleton policies, custom `NSMergePolicy` construction, and context merge-policy round-tripping | ✅ implemented | Added in `src/merge_policy.rs` and wired into `NSManagedObjectContext`. |
| `NSMergeConflict` and `NSConstraintConflict` | 🟡 type only | Opaque types with no constructor or methods; no API returns them. Save conflicts surface as a `CoreDataError`. |
| Conflict inspection and custom conflict-resolution entry points | 🟡 partial | Not exposed. |

## Migration / StagedMigration

| API row | Status | Notes |
| --- | --- | --- |
| Inferred `NSMappingModel` construction, entity-mapping-name inspection, `NSMigrationManager` construction, source/destination model/context access, migration expression keys, and SQLite store migration execution | ✅ implemented | Added in `src/migration.rs` with a lightweight-migration smoke test and example. |
| Entity/property mappings, model references, migration stages, `NSStagedMigrationManager`, and custom-stage `willMigrate`/`didMigrate` handlers | ✅ implemented | `NSPersistentStoreDescription::set_staged_migration_manager` attaches a manager; a staged migration with custom-stage handlers is covered by `tests/migration_tests.rs`. Requires macOS 14; older systems return an error. |
| `NSEntityMigrationPolicy` | 🟡 type only | Opaque type; custom migration policies need an Objective-C subclass, which is not supported. |

## PersistentStoreCoordinator / CustomStore

| API row | Status | Notes |
| --- | --- | --- |
| Coordinator construction, store enumeration, add/remove/destroy operations, model lookup, current history token, and persistent-store inspection/mutation | ✅ implemented | Existing coordinator surface retained and expanded. |
| Named notification/user-info constants, metadata keys, XML/binary/store-type constants, and generic option-key coverage | ✅ implemented | Added in `src/constants.rs`, plus store-type helpers in `src/store.rs`. |
| Custom-store node helpers (`NSAtomicStoreCacheNode`, `NSIncrementalStoreNode`) | ✅ implemented | Added in `src/custom_store.rs`. |
| `NSAtomicStore` / `NSIncrementalStore` and custom-store subclassing | 🟡 type only | Opaque types with no constructor or methods; custom stores cannot be implemented from Rust. |

## Validation / Constants

| API row | Status | Notes |
| --- | --- | --- |
| Validation-rule metadata on attributes and relationships | ✅ implemented | Exposed as `ValidationRule` plus rule get/set helpers. |
| Managed-object validation entry points (`validateValue`, `validateForInsert`, `validateForUpdate`, `validateForDelete`) | ✅ implemented | Bridged in `Validation.swift` and wrapped in `src/validation.rs`. |
| Named Core Data error domains/user-info constants and framework version helper | ✅ implemented | Added in `src/constants.rs` plus `constants::coredata_version_number()`. |
| Nested multi-error validation decoding | 🟡 partial | Errors are surfaced through `CoreDataError`, but nested detail trees are not yet strongly typed. |

## Deferred / skipped rows

| API row | Status | Reason |
| --- | --- | --- |
| Live CloudKit sync callbacks and CKShare / record-zone convenience APIs attached to Core Data mirroring | ⏭️ skipped | Entitlement- and account-dependent. |
| `NSCoreDataCoreSpotlightDelegate` | ⏭️ skipped | Opaque type only; it cannot be created or attached to a store. |
| Notification observation (context, coordinator, CloudKit event and Spotlight notifications) | ⏭️ skipped | Only the notification names are exposed. |
| Legacy ubiquity / Spotlight external-record APIs listed in `COVERAGE_AUDIT.md` as exempt | ⏭️ skipped | Deprecated in the SDK and intentionally kept out of the safe surface. |
