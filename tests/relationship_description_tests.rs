#![allow(clippy::wildcard_imports)]

mod support;

use support::*;

#[test]
fn relationship_description_tracks_inverse_ordering_and_destination(
) -> Result<(), Box<dyn std::error::Error>> {
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
    Ok(())
}
