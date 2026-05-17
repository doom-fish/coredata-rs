//! Tests for `async_api` module.
//!
//! Run with:
//! ```
//! cargo test --features async --test async_api_tests
//! ```

#![allow(clippy::wildcard_imports)]

mod support;

use coredata::async_api::{
    AsyncBatchOperation, AsyncManagedObjectContext, AsyncPersistentContainer,
};
use coredata::prelude::*;
use support::*;

// ── helpers ──────────────────────────────────────────────────────────────────

/// Build an in-memory container without loading stores (so async load can load it).
fn unloaded_in_memory_container(
    name: &str,
    model: &NSManagedObjectModel,
) -> Result<NSPersistentContainer, CoreDataError> {
    let container = NSPersistentContainer::new(name, model)?;
    let description = NSPersistentStoreDescription::new()?;
    description.set_store_type(store_types::IN_MEMORY)?;
    description.set_should_add_asynchronously(false);
    container.set_persistent_store_descriptions(&[&description])?;
    Ok(container)
}

// ── AsyncPersistentContainer ─────────────────────────────────────────────────

/// Happy path: async load succeeds for an in-memory store.
#[test]
fn test_load_stores_async_ok() {
    let fixture = basic_model().unwrap();
    let container = unloaded_in_memory_container("AsyncLoadOk", &fixture.model).unwrap();
    pollster::block_on(async {
        AsyncPersistentContainer(&container)
            .load_persistent_stores()
            .await
            .expect("async load should succeed");
    });
}

/// Error path: calling load on an already-loaded in-memory container either
/// succeeds (idempotent) or returns an error — both outcomes are acceptable;
/// the future must resolve without panicking.
#[test]
fn test_load_stores_async_resolves() {
    let fixture = basic_model().unwrap();
    let container = unloaded_in_memory_container("AsyncLoadResolves", &fixture.model).unwrap();
    pollster::block_on(async {
        // First load — must succeed.
        AsyncPersistentContainer(&container)
            .load_persistent_stores()
            .await
            .expect("first async load should succeed");
        // Second load — may succeed or error; we just confirm it resolves.
        let _result = AsyncPersistentContainer(&container)
            .load_persistent_stores()
            .await;
    });
}

// ── AsyncManagedObjectContext ─────────────────────────────────────────────────

/// Happy path: async save of a dirty context succeeds.
#[test]
fn test_perform_save_async_ok() {
    let fixture = basic_model().unwrap();
    let container = in_memory_container("AsyncSaveOk", &fixture.model).unwrap();
    // Use a background (private-queue) context to avoid deadlocking the main
    // queue when pollster::block_on runs on the main thread.
    let ctx = container.new_background_context().unwrap();

    let person = NSManagedObject::new(&fixture.person, Some(&ctx)).unwrap();
    person.set_value("name", "Charlie").unwrap();
    person.set_value("age", 40_i32).unwrap();

    pollster::block_on(async {
        AsyncManagedObjectContext(&ctx)
            .perform_save()
            .await
            .expect("async save should succeed");
    });
}

/// An empty context (nothing to save) should also resolve without error.
#[test]
fn test_perform_save_async_empty_context_ok() {
    let fixture = basic_model().unwrap();
    let container = in_memory_container("AsyncSaveEmpty", &fixture.model).unwrap();
    // Background context to avoid main-queue deadlock.
    let ctx = container.new_background_context().unwrap();
    pollster::block_on(async {
        AsyncManagedObjectContext(&ctx)
            .perform_save()
            .await
            .expect("save on empty context should be a no-op");
    });
}

// ── AsyncBatchOperation ───────────────────────────────────────────────────────

/// The batch insert future must succeed against a SQLite-backed store.
#[test]
fn test_batch_insert_async_resolves() {
    let fixture = basic_model().unwrap();
    let (container, artifact) =
        sqlite_container("AsyncBatchInsert", &fixture.model, "async-batch-insert", false).unwrap();
    let bg = container.new_background_context().unwrap();

    let req = NSBatchInsertRequest::new(
        "Person",
        &[{
            let mut m = std::collections::BTreeMap::new();
            m.insert("name".to_string(), Value::String("Dave".to_string()));
            m.insert("age".to_string(), Value::Int64(20));
            m
        }],
    )
    .unwrap();
    req.set_result_type(BatchInsertRequestResultType::StatusOnly);

    pollster::block_on(async {
        AsyncBatchOperation::insert(&bg, &req)
            .await
            .expect("async batch insert should succeed on SQLite store");
    });

    artifact.cleanup();
}

/// The batch update future must succeed against a SQLite-backed store.
#[test]
fn test_batch_update_async_resolves() {
    let fixture = basic_model().unwrap();
    let (container, artifact) =
        sqlite_container("AsyncBatchUpdate", &fixture.model, "async-batch-update", false).unwrap();
    let bg = container.new_background_context().unwrap();

    // Pre-insert a row so the update has something to act on.
    let insert_req = NSBatchInsertRequest::new(
        "Person",
        &[{
            let mut m = std::collections::BTreeMap::new();
            m.insert("name".to_string(), Value::String("Eve".to_string()));
            m.insert("age".to_string(), Value::Int64(30));
            m
        }],
    )
    .unwrap();
    insert_req.set_result_type(BatchInsertRequestResultType::StatusOnly);
    pollster::block_on(async {
        AsyncBatchOperation::insert(&bg, &insert_req)
            .await
            .expect("pre-insert for update test should succeed");
    });

    let update_req = NSBatchUpdateRequest::new("Person").unwrap();
    update_req.set_result_type(BatchUpdateRequestResultType::StatusOnly);
    update_req
        .set_properties_to_update(Some(&std::collections::BTreeMap::from([(
            "age".to_string(),
            Value::from(31_i32),
        )])))
        .unwrap();

    pollster::block_on(async {
        AsyncBatchOperation::update(&bg, &update_req)
            .await
            .expect("async batch update should succeed on SQLite store");
    });

    artifact.cleanup();
}
