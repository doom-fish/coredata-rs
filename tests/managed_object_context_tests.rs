use coredata::prelude::*;

#[test]
fn managed_object_context_constructors_report_expected_concurrency_types_and_merge_policy(
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

    let overwrite = NSMergePolicy::overwrite_policy()?;
    private.set_merge_policy(&overwrite)?;
    assert!(matches!(
        private.merge_policy()?.merge_type(),
        MergePolicyType::Overwrite
    ));
    assert!(matches!(
        NSMergePolicy::new(MergePolicyType::MergeByPropertyObjectTrump)?.merge_type(),
        MergePolicyType::MergeByPropertyObjectTrump
    ));
    Ok(())
}
