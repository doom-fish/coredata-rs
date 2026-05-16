#![allow(clippy::wildcard_imports)]

mod support;

use std::collections::BTreeMap;

use support::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = basic_model()?;

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
    assert_eq!(fixture.person.managed_object_model()?.entities()?.len(), 1);
    assert!(!fixture.person.version_hash()?.is_empty());

    assert!(fixture.model.entity_named("Person")?.is_some());

    println!("✅ entity description example OK");
    Ok(())
}
