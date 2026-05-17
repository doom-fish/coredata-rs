import CoreData
import Foundation

@_cdecl("cd_persistent_store_request_get_affected_stores")
public func cdPersistentStoreRequestGetAffectedStores(_ requestPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let requestPtr else {
        return nil
    }
    let request: NSPersistentStoreRequest = cdBorrow(requestPtr)
    guard let stores = request.affectedStores else {
        return nil
    }
    return cdRetain(stores as NSArray)
}

@_cdecl("cd_persistent_store_request_set_affected_stores")
public func cdPersistentStoreRequestSetAffectedStores(
    _ requestPtr: UnsafeMutableRawPointer?,
    _ storePtrs: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let requestPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing persistent store request")
        }
        let request: NSPersistentStoreRequest = cdBorrow(requestPtr)
        request.affectedStores = cdObjects(from: storePtrs, count: count)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_persistent_store_request_get_request_type")
public func cdPersistentStoreRequestGetRequestType(_ requestPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let requestPtr else {
        return 0
    }
    let request: NSPersistentStoreRequest = cdBorrow(requestPtr)
    return UInt64(request.requestType.rawValue)
}

@_cdecl("cd_asynchronous_fetch_request_new")
public func cdAsynchronousFetchRequestNew(
    _ fetchRequestPtr: UnsafeMutableRawPointer?,
    _ outRequest: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let fetchRequestPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing fetch request")
        }
        let fetchRequest: NSFetchRequest<NSFetchRequestResult> = cdBorrow(fetchRequestPtr)
        outRequest?.pointee = cdRetain(NSAsynchronousFetchRequest(fetchRequest: fetchRequest, completionBlock: nil))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_asynchronous_fetch_request_get_fetch_request")
public func cdAsynchronousFetchRequestGetFetchRequest(_ requestPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let requestPtr else {
        return nil
    }
    let request: NSAsynchronousFetchRequest<NSFetchRequestResult> = cdBorrow(requestPtr)
    return cdRetain(request.fetchRequest)
}

@_cdecl("cd_asynchronous_fetch_request_get_estimated_result_count")
public func cdAsynchronousFetchRequestGetEstimatedResultCount(_ requestPtr: UnsafeMutableRawPointer?) -> Int64 {
    guard let requestPtr else {
        return 0
    }
    let request: NSAsynchronousFetchRequest<NSFetchRequestResult> = cdBorrow(requestPtr)
    return Int64(request.estimatedResultCount)
}

@_cdecl("cd_asynchronous_fetch_request_set_estimated_result_count")
public func cdAsynchronousFetchRequestSetEstimatedResultCount(
    _ requestPtr: UnsafeMutableRawPointer?,
    _ estimatedResultCount: Int64
) {
    guard let requestPtr else {
        return
    }
    let request: NSAsynchronousFetchRequest<NSFetchRequestResult> = cdBorrow(requestPtr)
    request.estimatedResultCount = Int(estimatedResultCount)
}

@_cdecl("cd_managed_object_context_execute_asynchronous_fetch_request")
public func cdManagedObjectContextExecuteAsynchronousFetchRequest(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ requestPtr: UnsafeMutableRawPointer?,
    _ outResult: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let contextPtr, let requestPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object context or asynchronous fetch request")
        }
        let context: NSManagedObjectContext = cdBorrow(contextPtr)
        let request: NSAsynchronousFetchRequest<NSFetchRequestResult> = cdBorrow(requestPtr)
        let result = try context.execute(request) as? NSAsynchronousFetchResult<NSFetchRequestResult>
        outResult?.pointee = result.map(cdRetain)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_persistent_store_async_result_get_managed_object_context")
public func cdPersistentStoreAsyncResultGetManagedObjectContext(_ resultPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let resultPtr else {
        return nil
    }
    let result: NSPersistentStoreAsynchronousResult = cdBorrow(resultPtr)
    return cdRetain(result.managedObjectContext)
}

@_cdecl("cd_persistent_store_async_result_get_operation_error_json")
public func cdPersistentStoreAsyncResultGetOperationErrorJSON(_ resultPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let resultPtr else {
        return nil
    }
    let result: NSPersistentStoreAsynchronousResult = cdBorrow(resultPtr)
    guard let error = result.operationError as NSError? else {
        return nil
    }
    let payload = CDErrorPayload(domain: error.domain, code: error.code, message: error.localizedDescription)
    return (try? cdEncodeJSON(payload)).flatMap(cdCString)
}

@_cdecl("cd_persistent_store_async_result_get_progress_fraction_completed")
public func cdPersistentStoreAsyncResultGetProgressFractionCompleted(_ resultPtr: UnsafeMutableRawPointer?) -> Double {
    guard let resultPtr else {
        return 0
    }
    let result: NSPersistentStoreAsynchronousResult = cdBorrow(resultPtr)
    return result.progress?.fractionCompleted ?? 0
}

