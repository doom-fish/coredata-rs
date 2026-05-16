#![allow(clippy::wildcard_imports)]

mod support;

use coredata::prelude::*;
use support::*;

#[test]
fn cloudkit_configuration_round_trips_without_loading_stores(
) -> Result<(), Box<dyn std::error::Error>> {
    let fixture = basic_model()?;
    let container = NSPersistentCloudKitContainer::new("CloudKitTests", &fixture.model)?;
    let options = NSPersistentCloudKitContainerOptions::new("iCloud.com.example.coredata-rs")?;
    options.set_database_scope(CloudKitDatabaseScope::Private)?;

    let description = NSPersistentStoreDescription::new()?;
    description.set_store_type(store_types::SQLITE)?;
    description.set_cloudkit_container_options(Some(&options))?;
    container.set_persistent_store_descriptions(&[&description])?;

    let roundtrip = container.persistent_store_descriptions()?[0]
        .cloudkit_container_options()?
        .ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "missing CloudKit options")
        })?;
    assert_eq!(
        roundtrip.container_identifier()?,
        "iCloud.com.example.coredata-rs"
    );
    assert!(matches!(
        roundtrip.database_scope(),
        CloudKitDatabaseScope::Private
    ));
    Ok(())
}
