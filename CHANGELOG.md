# Changelog

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
