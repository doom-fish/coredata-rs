use coredata::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    println!("✅ managed object context example OK");
    Ok(())
}
