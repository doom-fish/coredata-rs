use std::time::UNIX_EPOCH;

use coredata::prelude::*;

#[test]
fn persistent_history_change_requests_can_be_built_and_configured(
) -> Result<(), Box<dyn std::error::Error>> {
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
    Ok(())
}
