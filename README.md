# coredata

Safe Rust bindings for Apple's [Core Data](https://developer.apple.com/documentation/coredata) framework on macOS.

> **Status:** v0.3.0 adds a Tier-1 `async_api` module (gated behind the `async` feature) wrapping CoreData completion-handler and expensive-sync APIs as executor-agnostic Rust Futures.

## Async API

Enable the `async` feature to access the `async_api` module:

```toml
[dependencies]
coredata = { version = "0.3", features = ["async"] }
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

> **Note:** `performAndWait`-style sync APIs and multi-fire observer patterns (e.g.
> `NSFetchedResultsController` delegate, CloudKit event notifications) are not
> covered here — those belong to a Tier-2 Stream wrapper.

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

    let context = container.view_context()?;
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
- `NSManagedObject` state inspection plus `NSManagedObjectID` wrappers
- entity, attribute, and relationship metadata including versioning, user info, uniqueness constraints, ordering, and validation rules
- fetch-request result types, persistent-store request/result wrappers, fetched-results controllers/section info, prefetch configuration, batch sizing, and predicate substitution/evaluation
- persistent-history request/result/transaction/change wrappers plus named Core Data notifications, metadata keys, option keys, and error constants
- advanced model metadata wrappers including fetched/expression/derived/composite properties and fetch-index descriptions
- SQLite-backed batch insert/update/delete requests and results
- inferred mapping models, staged-migration support types, and custom-store node helpers for migration/extensibility workflows
- validation rule metadata and object-validation entry points

## Coverage, examples, and tests

- [`COVERAGE_AUDIT.md`](COVERAGE_AUDIT.md) records the 100% non-exempt public-symbol audit, and [`COVERAGE.md`](COVERAGE.md) tracks family-level depth/deferred rows.
- `examples/01_in_memory_smoke.rs` plus `examples/02_*` through `examples/15_*` cover every logical area.
- `tests/*_tests.rs` provides smoke coverage for each logical area.

Run the full verification suite with:

```bash
cargo clippy --all-targets -- -D warnings
cargo test
for ex in examples/*.rs; do cargo run --example "$(basename "$ex" .rs)"; done
```

## Notes

- CloudKit mirroring and event-request wrappers are available, but live iCloud sync callbacks remain environment-dependent.
- Persistent-history request construction and wrappers are covered; end-to-end history replay is documented in `COVERAGE.md` as a deferred deeper runtime workflow.
- Core Data still enforces queue confinement rules; use `NSManagedObjectContext::perform` or `perform_and_wait` when moving work onto a context-owned queue.

## License

Licensed under either [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
