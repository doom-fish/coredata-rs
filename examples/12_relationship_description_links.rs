#![allow(clippy::wildcard_imports)]

mod support;

use support::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut fixture = relationship_model()?;

    fixture.children.set_ordered(true)?;
    assert!(fixture.children.is_to_many());
    assert!(fixture.children.is_ordered());
    assert_eq!(
        fixture.children.destination_entity_name().as_deref(),
        Some("Task")
    );
    assert_eq!(
        fixture.parent.inverse_relationship_name().as_deref(),
        Some("tasks")
    );

    let person = fixture
        .model
        .entity_named("Person")?
        .ok_or_else(|| std::io::Error::other("missing Person entity"))?;
    let relationships = person.relationships()?;
    let destination = relationships[0]
        .destination_entity()?
        .ok_or_else(|| std::io::Error::other("relationship is not linked"))?;
    assert_eq!(destination.name()?, "Task");
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
