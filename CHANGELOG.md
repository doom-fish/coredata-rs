# Changelog

## 0.3.0 - 2026-05-25

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
