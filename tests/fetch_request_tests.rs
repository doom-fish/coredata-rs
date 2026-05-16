#![allow(clippy::wildcard_imports)]

mod support;

use coredata::prelude::*;
use support::*;

#[test]
fn fetch_request_returns_object_ids_and_prefetch_settings() -> Result<(), Box<dyn std::error::Error>>
{
    let fixture = basic_model()?;
    let request = NSFetchRequest::new("Person")?;
    let empty_prefetch: [&str; 0] = [];

    request.set_entity(Some(&fixture.person))?;
    request.set_result_type(FetchRequestResultType::ManagedObjectId)?;
    request.set_relationship_key_paths_for_prefetching(&empty_prefetch)?;
    request.set_fetch_batch_size(8)?;

    assert!(matches!(
        request.result_type(),
        FetchRequestResultType::ManagedObjectId
    ));
    assert_eq!(request.entity_name().as_deref(), Some("Person"));
    assert!(request.relationship_key_paths_for_prefetching()?.is_empty());
    assert_eq!(request.fetch_batch_size(), 8);
    Ok(())
}
