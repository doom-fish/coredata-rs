# coredata

Safe Rust bindings for Apple's [Core Data](https://developer.apple.com/documentation/coredata) framework on macOS.

> **Status:** v0.1.0 covers the practical managed-object surface for programmatic models, model loading from `.momd`, persistent-store coordinators, persistent containers, managed-object contexts, fetch requests, predicates, schema descriptions, and basic `NSManagedObject` value access.

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

    let coordinator = NSPersistentStoreCoordinator::new(&model)?;
    coordinator.add_persistent_store(store_types::IN_MEMORY, None::<&str>, None::<&str>, None)?;

    let context = NSManagedObjectContext::new_main_queue()?;
    context.set_persistent_store_coordinator(&coordinator)?;

    let person_object = NSManagedObject::new(&person, None)?;
    context.insert(&person_object)?;
    person_object.set_value("name", "doom-fish")?;
    person_object.set_value("age", 7_i32)?;
    context.save()?;
    Ok(())
}
```

## Highlights

- `NSManagedObjectModel::new`, `from_url`, `add_entity`, and schema introspection
- `NSEntityDescription`, `NSAttributeDescription`, and `NSRelationshipDescription` builders
- `NSPersistentStoreCoordinator::add_persistent_store` for SQLite, binary, and in-memory stores
- `NSPersistentContainer::new`, `load_persistent_stores`, `view_context`, and `new_background_context`
- `NSManagedObjectContext::{perform, perform_and_wait, save, has_changes, insert, delete}`
- `NSManagedObject::{entity, set_value, value}`
- `NSFetchRequest` with predicates, sort descriptors, limits, and offsets
- `NSPredicate::from_format` for basic format-string predicates

## Smoke example

Run the in-memory Core Data smoke test with:

```bash
cargo run --all-features --example 01_in_memory_smoke
```

It programmatically builds a `Person` model, creates an in-memory store, inserts three rows, fetches them sorted by age, and prints `✅ coredata insert + fetch OK`.

## Threading

Core Data still enforces its normal queue confinement rules. Use `NSManagedObjectContext::perform` or `perform_and_wait` when moving work onto a context-owned queue.

## License

Licensed under either [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
