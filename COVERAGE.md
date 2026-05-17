# Core Data coverage matrix (v0.2.2)

This audit covers the v0.2.2 target areas requested for `coredata-rs`: persistent containers, store coordinators, contexts, managed objects, schema/model metadata, fetch requests, fetched-results controllers, predicates, persistent-store request/result layers, history, CloudKit mirroring/events, batch operations, merge policies/conflicts, migrations/staged migration, validation, and named Core Data constants.

`COVERAGE_AUDIT.md` now reports **100% coverage of all non-exempt top-level `CoreData.framework` public symbols**. The matrix below tracks family-level depth: `✅` means the family has public Rust wrappers and Swift bridge coverage, `🟡` means higher-order workflows or callback-heavy members are still additive work, and `⏭️` marks intentionally skipped entitlement- or legacy-driven areas.

## PersistentContainer

| API row | Status | Notes |
| --- | --- | --- |
| `NSPersistentStoreDescription` construction, URL, type, configuration, read-only, timeout, SQLite pragmas, named option keys, and option dictionaries | ✅ implemented | Covered by `src/persistent_container.rs`, `src/constants.rs`, and the Swift bridge. |
| `NSPersistentContainer` construction, `loadPersistentStores`, `viewContext`, `newBackgroundContext`, default directory, model/coordinator access, and store-description round-tripping | ✅ implemented | Async loading is bridged synchronously via a semaphore-backed tracker. |
| `performBackgroundTask(_:)` and app-specific background task helpers | 🟡 partial | Context creation is covered; closure-based background task dispatch is not yet exposed. |

## ManagedObjectContext

| API row | Status | Notes |
| --- | --- | --- |
| Context construction, concurrency type, naming, save/insert/delete, `perform`, `performAndWait`, and coordinator attachment | ✅ implemented | Existing 0.1.x surface retained. |
| Parent context, merge flags, transaction author, inserted/updated/deleted/registered object snapshots, refresh/reset/rollback/process-pending-changes, permanent IDs, merge-policy round-tripping, query-generation symbol coverage, and history merge helpers | ✅ implemented | Includes the `NSManagedObjectContextQueryGenerationKey` constant and `NSQueryGenerationToken` wrappers. |
| Undo manager and remote-context save-merging orchestration | 🟡 partial | Named notification/user-info constants are exposed, but higher-level merge orchestration remains additive work. |

## ManagedObject

| API row | Status | Notes |
| --- | --- | --- |
| Object construction, entity lookup, keyed value set/get, and context lookup | ✅ implemented | Existing wrapper extended in `src/managed_object.rs`. |
| Inserted/updated/deleted/fault/change-state inspection, committed/changed-value snapshots, relationship-fault inspection, object IDs, URI representations, and snapshot-event type coverage | ✅ implemented | Exposed through object/object-id wrappers plus `managed_object::NSSnapshotEventType`. |
| Reference-object helpers and KVC collection mutation helpers | 🟡 partial | Still deferred for a later release. |

## Schema / ModelMetadata

| API row | Status | Notes |
| --- | --- | --- |
| Entity, attribute, and relationship construction; user info, renaming identifiers, uniqueness constraints, ordering, and validation rules | ✅ implemented | Covered by `schema`, `entity_description`, `relationship_description`, and `validation`. |
| Property-description wrappers, fetched/expression/derived/composite properties, fetch-index descriptions/elements, and fetched-property request attachment | ✅ implemented | Added in `src/model_metadata.rs` and `swift-bridge/.../ModelMetadata.swift`. |
| Subentity graph authoring, richer aggregation helpers, and app-specific schema convenience layers | 🟡 partial | The top-level SDK symbols are wrapped; deeper authoring helpers remain additive. |

## FetchRequest / PersistentStoreRequest

| API row | Status | Notes |
| --- | --- | --- |
| Fetch-request construction, predicate assignment, sort descriptors, limits/offsets, entity assignment, prefetch flags, batch sizing, and execution | ✅ implemented | Existing fetch-request surface retained and expanded. |
| Base persistent-store request/result wrappers, async fetch request/result wrappers, save-changes requests, fetch-request expressions, and request/result type enums | ✅ implemented | Added in `src/persistent_store_request.rs` and `PersistentStoreRequest.swift`. |
| `propertiesToFetch`, grouping/aggregation authoring, completion-block customization, and progress observation ergonomics | 🟡 partial | Symbol coverage is complete, but higher-level async orchestration remains intentionally thin. |

## FetchedResultsController

| API row | Status | Notes |
| --- | --- | --- |
| `NSFetchedResultsController` construction, `performFetch`, fetched objects, index-path lookup, section metadata, and cache deletion | ✅ implemented | Added in `src/fetched_results_controller.rs`. |
| Delegate protocol symbol coverage | ✅ implemented | `NSFetchedResultsControllerDelegate` now has a public Rust marker trait. |
| Delegate-driven change callback bridging | 🟡 partial | Callback delivery remains deferred. |

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
| `NSMergeConflict` and `NSConstraintConflict` top-level symbol coverage | ✅ implemented | Public Rust wrapper types are now exported for both conflict classes. |
| Conflict inspection and custom conflict-resolution entry points | 🟡 partial | The crate currently exposes marker/wrapper coverage rather than rich conflict introspection helpers. |

## Migration / StagedMigration

| API row | Status | Notes |
| --- | --- | --- |
| Inferred `NSMappingModel` construction, entity-mapping-name inspection, `NSMigrationManager` construction, source/destination model/context access, migration expression keys, and SQLite store migration execution | ✅ implemented | Added in `src/migration.rs` with a lightweight-migration smoke test and example. |
| Entity/property mappings, migration-policy marker coverage, model references, migration stages, and `NSStagedMigrationManager` | ✅ implemented | Added in `src/migration_support.rs`; staged migration wrappers are available behind the platform/runtime constraints enforced by Core Data. |
| End-to-end staged-migration authoring with versioned model artifacts | 🟡 partial | Public symbols are wrapped, but app-specific staged migration flows remain lightly exercised. |

## PersistentStoreCoordinator / CustomStore

| API row | Status | Notes |
| --- | --- | --- |
| Coordinator construction, store enumeration, add/remove/destroy operations, model lookup, current history token, and persistent-store inspection/mutation | ✅ implemented | Existing coordinator surface retained and expanded. |
| Named notification/user-info constants, metadata keys, XML/binary/store-type constants, and generic option-key coverage | ✅ implemented | Added in `src/constants.rs`, plus store-type helpers in `src/store.rs`. |
| Custom-store node helpers (`NSAtomicStoreCacheNode`, `NSIncrementalStoreNode`) and marker coverage for `NSAtomicStore` / `NSIncrementalStore` | ✅ implemented | Added in `src/custom_store.rs`. |
| Custom incremental-store subclassing and replace/migrate metadata convenience APIs | 🟡 partial | Top-level symbol coverage is complete, but subclass-oriented workflows remain additive. |

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
| Delegate-driven fetched-results change callback delivery | ⏭️ skipped | Requires a callback/lifetime model beyond the current safe wrapper scope. |
| Legacy ubiquity / Spotlight external-record APIs listed in `COVERAGE_AUDIT.md` as exempt | ⏭️ skipped | Deprecated in the SDK and intentionally kept out of the safe surface. |
