# coredata

Safe Rust bindings for Apple's [Core Data](https://developer.apple.com/documentation/coredata) framework on macOS.

Requires macOS 13 or later. The staged-migration APIs (`NSStagedMigrationManager`, migration stages and model references) need macOS 14 and return an error on older systems.

## Installation

```toml
[dependencies]
coredata = "0.4"
```

## Concurrency

Core Data contexts and managed objects are confined to their context's queue. This crate enforces that confinement instead of leaving it to the caller:

- `NSManagedObjectContext`, `NSManagedObject`, `NSFetchedResultsController` and `NSFetchedResultsSectionInfo` are `Send + Sync`. Every method runs through `performAndWait` on the context's queue, so a private-queue context can be shared between threads.
- A main-queue context (for example `NSPersistentContainer::view_context`) and its objects can only be used on the main thread. Calls from any other thread return an error with code `bridge_error_codes::WRONG_THREAD`; use a private-queue context such as `new_background_context` for background work.
- `perform_and_wait` runs a closure on the context's queue and returns its result. The closure and the result must be `Send`, because on a main-queue context the closure runs on the main thread. Called off the main thread on a main-queue context, it waits until the main thread runs the main queue, so it only returns in a process whose main thread runs a run loop. `perform` schedules a closure without waiting.
- A managed-object handle keeps its context alive. Once the object leaves its context (the context was reset or rolled back, or the object was deleted and saved), value and state calls return `bridge_error_codes::INVALIDATED`; `managed_object_context` returns `None` and `object_id` still works.
- Configuration objects (models, entity and property descriptions, fetch requests, store descriptions, batch and history requests) are `Send` but not `Sync`, and their clones share one Objective-C object. Configure each one from one thread at a time. Asynchronous APIs copy the request they are given before they queue it.

## Errors

Every fallible call returns `CoreDataError` with the Core Data domain and code, or the `CoreDataBridge` domain and one of the `bridge_error_codes`. Objective-C exceptions that Core Data raises for misuse (malformed predicate formats, unknown keys, editing a model that is in use, unsupported store types, and similar) are caught and returned as `bridge_error_codes::OBJC_EXCEPTION` errors instead of aborting the process. Common mistakes are checked first and return `INVALID_ARGUMENT`: unknown entities and attributes, inserting or deleting an object that belongs to another context, saving without a persistent store, fetched-results controllers without sort descriptors, and batch requests against non-SQLite stores.

Build predicates from values with `NSPredicate::comparison`, `and`, `or` and `not`. `NSPredicate::from_format` takes a trusted format string; pass values as arguments instead of formatting them into the string.

## Async API

Enable the `async` feature to access the `async_api` module:

```toml
[dependencies]
coredata = { version = "0.4", features = ["async"] }
```

The async API is **executor-agnostic** — it works with Tokio, async-std, smol, pollster, or any other `std::future::Future`-compatible runtime.

```rust,no_run
use coredata::async_api::{AsyncPersistentContainer, AsyncManagedObjectContext};
use coredata::{NSPersistentContainer, NSManagedObjectModel};

async fn load_and_save(container: &NSPersistentContainer) -> Result<(), Box<dyn std::error::Error>> {
    // Async store loading
    AsyncPersistentContainer(container).load_persistent_stores().await?;

    // Async save on a background context
    let ctx = container.new_background_context()?;
    AsyncManagedObjectContext(&ctx).perform_save().await?;
    Ok(())
}
```

### Available async types

| Type | Apple API |
|------|-----------|
| `AsyncPersistentContainer` | `NSPersistentContainer.loadPersistentStores(completionHandler:)` |
| `AsyncPersistentCloudKitContainer` | `NSPersistentCloudKitContainer.initializeCloudKitSchema(options:)` |
| `AsyncManagedObjectContext` | `NSManagedObjectContext.perform { save() }` |
| `AsyncHistory` | `NSPersistentHistoryChangeRequest` execute |
| `AsyncBatchOperation` | `NSBatchInsertRequest` / `NSBatchUpdateRequest` |

