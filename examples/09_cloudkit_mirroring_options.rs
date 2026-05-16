#![allow(clippy::wildcard_imports)]

mod support;

use coredata::prelude::*;
use support::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = basic_model()?;
    let container = NSPersistentCloudKitContainer::new("CloudKitExample", &fixture.model)?;
    let options = NSPersistentCloudKitContainerOptions::new("iCloud.com.example.coredata-rs")?;
    options.set_database_scope(CloudKitDatabaseScope::Private)?;

    let description = NSPersistentStoreDescription::new()?;
    description.set_store_type(store_types::SQLITE)?;
    description.set_cloudkit_container_options(Some(&options))?;
    container.set_persistent_store_descriptions(&[&description])?;

    assert_eq!(container.name()?, "CloudKitExample");
    assert_eq!(container.persistent_store_descriptions()?.len(), 1);

    let roundtrip = container.persistent_store_descriptions()?[0]
        .cloudkit_container_options()?
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "missing CloudKit container options",
            )
        })?;
    assert_eq!(
        roundtrip.container_identifier()?,
        "iCloud.com.example.coredata-rs"
    );
    assert!(matches!(
        roundtrip.database_scope(),
        CloudKitDatabaseScope::Private
    ));

    println!("✅ CloudKit example OK");
    Ok(())
}
