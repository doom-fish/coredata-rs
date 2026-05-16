#![allow(clippy::wildcard_imports)]

mod support;

use std::collections::BTreeMap;

use support::*;

#[test]
fn entity_description_round_trips_metadata_and_lookup() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = basic_model()?;
    fixture.person.set_user_info(&BTreeMap::from([(
        String::from("suite"),
        String::from("entity-tests"),
    )]))?;
    fixture
        .person
        .set_renaming_identifier(Some("LegacyPerson"))?;
    fixture
        .person
        .set_uniqueness_constraints(&[vec![String::from("name")]])?;

    assert!(fixture.model.entity_named("Person")?.is_some());

    assert_eq!(
        fixture.person.user_info()?.get("suite"),
        Some(&String::from("entity-tests"))
    );
    assert_eq!(
        fixture.person.renaming_identifier().as_deref(),
        Some("LegacyPerson")
    );
    assert_eq!(fixture.person.uniqueness_constraints()?.len(), 1);
    Ok(())
}
