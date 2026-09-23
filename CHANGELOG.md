# Changelog

All notable changes to `coredata` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - Unreleased

### Security

- Contexts and managed objects were `Send` but every call ran on the
  calling thread, so using a background context from another thread or the
  view context off the main thread raced inside Core Data. Every context,
  managed-object and fetched-results-controller operation now runs through
  `performAndWait` on the context's queue.
- A managed object's `managedObjectContext` became a dangling pointer once
  its context was deallocated, so `managed_object_context()` could read
  freed memory. Managed-object handles now keep their context alive.
- With a nil argument array, a predicate format containing a conversion
  such as `%@` made Foundation dereference a missing argument and crash.
  The validation-rule path did this with every rule. Predicates are now
  always parsed with an argument array.

### Fixed

- Objective-C exceptions no longer abort the process. A new
  `CoreDataObjCBridge` target catches them for predicate parsing,
  substitution and evaluation, validation rules, attribute get/set, fetch,
  count, save, insert, delete, refresh, `object_with_id`, request execution,
  fetched-results controllers, model edits after the model is in use, and
  store loading. They return errors with code
  `bridge_error_codes::OBJC_EXCEPTION`.
- Unknown entities and attributes, cross-context insert/delete/refresh,
  abstract entities, saving without a store or coordinator, fetched-results
  controllers without sort descriptors, batch requests on non-SQLite stores,
  unsupported store types and self-parented contexts return
  `INVALID_ARGUMENT` errors.
- `set_fetch_limit(usize::MAX)`, `set_fetch_offset` and
  `set_fetch_batch_size` no longer trap in Swift. The fetch limit is clamped
  to `u32::MAX` because Core Data stores it in 32 bits; before, a limit of
  2^32 + 5 silently became 5. Relationship minimum and maximum counts above
  `i64::MAX` return errors.
- Unknown enum raw values (attribute types, delete rules, merge policies,
  mapping and collation types, result types, database scopes, concurrency
  types) return errors; Swift's imported `init(rawValue:)` accepted any
  value.
- An `NSError` with code 0 (for example a save that fails without an error)
  no longer reads as success.
- Fetching with a dictionary, object-ID or count result type returns an
  error instead of trapping in a typed Swift cast. `as_asynchronous_result`
  checks the result type instead of trapping on use.
- Loading a container with no store descriptions completes immediately, in
  both the synchronous and the async API (it waited 30 seconds or forever).
- Async errors keep their Core Data domain and code, and async requests are
  snapshotted before they are queued.
- `load_persistent_stores` reports a timeout honestly; the load itself
  cannot be cancelled and keeps running (documented).
- Package.swift no longer ships the excluded duplicate sources
  `Context.swift`, `Model.swift` and `Store.swift`.

### Changed

- **Breaking:** `NSManagedObjectContext`, `NSManagedObject`,
  `NSFetchedResultsController` and `NSFetchedResultsSectionInfo` are now
  `Sync`. A main-queue context and its objects return
  `bridge_error_codes::WRONG_THREAD` off the main thread.
- **Breaking:** managed-object handles fail with
  `bridge_error_codes::INVALIDATED` after their object leaves the context
  (reset, rollback, deleted and saved).
- **Breaking:** `perform_and_wait` requires `F: Send` and `R: Send` and
  returns `Result<R, CoreDataError>`; `perform` returns `Result`.
- **Breaking:** these now return `Result`: `NSManagedObjectContext::{has_changes,
  delete, name, transaction_author, refresh_object, process_pending_changes,
  reset, rollback, refresh_all_objects, automatically_merges_changes_from_parent}`,
  `NSManagedObject::{is_inserted, is_updated, is_deleted, has_changes,
  has_persistent_changed_values, is_fault}`,
  `NSFetchedResultsSectionInfo::{index_title, number_of_objects}`,
  `NSAsynchronousFetchResult::final_result_count`,
  `NSPersistentStoreAsynchronousResult::operation_error`, and the
  `set_result_type` methods of batch, history and CloudKit event requests.
- **Breaking:** `NSFetchedResultsControllerDelegate` is a real delegate trait
  with default methods instead of an empty marker trait.
- **Breaking:** `NSManagedObject::value`, `set_value` and `validate_value`
  accept only attribute names; `has_fault_for_relationship_named` and
  `object_ids_for_relationship_named` accept only relationship names.
