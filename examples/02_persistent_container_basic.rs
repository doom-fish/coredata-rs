#![allow(clippy::wildcard_imports)]

mod support;

use coredata::prelude::*;
use support::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = basic_model()?;
    let container = in_memory_container("PersistentContainerExample", &fixture.model)?;

    assert_eq!(container.name()?, "PersistentContainerExample");
    assert!(NSPersistentContainer::default_directory()?.is_absolute());

    let descriptions = container.persistent_store_descriptions()?;
    assert_eq!(descriptions.len(), 1);
    let description = &descriptions[0];
    assert_eq!(description.store_type()?, store_types::IN_MEMORY);
    description.set_timeout(3.5)?;
    assert!((description.timeout() - 3.5).abs() < f64::EPSILON);

    let background = container.new_background_context()?;
    assert!(matches!(
        background.concurrency_type(),
        NSManagedObjectContextConcurrencyType::PrivateQueue
    ));
    assert!(matches!(
        container.view_context()?.concurrency_type(),
        NSManagedObjectContextConcurrencyType::MainQueue
    ));

    println!("✅ persistent container example OK");
    Ok(())
}
