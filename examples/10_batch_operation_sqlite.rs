#![allow(clippy::wildcard_imports)]

mod support;

use coredata::prelude::*;
use support::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = basic_model()?;
    let artifact;

    {
        let (container, sqlite_artifact) =
            sqlite_container("BatchExample", &fixture.model, "batch-example", false)?;
        artifact = sqlite_artifact;

        let context = container.new_background_context()?;
        let insert_request = NSBatchInsertRequest::new(
            "Person",
            &[
                person_row("Ada", 32),
                person_row("Bea", 25),
                person_row("Cleo", 41),
            ],
        )?;
        insert_request.set_result_type(BatchInsertRequestResultType::Count);
        let insert_result = insert_request.execute(&context)?;
        assert_eq!(insert_result.count(), 3);

        let count_request = NSFetchRequest::new("Person")?;
        assert_eq!(context.count(&count_request)?, 3);

        let delete_request = NSBatchDeleteRequest::from_fetch_request(&count_request)?;
        delete_request.set_result_type(BatchDeleteRequestResultType::Count);
        let delete_result = delete_request.execute(&context)?;
        assert_eq!(delete_result.count(), 3);
    }

    artifact.cleanup();
    println!("✅ batch operation example OK");
    Ok(())
}