- **Breaking:** unsupported `NSManagedObjectContextConcurrencyType` values
  are rejected.
- `NSFetchedResultsController` keeps its own copy of the fetch request, and
  `fetch_request()` returns a copy.
- `rust-version` is now 1.82 (was 1.76).
- `doom-fish-utils` requirement is now `>=0.4.1, <0.5`.

### Added

- `NSFetchedResultsController::set_delegate` delivers
  will/did-change-content and object and section changes to an
  `NSFetchedResultsControllerDelegate`, returning a
  `FetchedResultsControllerDelegateRegistration`.
- `NSCustomMigrationStage::set_will_migrate_handler` /
  `set_did_migrate_handler` and
  `NSPersistentStoreDescription::set_staged_migration_manager`, so staged
  migrations can run.
- `NSPredicate::comparison`, `and`, `or` and `not` with `PredicateOperator`
  and `PredicateOptions`, to build predicates without format strings.
- `NSFetchRequest::fetch_limit` and `fetch_offset`.
- `bridge_error_codes` for the bridge error codes.

### Removed

- `COVERAGE_AUDIT_V2.md`, which listed symbols that do not exist in the SDK.
  `COVERAGE_AUDIT.md` no longer counts the six method-less wrappers
  (`NSAtomicStore`, `NSIncrementalStore`, `NSEntityMigrationPolicy`,
  `NSMergeConflict`, `NSConstraintConflict`,
  `NSCoreDataCoreSpotlightDelegate`) as verified.

## [0.3.8] - 2026-05-20

- Migrated local `take_string` body to call `doom_fish_utils::ffi_string::take_owned_cstring_c`. Centralises the duplicated FFI take-string pattern fleet-wide. No public API change.

## [0.3.7] - 2026-05-20

- Added in-`src/` unit tests across src/error.rs, src/merge_policy.rs, src/schema.rs, and src/validation.rs (Tier 2 quality polish), providing fast `cargo test --lib` fail-fast signal alongside the existing integration tests under `tests/`.

## [0.3.6] - 2026-05-20

- Clippy hygiene sweep: cleared all `-D warnings` lints across the crate. No public API change.

## [0.3.5] - 2026-05-20

- Widen `doom-fish-utils` dependency bound to `<0.4` so the 0.3.x SPSC-ring release resolves cleanly. No source changes.

## [0.3.4] - 2026-05-19

- Bump MSRV from 1.70 to 1.76 to match fleet baseline.

## 0.3.3 - 2026-05-25

- Added a crate-wide public API doc pass across `src/` (excluding `src/ffi/`), documenting public modules, wrapper types, enums, fields, constants, and methods with Core Data counterpart references.
- Documented the macro-generated wrapper structs emitted by `impl_object_wrapper!`, bringing the library to a missing-docs-clean public API surface.

## 0.3.2 - 2026-05-25

- Added `#[derive(Debug)]` to the public async adapter and marker structs that can derive it directly: `AsyncPersistentContainer`, `AsyncPersistentCloudKitContainer`, `AsyncManagedObjectContext`, `AsyncHistory`, and `AsyncBatchOperation`.
- Kept the manual `Debug` impls on async future wrappers whose internal completion state does not currently support a derived implementation.

## 0.3.1 - 2026-05-25

### Fixed: quality pass — async/unsafe/hygiene audit

- **Panic safety (UB fix):** all `extern "C"` callbacks in `src/async_api.rs`
  (`load_stores_cb`, `init_schema_cb`, `perform_save_cb`, `fetch_history_cb`,
  `batch_insert_cb`, `batch_update_cb`) now wrap their bodies in
  `doom_fish_utils::panic_safe::catch_user_panic`. Previously a panic inside any
  callback would unwind across the FFI boundary into Swift — undefined behaviour.
- **Panic safety (UB fix):** `perform_trampoline` and `perform_and_wait_trampoline`
  in `src/context.rs` now wrap user-closure calls in `catch_user_panic` for the same
  reason.
- **SAFETY comments:** added `// SAFETY:` annotations to every previously bare
  `unsafe {}` block in `src/async_api.rs` and `src/context.rs`, documenting the
  pointer-validity and single-fire callback contracts that make each call sound.
- **Cargo.toml:** tightened `doom-fish-utils` version spec from `"0.1"` to
  `">=0.1, <0.3"` to admit the next minor release without accidentally pulling in a
  potentially breaking 0.3 release.



