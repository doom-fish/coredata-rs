//! # Async API Example
//!
//! Demonstrates the `async_api` module using `pollster::block_on` for
//! executor-agnostic async execution on a headless macOS system.
//!
//! Requires the `async` Cargo feature:
//! ```
//! cargo run --example 16_async_api --features async
//! ```

#![allow(clippy::wildcard_imports)]
#![allow(clippy::future_not_send)]

mod support;

use coredata::async_api::{
    AsyncBatchOperation, AsyncManagedObjectContext, AsyncPersistentContainer,
};
use coredata::prelude::*;
use support::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pollster::block_on(run())
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = basic_model()?;

    // ── 1. AsyncPersistentContainer.load_persistent_stores ─────────────────
    let container = NSPersistentContainer::new("AsyncExample", &fixture.model)?;
    let description = NSPersistentStoreDescription::new()?;
    description.set_store_type(store_types::IN_MEMORY)?;
    description.set_should_add_asynchronously(false);
    container.set_persistent_store_descriptions(&[&description])?;

    AsyncPersistentContainer(&container)
        .load_persistent_stores()
        .await?;
    println!("✅ AsyncPersistentContainer.load_persistent_stores OK");

    // ── 2. AsyncManagedObjectContext.perform_save ───────────────────────────
    // Use a background (private-queue) context so context.perform doesn't
    // try to schedule on the main queue (which pollster already owns).
    let ctx = container.new_background_context()?;

    // Insert an object, then save asynchronously.
    let person = NSManagedObject::new(&fixture.person, Some(&ctx))?;
    person.set_value("name", "Alice")?;
    person.set_value("age", 30_i32)?;

    AsyncManagedObjectContext(&ctx).perform_save().await?;
    println!("✅ AsyncManagedObjectContext.perform_save OK");

    // ── 3. AsyncBatchOperation.insert ──────────────────────────────────────
    // NSBatchInsertRequest requires a SQLite-backed store. We use a temp
    // SQLite container here to demonstrate the async batch plumbing.
    let fixture2 = basic_model()?;
    let (sqlite_container, artifact) =
        sqlite_container("AsyncBatchExample", &fixture2.model, "async-batch-ex", false)?;
    let bg = sqlite_container.new_background_context()?;
    let insert_req = NSBatchInsertRequest::new(
        "Person",
        &[{
            let mut m = std::collections::BTreeMap::new();
            m.insert("name".to_string(), Value::String("Bob".to_string()));
            m.insert("age".to_string(), Value::Int64(25));
            m
        }],
    )?;
    insert_req.set_result_type(BatchInsertRequestResultType::StatusOnly);

    AsyncBatchOperation::insert(&bg, &insert_req).await?;
    println!("✅ AsyncBatchOperation.insert OK");
    artifact.cleanup();

    println!("✅ async_api example complete");
    Ok(())
}
