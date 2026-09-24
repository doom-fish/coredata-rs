#![allow(clippy::wildcard_imports)]

mod support;

use coredata::prelude::*;
use support::*;

#[test]
fn managed_object_supports_entity_lookup_and_value_round_trip(
) -> Result<(), Box<dyn std::error::Error>> {
    let fixture = basic_model()?;
    let (_coordinator, context) = in_memory_context(&fixture.model)?;
    let person = NSManagedObject::new(&fixture.person, &context)?;
    person.set_value("name", "Bea")?;
    person.set_value("age", 25_i32)?;

    assert_eq!(person.entity()?.name()?, "Person");
    assert_eq!(person.value("name")?.as_str(), Some("Bea"));
    assert_eq!(person.value("age")?.as_i64(), Some(25));
    assert!(!person.is_deleted()?);
    Ok(())
}
