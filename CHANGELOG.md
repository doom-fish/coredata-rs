# Changelog

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