### Added: `async_api` module (Tier-1 async)

Gated behind the `async` Cargo feature. Wraps CoreData completion-handler and
expensive-synchronous Apple APIs as executor-agnostic Rust `Future`s backed by
`doom-fish-utils` `AsyncCompletion`.

| Future type | Apple API | Notes |
|---|---|---|
| `LoadStoresFuture` / `AsyncPersistentContainer` | `NSPersistentContainer.loadPersistentStores(completionHandler:)` | Aggregates all store-description callbacks |
| `InitializeCloudKitSchemaFuture` / `AsyncPersistentCloudKitContainer` | `NSPersistentCloudKitContainer.initializeCloudKitSchema(options:)` | Offloaded to background queue |
| `ContextPerformSaveFuture` / `AsyncManagedObjectContext` | `NSManagedObjectContext.perform { save() }` | Uses context's private queue |
| `FetchHistoryFuture` / `AsyncHistory` | `NSPersistentHistoryChangeRequest` execute | Via `context.perform` |
| `BatchInsertFuture` / `BatchUpdateFuture` / `AsyncBatchOperation` | `NSBatchInsertRequest` / `NSBatchUpdateRequest` | Via `context.perform`; requires SQLite store |

`NSPersistentStoreCoordinator.performAndWait` and `NSManagedObjectContext.performAndWait`
are synchronous and are not Future candidates. `NSFetchedResultsController` delegate
and CloudKit event notifications are multi-fire observer patterns; those are deferred
to Tier-2 Stream wrappers.

New dependencies: `doom-fish-utils` (workspace sibling), `pollster = "0.3"` (dev).

New files: `src/async_api.rs`, `src/ffi/async_api.rs`,
`swift-bridge/Sources/CoreDataBridge/Async.swift`,
`examples/16_async_api.rs`, `tests/async_api_tests.rs`.

## 0.2.2 - 2026-05-17

- Closed the remaining 91 non-exempt Core Data SDK audit gaps and brought `COVERAGE_AUDIT.md` to 100% public-symbol coverage.
- Added persistent-store request/result wrappers, async fetch wrappers, save-changes requests, fetch-request expressions, and query-generation token support.
- Added advanced model-metadata wrappers (`NSPropertyDescription`, fetched/expression/derived/composite properties, and fetch-index types), plus custom-store node helpers and conflict/delegate marker types.
- Added staged-migration support wrappers (`NSEntityMapping`, `NSPropertyMapping`, `NSManagedObjectModelReference`, migration stages, and `NSStagedMigrationManager`) and expanded the named constant surface.
- Refreshed `README.md`, `COVERAGE.md`, and `COVERAGE_AUDIT.md`, and bumped the crate version to `0.2.2`.

## 0.2.1 - 2026-05-16

- Added `NSFetchedResultsController` / `NSFetchedResultsSectionInfo` wrappers plus fetched-results smoke coverage and example usage.
- Added `NSBatchUpdateRequest` / `NSBatchUpdateResult` wrappers, extending the batch-operation area from insert/delete to batch updates.
- Added CloudKit event request/result/event wrappers and named helpers for the event notification and user-info constants.
- Added `NSMergePolicy` wrappers, merge-policy type/singleton helpers, and managed-object-context merge-policy round-tripping.
- Added `NSMappingModel` / `NSMigrationManager` wrappers with inferred-mapping and SQLite migration smoke coverage.
- Refreshed `COVERAGE.md` and `COVERAGE_AUDIT.md` and bumped the crate version to `0.2.1`.

## 0.2.0 - 2026-05-16

- Split the Swift bridge into logical-area files and added matching Rust area modules.
- Expanded coverage across persistent containers, store coordinators, contexts, managed objects, entity/relationship metadata, fetch requests, predicates, history, CloudKit mirroring, batch operations, and validation.
- Added `NSPersistentStoreDescription`, `NSPersistentStore`, `NSManagedObjectID`, persistent-history wrappers, CloudKit container/options wrappers, and batch insert/delete wrappers.
- Added `COVERAGE.md`, 12 logical-area smoke tests, and 12 logical-area examples alongside the original in-memory smoke example.

## 0.1.0 - 2026-05-16

- Initial release.
- Added managed-object model loading and programmatic schema construction.
- Added persistent-store coordinators, persistent containers, contexts, fetch requests, and predicates.
- Added an in-memory smoke example covering insert + fetch.
