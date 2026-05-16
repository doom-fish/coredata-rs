use std::collections::BTreeMap;

use coredata::prelude::*;

#[test]
fn predicate_supports_substitution_variables_and_evaluation(
) -> Result<(), Box<dyn std::error::Error>> {
    let predicate = NSPredicate::from_format("age >= $MIN_AGE", &[])?;
    let substituted = predicate.with_substitution_variables(&BTreeMap::from([(
        String::from("MIN_AGE"),
        Value::from(21_i32),
    )]))?;
    let object = BTreeMap::from([(String::from("age"), Value::from(25_i32))]);

    assert!(substituted.evaluate_with_object(Some(&object), None)?);
    assert_eq!(NSPredicate::from_value(true)?.format()?, "TRUEPREDICATE");
    Ok(())
}
