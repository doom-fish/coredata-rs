#![allow(clippy::wildcard_imports)]

mod support;

use coredata::prelude::*;
use support::*;

#[test]
fn batch_insert_and_delete_work_against_sqlite() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = basic_model()?;
    let artifact;

    {
        let (container, sqlite_artifact) =
            sqlite_container("BatchTests", &fixture.model, "batch-tests", false)?;
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
        assert_eq!(insert_request.execute(&context)?.count(), 3);

        let request = NSFetchRequest::new("Person")?;
        assert_eq!(context.count(&request)?, 3);

        let delete_request = NSBatchDeleteRequest::from_fetch_request(&request)?;
        delete_request.set_result_type(BatchDeleteRequestResultType::Count);
        assert_eq!(delete_request.execute(&context)?.count(), 3);
    }

    artifact.cleanup();
    Ok(())
}
