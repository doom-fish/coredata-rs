#![allow(clippy::wildcard_imports)]

mod support;

use coredata::prelude::*;
use support::*;

#[test]
fn persistent_container_exposes_description_and_contexts() -> Result<(), Box<dyn std::error::Error>>
{
    let fixture = basic_model()?;
    let container = in_memory_container("PersistentContainerTests", &fixture.model)?;

    assert_eq!(container.name()?, "PersistentContainerTests");
    assert_eq!(container.persistent_store_descriptions()?.len(), 1);
    assert!(matches!(
        container.view_context()?.concurrency_type(),
        NSManagedObjectContextConcurrencyType::MainQueue
    ));
    assert!(matches!(
        container.new_background_context()?.concurrency_type(),
        NSManagedObjectContextConcurrencyType::PrivateQueue
    ));
    Ok(())
}