The context-based futures run their work with `perform` on the context's queue. On a main-queue context that work only runs once the main thread services the main queue, so use a private-queue context in processes without a main run loop.

> **Note:** `performAndWait`-style sync APIs are not futures. Fetched-results
> changes are delivered through `NSFetchedResultsController::set_delegate`;
> CloudKit event notifications are not bridged.

## Quick start

```rust,no_run
use coredata::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model = NSManagedObjectModel::new()?;
    let person = NSEntityDescription::named("Person")?;
    person.set_managed_object_class_name("NSManagedObject")?;

    let name = NSAttributeDescription::new("name", AttributeType::String)?;
    let age = NSAttributeDescription::new("age", AttributeType::Integer32)?;
    person.add_attribute(&name)?;
    person.add_attribute(&age)?;
    model.add_entity(&person)?;

    let container = NSPersistentContainer::new("Example", &model)?;
    let description = NSPersistentStoreDescription::new()?;
    description.set_store_type(store_types::IN_MEMORY)?;
    description.set_should_add_asynchronously(false);
    container.set_persistent_store_descriptions(&[&description])?;
    container.load_persistent_stores()?;

    let context = container.new_background_context()?;
    let person_object = NSManagedObject::new(&person, None)?;
    context.insert(&person_object)?;
    person_object.set_value("name", "doom-fish")?;
    person_object.set_value("age", 7_i32)?;
    context.save()?;
    Ok(())
}
```

## Highlights

- persistent-store descriptions, option keys, CloudKit mirroring options, CloudKit event requests, and synchronous store loading
- richer `NSPersistentStoreCoordinator` administration and `NSPersistentStore` inspection
- `NSManagedObjectContext` concurrency helpers, parent/merge metadata, merge-policy round-tripping, and history-request execution
- `NSManagedObject` state inspection plus `NSManagedObjectID` wrappers, with every object operation confined to its context's queue
- entity, attribute, and relationship metadata including versioning, user info, uniqueness constraints, ordering, and validation rules
- fetch-request result types, persistent-store request/result wrappers, fetched-results controllers/section info with delegate callbacks, prefetch configuration, batch sizing, argument-based predicates, and predicate substitution/evaluation
- persistent-history request/result/transaction/change wrappers plus named Core Data notification names (names only; there is no observer API), metadata keys, option keys, and error constants
- advanced model metadata wrappers including fetched/expression/derived/composite properties and fetch-index descriptions
- SQLite-backed batch insert/update/delete requests and results
- inferred mapping models, staged migrations with custom-stage handlers, and custom-store node helpers (custom store subclasses and migration policies are not supported)
- validation rule metadata and object-validation entry points

## Coverage, examples, and tests

- [`COVERAGE_AUDIT.md`](COVERAGE_AUDIT.md) counts top-level SDK symbols (155 of 161 non-exempt symbols against MacOSX26.2.sdk; the gaps are opaque types with no methods), and [`COVERAGE.md`](COVERAGE.md) tracks family-level depth and missing members.
- `examples/01_in_memory_smoke.rs` plus `examples/02_*` through `examples/16_*` cover every logical area.
- `tests/*_tests.rs` provides smoke coverage for each logical area; `tests/soundness_tests.rs` covers queue confinement, invalidated objects and errors in place of Objective-C exceptions.

Run the full verification suite with:

```bash
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
for ex in examples/*.rs; do cargo run --all-features --example "$(basename "$ex" .rs)"; done
```

## Notes

- CloudKit mirroring and event-request wrappers are available, but live iCloud sync callbacks remain environment-dependent.
- Persistent-history request construction and wrappers are covered; end-to-end history replay is documented in `COVERAGE.md` as a deferred deeper runtime workflow.
- `load_persistent_stores` waits up to 30 seconds (`load_persistent_stores_with_timeout` takes another limit) and returns a timeout error when the limit passes. Core Data cannot cancel a store load, so a load that times out keeps running in the background.

## License

Licensed under either [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
