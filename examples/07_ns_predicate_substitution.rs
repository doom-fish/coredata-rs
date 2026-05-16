mod support;

use std::collections::BTreeMap;

use coredata::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let predicate = NSPredicate::from_format("age >= $MIN_AGE", &[])?;
    let substituted = predicate.with_substitution_variables(&BTreeMap::from([(
        String::from("MIN_AGE"),
        Value::from(30_i32),
    )]))?;

    let object = BTreeMap::from([(String::from("age"), Value::from(41_i32))]);
    assert!(substituted.evaluate_with_object(Some(&object), None)?);
    assert_eq!(NSPredicate::from_value(true)?.format()?, "TRUEPREDICATE");

    println!("✅ predicate example OK");
    Ok(())
}
