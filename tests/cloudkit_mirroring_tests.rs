#![allow(clippy::wildcard_imports)]

mod support;

use coredata::prelude::*;
use support::*;

#[test]
fn cloudkit_configuration_and_event_requests_round_trip_without_loading_stores(
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

    let request = NSPersistentCloudKitContainerEventRequest::fetch_events_after_date(
        std::time::UNIX_EPOCH,
    )?;
    assert!(matches!(
        request.result_type(),
        NSPersistentCloudKitContainerEventResultType::Events
    ));
    request.set_result_type(NSPersistentCloudKitContainerEventResultType::CountEvents);
    assert!(matches!(
        request.result_type(),
        NSPersistentCloudKitContainerEventResultType::CountEvents
    ));

    let all_events_request = NSPersistentCloudKitContainerEventRequest::fetch_events_after_event(None)?;
    assert!(matches!(
        all_events_request.result_type(),
        NSPersistentCloudKitContainerEventResultType::Events
    ));

    let fetch_request = NSPersistentCloudKitContainerEventRequest::fetch_request_for_events()?;
    assert!(fetch_request.entity_name().is_some());
    assert_eq!(
        event_notification_names::CHANGED,
        "NSPersistentCloudKitContainerEventChangedNotification"
    );
    assert_eq!(
        event_user_info_keys::EVENT,
        "NSPersistentCloudKitContainerEventUserInfoKey"
    );
    Ok(())
}