@_cdecl("cd_persistent_store_async_result_has_progress")
public func cdPersistentStoreAsyncResultHasProgress(_ resultPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let resultPtr else {
        return 0
    }
    let result: NSPersistentStoreAsynchronousResult = cdBorrow(resultPtr)
    return result.progress == nil ? 0 : 1
}

@_cdecl("cd_asynchronous_fetch_result_get_fetch_request")
public func cdAsynchronousFetchResultGetFetchRequest(_ resultPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let resultPtr else {
        return nil
    }
    let result: NSAsynchronousFetchResult<NSFetchRequestResult> = cdBorrow(resultPtr)
    return cdRetain(result.fetchRequest)
}

@_cdecl("cd_asynchronous_fetch_result_get_final_result_count")
public func cdAsynchronousFetchResultGetFinalResultCount(_ resultPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let resultPtr else {
        return 0
    }
    let result: NSAsynchronousFetchResult<NSFetchRequestResult> = cdBorrow(resultPtr)
    return UInt64(result.finalResult?.count ?? 0)
}

@_cdecl("cd_save_changes_request_new")
public func cdSaveChangesRequestNew(
    _ insertedObjectPtrs: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ insertedCount: Int32,
    _ updatedObjectPtrs: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ updatedCount: Int32,
    _ deletedObjectPtrs: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ deletedCount: Int32,
    _ lockedObjectPtrs: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ lockedCount: Int32,
    _ outRequest: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let inserted: Set<NSManagedObject> = Set(cdObjects(from: insertedObjectPtrs, count: insertedCount))
        let updated: Set<NSManagedObject> = Set(cdObjects(from: updatedObjectPtrs, count: updatedCount))
        let deleted: Set<NSManagedObject> = Set(cdObjects(from: deletedObjectPtrs, count: deletedCount))
        let locked: Set<NSManagedObject> = Set(cdObjects(from: lockedObjectPtrs, count: lockedCount))
        outRequest?.pointee = cdRetain(NSSaveChangesRequest(
            inserted: inserted.isEmpty ? nil : inserted,
            updated: updated.isEmpty ? nil : updated,
            deleted: deleted.isEmpty ? nil : deleted,
            locked: locked.isEmpty ? nil : locked
        ))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

private func cdRetainedManagedObjectArray(_ objects: Set<NSManagedObject>?) -> UnsafeMutableRawPointer? {
    guard let objects else {
        return nil
    }
    return cdRetain(Array(objects) as NSArray)
}

@_cdecl("cd_save_changes_request_get_inserted_objects")
public func cdSaveChangesRequestGetInsertedObjects(_ requestPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let requestPtr else {
        return nil
    }
    let request: NSSaveChangesRequest = cdBorrow(requestPtr)
    return cdRetainedManagedObjectArray(request.insertedObjects)
}

@_cdecl("cd_save_changes_request_get_updated_objects")
public func cdSaveChangesRequestGetUpdatedObjects(_ requestPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let requestPtr else {
        return nil
    }
    let request: NSSaveChangesRequest = cdBorrow(requestPtr)
    return cdRetainedManagedObjectArray(request.updatedObjects)
}

@_cdecl("cd_save_changes_request_get_deleted_objects")
public func cdSaveChangesRequestGetDeletedObjects(_ requestPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let requestPtr else {
        return nil
    }
    let request: NSSaveChangesRequest = cdBorrow(requestPtr)
    return cdRetainedManagedObjectArray(request.deletedObjects)
}

@_cdecl("cd_save_changes_request_get_locked_objects")
public func cdSaveChangesRequestGetLockedObjects(_ requestPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let requestPtr else {
        return nil
    }
    let request: NSSaveChangesRequest = cdBorrow(requestPtr)
    return cdRetainedManagedObjectArray(request.lockedObjects)
}

@_cdecl("cd_fetch_request_expression_new")
public func cdFetchRequestExpressionNew(
    _ fetchRequestPtr: UnsafeMutableRawPointer?,
    _ contextPtr: UnsafeMutableRawPointer?,
    _ countOnly: Int32,
    _ outExpression: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let fetchRequestPtr, let contextPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing fetch request or managed object context")
        }
        let fetchRequest: NSFetchRequest<NSFetchRequestResult> = cdBorrow(fetchRequestPtr)
        let context: NSManagedObjectContext = cdBorrow(contextPtr)
        let expression = NSFetchRequestExpression.expression(
            forFetch: NSExpression(forConstantValue: fetchRequest),
            context: NSExpression(forConstantValue: context),
            countOnly: countOnly != 0
        )
        outExpression?.pointee = cdRetain(expression)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_fetch_request_expression_get_count_only_request")
public func cdFetchRequestExpressionGetCountOnlyRequest(_ expressionPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let expressionPtr else {
        return 0
    }
    let expression: NSFetchRequestExpression = cdBorrow(expressionPtr)
    return expression.isCountOnlyRequest ? 1 : 0
}
