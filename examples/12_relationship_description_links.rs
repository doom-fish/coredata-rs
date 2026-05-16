#![allow(clippy::wildcard_imports)]

mod support;

use support::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = relationship_model()?;

    fixture.children.set_ordered(true)?;
    assert!(fixture.children.is_to_many());
    assert!(fixture.children.is_ordered());
    assert_eq!(
        fixture.children.destination_entity()?.unwrap().name()?,
        "Task"
    );
    assert_eq!(
        fixture.parent.inverse_relationship()?.unwrap().name()?,
        "tasks"
    );
    assert_eq!(
        fixture
            .person
            .relationships_with_destination_entity(&fixture.task)?
            .len(),
        1
    );
    assert!(fixture.person.is_kind_of_entity(&fixture.person));
    assert!(!fixture.children.version_hash()?.is_empty());

    println!("✅ relationship description example OK");
    Ok(())
}
