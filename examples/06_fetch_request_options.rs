#![allow(clippy::wildcard_imports)]

mod support;

use coredata::prelude::*;
use support::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = basic_model()?;
    let request = NSFetchRequest::new("Person")?;
    let empty_prefetch: [&str; 0] = [];

    request.set_entity(Some(&fixture.person))?;
    request.set_result_type(FetchRequestResultType::ManagedObjectId)?;
    request.set_includes_property_values(false);
    request.set_includes_subentities(true);
    request.set_returns_objects_as_faults(false);
    request.set_relationship_key_paths_for_prefetching(&empty_prefetch)?;
    request.set_includes_pending_changes(true);
    request.set_returns_distinct_results(false);
    request.set_fetch_batch_size(16)?;
    request.set_should_refresh_refetched_objects(true);

    assert_eq!(request.entity_name().as_deref(), Some("Person"));
    assert!(matches!(
        request.result_type(),
        FetchRequestResultType::ManagedObjectId
    ));
    assert!(request.relationship_key_paths_for_prefetching()?.is_empty());
    assert_eq!(request.fetch_batch_size(), 16);

    println!("✅ fetch request example OK");
    Ok(())
}
