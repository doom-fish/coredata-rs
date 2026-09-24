#![allow(clippy::wildcard_imports)]

mod support;

use support::*;

#[test]
fn relationship_description_tracks_inverse_ordering_and_destination(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut fixture = relationship_model()?;

    fixture.children.set_ordered(true)?;
    assert!(fixture.children.is_to_many());
    assert!(fixture.children.is_ordered());
    assert_eq!(
        fixture.children.destination_entity_name().as_deref(),
        Some("Task")
    );
    assert!(fixture.children.destination_entity()?.is_none());
    assert_eq!(
        fixture.parent.inverse_relationship_name().as_deref(),
        Some("tasks")
    );
    assert_eq!(
        fixture
            .person
            .relationships_with_destination_entity(&fixture.task)?
            .len(),
        1
    );

    let person = fixture
        .model
        .entity_named("Person")?
        .expect("Person entity");
    let relationships = person.relationships()?;
    let tasks = relationships
        .iter()
        .find(|relationship| relationship.name().ok().as_deref() == Some("tasks"))
        .expect("tasks relationship");
    assert!(!tasks.is_ordered());
    assert_eq!(
        tasks.destination_entity()?.expect("destination").name()?,
        "Task"
    );
    assert_eq!(
        tasks.inverse_relationship()?.expect("inverse").name()?,
        "owner"
    );
    Ok(())
}
