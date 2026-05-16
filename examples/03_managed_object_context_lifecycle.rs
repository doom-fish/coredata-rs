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

    println!("✅ managed object context example OK");
    Ok(())
}
