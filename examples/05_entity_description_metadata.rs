#![allow(clippy::wildcard_imports)]

mod support;

use std::collections::BTreeMap;

use support::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut fixture = basic_model()?;

    fixture.person.set_user_info(&BTreeMap::from([(
        String::from("module"),
        String::from("examples"),
    )]))?;
    fixture.person.set_version_hash_modifier(Some("v2"))?;
    fixture
        .person
        .set_renaming_identifier(Some("LegacyPerson"))?;
    fixture
        .person
        .set_uniqueness_constraints(&[vec![String::from("name")]])?;

    assert_eq!(
        fixture.person.user_info()?.get("module"),
        Some(&String::from("examples"))
    );
    assert_eq!(
        fixture.person.renaming_identifier().as_deref(),
        Some("LegacyPerson")
    );
    assert_eq!(fixture.person.uniqueness_constraints()?.len(), 1);
    assert!(fixture.person.managed_object_model()?.is_none());
    let model_person = fixture
        .model
        .entity_named("Person")?
        .ok_or_else(|| std::io::Error::other("missing Person entity"))?;
    let owning_model = model_person
        .managed_object_model()?
        .ok_or_else(|| std::io::Error::other("entity is not in a model"))?;
    assert_eq!(owning_model.entities()?.len(), 1);
    assert_ne!(fixture.person.version_hash()?, "");

    assert!(fixture.model.entity_named("Person")?.is_some());

    println!("✅ entity description example OK");
    Ok(())
}
