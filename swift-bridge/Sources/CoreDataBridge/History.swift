import CoreData
import Foundation

@_cdecl("cd_persistent_history_change_request_fetch_after_token")
public func cdPersistentHistoryChangeRequestFetchAfterToken(
    _ tokenPtr: UnsafeMutableRawPointer?,
    _ outRequest: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let token = tokenPtr.map { cdBorrow($0) as NSPersistentHistoryToken }
    outRequest?.pointee = cdRetain(NSPersistentHistoryChangeRequest.fetchHistory(after: token))
    return CDR_OK
}

@_cdecl("cd_persistent_history_change_request_fetch_after_date")
public func cdPersistentHistoryChangeRequestFetchAfterDate(
    _ timestamp: Double,
    _ outRequest: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outRequest?.pointee = cdRetain(NSPersistentHistoryChangeRequest.fetchHistory(after: Date(timeIntervalSince1970: timestamp)))
    return CDR_OK
}

@_cdecl("cd_persistent_history_change_request_fetch_after_transaction")
public func cdPersistentHistoryChangeRequestFetchAfterTransaction(
    _ transactionPtr: UnsafeMutableRawPointer?,
    _ outRequest: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let transaction = transactionPtr.map { cdBorrow($0) as NSPersistentHistoryTransaction }
    outRequest?.pointee = cdRetain(NSPersistentHistoryChangeRequest.fetchHistory(after: transaction))
    return CDR_OK
}

@_cdecl("cd_persistent_history_change_request_delete_before_token")
public func cdPersistentHistoryChangeRequestDeleteBeforeToken(
    _ tokenPtr: UnsafeMutableRawPointer?,
    _ outRequest: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let token = tokenPtr.map { cdBorrow($0) as NSPersistentHistoryToken }
    outRequest?.pointee = cdRetain(NSPersistentHistoryChangeRequest.deleteHistory(before: token))
    return CDR_OK
}

@_cdecl("cd_persistent_history_change_request_delete_before_date")
public func cdPersistentHistoryChangeRequestDeleteBeforeDate(
    _ timestamp: Double,
    _ outRequest: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outRequest?.pointee = cdRetain(NSPersistentHistoryChangeRequest.deleteHistory(before: Date(timeIntervalSince1970: timestamp)))
    return CDR_OK
}

@_cdecl("cd_persistent_history_change_request_delete_before_transaction")
public func cdPersistentHistoryChangeRequestDeleteBeforeTransaction(
    _ transactionPtr: UnsafeMutableRawPointer?,
    _ outRequest: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let transaction = transactionPtr.map { cdBorrow($0) as NSPersistentHistoryTransaction }
    outRequest?.pointee = cdRetain(NSPersistentHistoryChangeRequest.deleteHistory(before: transaction))
    return CDR_OK
}

@_cdecl("cd_persistent_history_change_request_get_result_type")
public func cdPersistentHistoryChangeRequestGetResultType(_ requestPtr: UnsafeMutableRawPointer?) -> Int64 {
    guard let requestPtr else {
        return 0
    }
    let request: NSPersistentHistoryChangeRequest = cdBorrow(requestPtr)
    return Int64(request.resultType.rawValue)
}

@_cdecl("cd_persistent_history_change_request_set_result_type")
public func cdPersistentHistoryChangeRequestSetResultType(_ requestPtr: UnsafeMutableRawPointer?, _ resultType: Int64) {
    guard let requestPtr else {
        return
    }
    let request: NSPersistentHistoryChangeRequest = cdBorrow(requestPtr)
    request.resultType = NSPersistentHistoryResultType(rawValue: Int(resultType)) ?? .transactionsAndChanges
}

