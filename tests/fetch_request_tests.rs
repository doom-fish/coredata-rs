#![allow(clippy::wildcard_imports)]

use coredata::prelude::*;

#[test]
fn fetch_request_returns_object_ids_and_prefetch_settings() -> Result<(), Box<dyn std::error::Error>>
{
    let mut request = NSFetchRequest::new("Person")?;
    let empty_prefetch: [&str; 0] = [];

    request.set_result_type(FetchRequestResultType::ManagedObjectId)?;
    request.set_relationship_key_paths_for_prefetching(&empty_prefetch)?;
    request.set_fetch_batch_size(8)?;

    assert!(matches!(
        request.result_type(),
        FetchRequestResultType::ManagedObjectId
    ));
    assert_eq!(request.entity_name().as_deref(), Some("Person"));
    assert_eq!(
        request.relationship_key_paths_for_prefetching()?,
        [] as [std::string::String; 0]
    );
    assert_eq!(request.fetch_batch_size(), 8);
    Ok(())
}
