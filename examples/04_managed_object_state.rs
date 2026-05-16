#![allow(clippy::wildcard_imports)]

mod support;

use coredata::prelude::*;
use support::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = basic_model()?;
    let person = NSManagedObject::new(&fixture.person, None)?;
    person.set_value("name", "Bea")?;
    person.set_value("age", 25_i32)?;

    assert_eq!(person.entity()?.name()?, "Person");
    assert_eq!(person.value("name")?.as_str(), Some("Bea"));
    assert_eq!(person.value("age")?.as_i64(), Some(25));
    assert!(!person.is_deleted());

    println!("✅ managed object example OK");
    Ok(())
}
