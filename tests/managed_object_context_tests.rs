use coredata::prelude::*;

#[test]
fn managed_object_context_constructors_report_expected_concurrency_types(
) -> Result<(), Box<dyn std::error::Error>> {
    let main = NSManagedObjectContext::new_main_queue()?;
    let private = NSManagedObjectContext::new_private_queue()?;

    assert!(matches!(
        main.concurrency_type(),
        NSManagedObjectContextConcurrencyType::MainQueue
    ));
    assert!(matches!(
        private.concurrency_type(),
        NSManagedObjectContextConcurrencyType::PrivateQueue
    ));
    Ok(())
}
