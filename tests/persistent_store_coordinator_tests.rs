#![allow(clippy::wildcard_imports)]

mod support;

use std::io::Error;

use coredata::prelude::*;
use support::*;

#[test]
fn persistent_store_coordinator_can_add_remove_and_destroy_sqlite_store(
) -> Result<(), Box<dyn std::error::Error>> {
    let fixture = basic_model()?;
    let artifact = SqliteStoreArtifact::new("coordinator-tests")?;
    let coordinator = NSPersistentStoreCoordinator::new(&fixture.model)?;

    let description = NSPersistentStoreDescription::with_url(&artifact.path)?;
    description.set_store_type(store_types::SQLITE)?;
    description.set_should_add_asynchronously(false);

    let store = coordinator
        .add_persistent_store_with_description(&description, 30)?
        .ok_or_else(|| Error::new(std::io::ErrorKind::Other, "expected persistent store"))?;
    assert_eq!(coordinator.persistent_stores()?.len(), 1);
    assert!(coordinator
        .persistent_store_for_url(&artifact.path)?
        .is_some());
    assert_eq!(coordinator.url_for_persistent_store(&store)?, artifact.path);

    coordinator.remove_persistent_store(&store)?;
    coordinator.destroy_persistent_store(&artifact.path, store_types::SQLITE, None)?;
    artifact.cleanup();
    Ok(())
}