@_cdecl("cd_managed_object_context_execute_persistent_history_change_request")
public func cdManagedObjectContextExecutePersistentHistoryChangeRequest(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ requestPtr: UnsafeMutableRawPointer?,
    _ outResult: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let contextPtr, let requestPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object context or history request")
        }
        let context: NSManagedObjectContext = cdBorrow(contextPtr)
        let request: NSPersistentHistoryChangeRequest = cdBorrow(requestPtr)
        let result = try context.execute(request) as? NSPersistentHistoryResult
        if let result {
            outResult?.pointee = cdRetain(result)
        } else {
            outResult?.pointee = nil
        }
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_persistent_history_result_get_result_type")
public func cdPersistentHistoryResultGetResultType(_ resultPtr: UnsafeMutableRawPointer?) -> Int64 {
    guard let resultPtr else {
        return 0
    }
    let result: NSPersistentHistoryResult = cdBorrow(resultPtr)
    return Int64(result.resultType.rawValue)
}

@_cdecl("cd_persistent_history_result_get_status")
public func cdPersistentHistoryResultGetStatus(_ resultPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let resultPtr else {
        return 0
    }
    let result: NSPersistentHistoryResult = cdBorrow(resultPtr)
    guard let number = result.result as? NSNumber else {
        return 0
    }
    return number.boolValue ? 1 : 0
}

@_cdecl("cd_persistent_history_result_get_count")
public func cdPersistentHistoryResultGetCount(_ resultPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let resultPtr else {
        return 0
    }
    let result: NSPersistentHistoryResult = cdBorrow(resultPtr)
    guard let number = result.result as? NSNumber else {
        return 0
    }
    return number.uint64Value
}

@_cdecl("cd_persistent_history_result_get_object_ids")
public func cdPersistentHistoryResultGetObjectIDs(_ resultPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let resultPtr else {
        return nil
    }
    let result: NSPersistentHistoryResult = cdBorrow(resultPtr)
    guard let objectIDs = result.result as? [NSManagedObjectID] else {
        return nil
    }
    return cdRetain(objectIDs as NSArray)
}

@_cdecl("cd_persistent_history_result_get_transactions")
public func cdPersistentHistoryResultGetTransactions(_ resultPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let resultPtr else {
        return nil
    }
    let result: NSPersistentHistoryResult = cdBorrow(resultPtr)
    guard let transactions = result.result as? [NSPersistentHistoryTransaction] else {
        return nil
    }
    return cdRetain(transactions as NSArray)
}

@_cdecl("cd_persistent_history_result_get_changes")
public func cdPersistentHistoryResultGetChanges(_ resultPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let resultPtr else {
        return nil
    }
    let result: NSPersistentHistoryResult = cdBorrow(resultPtr)
    guard let changes = result.result as? [NSPersistentHistoryChange] else {
        return nil
    }
    return cdRetain(changes as NSArray)
}

@_cdecl("cd_persistent_history_transaction_get_timestamp")
public func cdPersistentHistoryTransactionGetTimestamp(_ transactionPtr: UnsafeMutableRawPointer?) -> Double {
    guard let transactionPtr else {
        return 0
    }
    let transaction: NSPersistentHistoryTransaction = cdBorrow(transactionPtr)
    return transaction.timestamp.timeIntervalSince1970
}

@_cdecl("cd_persistent_history_transaction_get_changes")
public func cdPersistentHistoryTransactionGetChanges(_ transactionPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let transactionPtr else {
        return nil
    }
    let transaction: NSPersistentHistoryTransaction = cdBorrow(transactionPtr)
    guard let changes = transaction.changes else {
        return nil
    }
    return cdRetain(changes as NSArray)
}

@_cdecl("cd_persistent_history_transaction_get_transaction_number")
public func cdPersistentHistoryTransactionGetTransactionNumber(_ transactionPtr: UnsafeMutableRawPointer?) -> Int64 {
    guard let transactionPtr else {
        return 0
    }
    let transaction: NSPersistentHistoryTransaction = cdBorrow(transactionPtr)
    return transaction.transactionNumber
}

@_cdecl("cd_persistent_history_transaction_get_store_id")
public func cdPersistentHistoryTransactionGetStoreID(_ transactionPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let transactionPtr else {
        return nil
    }
    let transaction: NSPersistentHistoryTransaction = cdBorrow(transactionPtr)
    return cdCString(transaction.storeID)
}

