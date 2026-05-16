use std::time::UNIX_EPOCH;

use coredata::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fetch_request = NSPersistentHistoryChangeRequest::fetch_history_after_date(UNIX_EPOCH)?;
    fetch_request.set_result_type(PersistentHistoryResultType::TransactionsOnly);
    assert!(matches!(
        fetch_request.result_type(),
        PersistentHistoryResultType::TransactionsOnly
    ));

    let delete_request = NSPersistentHistoryChangeRequest::delete_history_before_date(UNIX_EPOCH)?;
    delete_request.set_result_type(PersistentHistoryResultType::StatusOnly);
    assert!(matches!(
        delete_request.result_type(),
        PersistentHistoryResultType::StatusOnly
    ));

    println!("✅ history example OK");
    Ok(())
}
