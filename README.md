# coredata

Safe Rust bindings for Apple's [Core Data](https://developer.apple.com/documentation/coredata) framework on macOS.

> **Status:** v0.2.0 expands the crate across 12 logical areas: `PersistentContainer`, `ManagedObjectContext`, `ManagedObject`, `EntityDescription`, `FetchRequest`, `NSPredicate`, `History`, `CloudKitMirroring`, `BatchOperation`, `PersistentStoreCoordinator`, `RelationshipDescription`, and `Validation`.

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

- persistent-store descriptions, option keys, CloudKit mirroring options, and synchronous store loading
- richer `NSPersistentStoreCoordinator` administration and `NSPersistentStore` inspection
- `NSManagedObjectContext` concurrency helpers, parent/merge metadata, and history-request execution
- `NSManagedObject` state inspection plus `NSManagedObjectID` wrappers
- entity, attribute, and relationship metadata including versioning, user info, uniqueness constraints, ordering, and validation rules
- fetch-request result types, prefetch configuration, batch sizing, and predicate substitution/evaluation
- persistent-history request/result/transaction/change wrappers
- SQLite-backed batch insert/delete requests and results
- validation rule metadata and object-validation entry points

## Coverage, examples, and tests

- [`COVERAGE.md`](COVERAGE.md) records the audited API families and deferred rows.
- `examples/01_in_memory_smoke.rs` plus `examples/02_*` through `examples/13_*` cover every logical area.
- `tests/*_tests.rs` provides smoke coverage for each logical area.

Run the full verification suite with:

```bash
cargo clippy --all-targets -- -D warnings
cargo test
for ex in examples/*.rs; do cargo run --example "$(basename "$ex" .rs)"; done
```

## Notes

- CloudKit mirroring wrappers are available, but live iCloud sync/event workflows remain environment-dependent.
- Persistent-history request construction and wrappers are covered; end-to-end history replay is documented in `COVERAGE.md` as a deferred deeper runtime workflow.
- Core Data still enforces queue confinement rules; use `NSManagedObjectContext::perform` or `perform_and_wait` when moving work onto a context-owned queue.

## License

Licensed under either [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