@_cdecl("cd_persistent_history_transaction_get_bundle_id")
public func cdPersistentHistoryTransactionGetBundleID(_ transactionPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let transactionPtr else {
        return nil
    }
    let transaction: NSPersistentHistoryTransaction = cdBorrow(transactionPtr)
    return cdCString(transaction.bundleID)
}

@_cdecl("cd_persistent_history_transaction_get_process_id")
public func cdPersistentHistoryTransactionGetProcessID(_ transactionPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let transactionPtr else {
        return nil
    }
    let transaction: NSPersistentHistoryTransaction = cdBorrow(transactionPtr)
    return cdCString(transaction.processID)
}

@_cdecl("cd_persistent_history_transaction_get_context_name")
public func cdPersistentHistoryTransactionGetContextName(_ transactionPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let transactionPtr else {
        return nil
    }
    let transaction: NSPersistentHistoryTransaction = cdBorrow(transactionPtr)
    return transaction.contextName.flatMap { cdCString($0) }
}

@_cdecl("cd_persistent_history_transaction_get_author")
public func cdPersistentHistoryTransactionGetAuthor(_ transactionPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let transactionPtr else {
        return nil
    }
    let transaction: NSPersistentHistoryTransaction = cdBorrow(transactionPtr)
    return transaction.author.flatMap { cdCString($0) }
}

@_cdecl("cd_persistent_history_transaction_get_token")
public func cdPersistentHistoryTransactionGetToken(_ transactionPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let transactionPtr else {
        return nil
    }
    let transaction: NSPersistentHistoryTransaction = cdBorrow(transactionPtr)
    return cdRetain(transaction.token)
}

@_cdecl("cd_persistent_history_change_get_change_id")
public func cdPersistentHistoryChangeGetChangeID(_ changePtr: UnsafeMutableRawPointer?) -> Int64 {
    guard let changePtr else {
        return 0
    }
    let change: NSPersistentHistoryChange = cdBorrow(changePtr)
    return change.changeID
}

@_cdecl("cd_persistent_history_change_get_change_type")
public func cdPersistentHistoryChangeGetChangeType(_ changePtr: UnsafeMutableRawPointer?) -> Int64 {
    guard let changePtr else {
        return 0
    }
    let change: NSPersistentHistoryChange = cdBorrow(changePtr)
    return Int64(change.changeType.rawValue)
}

@_cdecl("cd_persistent_history_change_get_changed_object_id")
public func cdPersistentHistoryChangeGetChangedObjectID(_ changePtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let changePtr else {
        return nil
    }
    let change: NSPersistentHistoryChange = cdBorrow(changePtr)
    return cdRetain(change.changedObjectID)
}

@_cdecl("cd_persistent_history_change_get_tombstone_json")
public func cdPersistentHistoryChangeGetTombstoneJSON(
    _ changePtr: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let changePtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing history change")
        }
        let change: NSPersistentHistoryChange = cdBorrow(changePtr)
        let payload = try cdValuePayloadMap(from: change.tombstone as? [String: Any] ?? [:])
        outJSON?.pointee = cdCString(try cdEncodeJSON(payload))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_persistent_history_change_get_updated_properties_json")
public func cdPersistentHistoryChangeGetUpdatedPropertiesJSON(
    _ changePtr: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let changePtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing history change")
        }
        let change: NSPersistentHistoryChange = cdBorrow(changePtr)
        let names = (change.updatedProperties ?? []).map(\.name).sorted()
        outJSON?.pointee = cdCString(try cdEncodeJSON(names))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_persistent_history_change_get_transaction")
public func cdPersistentHistoryChangeGetTransaction(_ changePtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let changePtr else {
        return nil
    }
    let change: NSPersistentHistoryChange = cdBorrow(changePtr)
    return change.transaction.map(cdRetain)
}
