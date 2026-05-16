#![allow(clippy::wildcard_imports)]

mod support;

use coredata::prelude::*;
use support::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = basic_model()?;
    fixture
        .name
        .set_validation_rules(&[ValidationRule::new("SELF != nil", "Name required")])?;
    assert_eq!(fixture.name.validation_rules()?.len(), 1);

    assert_eq!(validation_error_codes::MISSING_MANDATORY_PROPERTY, 1570);

    println!("✅ validation example OK");
    Ok(())
}
