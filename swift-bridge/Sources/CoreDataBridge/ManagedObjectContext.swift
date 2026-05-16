import CoreData
import Foundation

public typealias CDVoidCallback = @convention(c) (UnsafeMutableRawPointer?) -> Void

@_cdecl("cd_managed_object_context_new")
public func cdManagedObjectContextNew(
    _ concurrencyType: Int32,
    _ outContext: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let concurrencyType = NSManagedObjectContextConcurrencyType(rawValue: UInt(concurrencyType)) else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Invalid managed object context concurrency type")
        }
        outContext?.pointee = cdRetain(NSManagedObjectContext(concurrencyType: concurrencyType))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_managed_object_context_get_name")
public func cdManagedObjectContextGetName(_ contextPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let contextPtr else {
        return nil
    }
    let context: NSManagedObjectContext = cdBorrow(contextPtr)
    return context.name.flatMap { cdCString($0) }
}

@_cdecl("cd_managed_object_context_set_name")
public func cdManagedObjectContextSetName(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let contextPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object context")
        }
        let context: NSManagedObjectContext = cdBorrow(contextPtr)
        context.name = cdOptionalString(name)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_managed_object_context_get_parent_context")
public func cdManagedObjectContextGetParentContext(_ contextPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let contextPtr else {
        return nil
    }
    let context: NSManagedObjectContext = cdBorrow(contextPtr)
    return context.parent.map(cdRetain)
}

@_cdecl("cd_managed_object_context_set_parent_context")
public func cdManagedObjectContextSetParentContext(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ parentContextPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let contextPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object context")
        }
        let context: NSManagedObjectContext = cdBorrow(contextPtr)
        context.parent = parentContextPtr.map { cdBorrow($0) as NSManagedObjectContext }
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_managed_object_context_get_concurrency_type")
public func cdManagedObjectContextGetConcurrencyType(_ contextPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let contextPtr else {
        return 0
    }
    let context: NSManagedObjectContext = cdBorrow(contextPtr)
    return Int32(context.concurrencyType.rawValue)
}

@_cdecl("cd_managed_object_context_set_persistent_store_coordinator")
public func cdManagedObjectContextSetPersistentStoreCoordinator(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ coordinatorPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let contextPtr, let coordinatorPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object context or persistent store coordinator")
        }
        let context: NSManagedObjectContext = cdBorrow(contextPtr)
        let coordinator: NSPersistentStoreCoordinator = cdBorrow(coordinatorPtr)
        context.persistentStoreCoordinator = coordinator
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_managed_object_context_has_changes")
public func cdManagedObjectContextHasChanges(_ contextPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let contextPtr else {
        return 0
    }
    let context: NSManagedObjectContext = cdBorrow(contextPtr)
    return context.hasChanges ? 1 : 0
}

@_cdecl("cd_managed_object_context_get_inserted_objects")
public func cdManagedObjectContextGetInsertedObjects(_ contextPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let contextPtr else {
        return nil
    }
    let context: NSManagedObjectContext = cdBorrow(contextPtr)
    return cdRetain(Array(context.insertedObjects) as NSArray)
}

@_cdecl("cd_managed_object_context_get_updated_objects")
public func cdManagedObjectContextGetUpdatedObjects(_ contextPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let contextPtr else {
        return nil
    }
    let context: NSManagedObjectContext = cdBorrow(contextPtr)
    return cdRetain(Array(context.updatedObjects) as NSArray)
}

@_cdecl("cd_managed_object_context_get_deleted_objects")
public func cdManagedObjectContextGetDeletedObjects(_ contextPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let contextPtr else {
        return nil
    }
    let context: NSManagedObjectContext = cdBorrow(contextPtr)
    return cdRetain(Array(context.deletedObjects) as NSArray)
}

@_cdecl("cd_managed_object_context_get_registered_objects")
public func cdManagedObjectContextGetRegisteredObjects(_ contextPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let contextPtr else {
        return nil
    }
    let context: NSManagedObjectContext = cdBorrow(contextPtr)
    return cdRetain(Array(context.registeredObjects) as NSArray)
}

@_cdecl("cd_managed_object_context_save")
public func cdManagedObjectContextSave(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let contextPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object context")
        }
        let context: NSManagedObjectContext = cdBorrow(contextPtr)
        try context.save()
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_managed_object_context_insert_object")
public func cdManagedObjectContextInsertObject(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ objectPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let contextPtr, let objectPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object context or object")
        }
        let context: NSManagedObjectContext = cdBorrow(contextPtr)
        let object: NSManagedObject = cdBorrow(objectPtr)
        context.insert(object)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_managed_object_context_delete_object")
public func cdManagedObjectContextDeleteObject(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ objectPtr: UnsafeMutableRawPointer?
) {
    guard let contextPtr, let objectPtr else {
        return
    }
    let context: NSManagedObjectContext = cdBorrow(contextPtr)
    let object: NSManagedObject = cdBorrow(objectPtr)
    context.delete(object)
}

@_cdecl("cd_managed_object_context_object_registered_for_id")
public func cdManagedObjectContextObjectRegisteredForID(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ objectIDPtr: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let contextPtr, let objectIDPtr else {
        return nil
    }
    let context: NSManagedObjectContext = cdBorrow(contextPtr)
    let objectID: NSManagedObjectID = cdBorrow(objectIDPtr)
    return context.registeredObjects.first { $0.objectID == objectID }.map(cdRetain)
}

@_cdecl("cd_managed_object_context_object_with_id")
public func cdManagedObjectContextObjectWithID(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ objectIDPtr: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let contextPtr, let objectIDPtr else {
        return nil
    }
    let context: NSManagedObjectContext = cdBorrow(contextPtr)
    let objectID: NSManagedObjectID = cdBorrow(objectIDPtr)
    return cdRetain(context.object(with: objectID))
}

@_cdecl("cd_managed_object_context_existing_object_with_id")
public func cdManagedObjectContextExistingObjectWithID(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ objectIDPtr: UnsafeMutableRawPointer?,
    _ outObject: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let contextPtr, let objectIDPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object context or object ID")
        }
        let context: NSManagedObjectContext = cdBorrow(contextPtr)
        let objectID: NSManagedObjectID = cdBorrow(objectIDPtr)
        let object = try context.existingObject(with: objectID)
        outObject?.pointee = cdRetain(object)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_managed_object_context_perform")
public func cdManagedObjectContextPerform(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ callback: @escaping CDVoidCallback,
    _ refcon: UnsafeMutableRawPointer?
) {
    guard let contextPtr else {
        return
    }
    let context: NSManagedObjectContext = cdBorrow(contextPtr)
    context.perform {
        callback(refcon)
    }
}

@_cdecl("cd_managed_object_context_perform_and_wait")
public func cdManagedObjectContextPerformAndWait(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ callback: @escaping CDVoidCallback,
    _ refcon: UnsafeMutableRawPointer?
) {
    guard let contextPtr else {
        return
    }
    let context: NSManagedObjectContext = cdBorrow(contextPtr)
    context.performAndWait {
        callback(refcon)
    }
}

@_cdecl("cd_managed_object_context_execute_fetch_request")
public func cdManagedObjectContextExecuteFetchRequest(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ requestPtr: UnsafeMutableRawPointer?,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let contextPtr, let requestPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object context or fetch request")
        }
        let context: NSManagedObjectContext = cdBorrow(contextPtr)
        let request: NSFetchRequest<NSManagedObject> = cdBorrow(requestPtr)
        let results = try context.fetch(request)
        outArray?.pointee = cdRetain(results as NSArray)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_managed_object_context_count_for_fetch_request")
public func cdManagedObjectContextCountForFetchRequest(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ requestPtr: UnsafeMutableRawPointer?,
    _ outCount: UnsafeMutablePointer<UInt64>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let contextPtr, let requestPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object context or fetch request")
        }
        let context: NSManagedObjectContext = cdBorrow(contextPtr)
        let request: NSFetchRequest<NSFetchRequestResult> = cdBorrow(requestPtr)
        outCount?.pointee = UInt64(try context.count(for: request))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_managed_object_context_refresh_object")
public func cdManagedObjectContextRefreshObject(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ objectPtr: UnsafeMutableRawPointer?,
    _ mergeChanges: Int32
) {
    guard let contextPtr, let objectPtr else {
        return
    }
    let context: NSManagedObjectContext = cdBorrow(contextPtr)
    let object: NSManagedObject = cdBorrow(objectPtr)
    context.refresh(object, mergeChanges: mergeChanges != 0)
}

@_cdecl("cd_managed_object_context_process_pending_changes")
public func cdManagedObjectContextProcessPendingChanges(_ contextPtr: UnsafeMutableRawPointer?) {
    guard let contextPtr else {
        return
    }
    let context: NSManagedObjectContext = cdBorrow(contextPtr)
    context.processPendingChanges()
}

@_cdecl("cd_managed_object_context_reset")
public func cdManagedObjectContextReset(_ contextPtr: UnsafeMutableRawPointer?) {
    guard let contextPtr else {
        return
    }
    let context: NSManagedObjectContext = cdBorrow(contextPtr)
    context.reset()
}

@_cdecl("cd_managed_object_context_rollback")
public func cdManagedObjectContextRollback(_ contextPtr: UnsafeMutableRawPointer?) {
    guard let contextPtr else {
        return
    }
    let context: NSManagedObjectContext = cdBorrow(contextPtr)
    context.rollback()
}

@_cdecl("cd_managed_object_context_refresh_all_objects")
public func cdManagedObjectContextRefreshAllObjects(_ contextPtr: UnsafeMutableRawPointer?) {
    guard let contextPtr else {
        return
    }
    let context: NSManagedObjectContext = cdBorrow(contextPtr)
    context.refreshAllObjects()
}

@_cdecl("cd_managed_object_context_get_automatically_merges_changes_from_parent")
public func cdManagedObjectContextGetAutomaticallyMergesChangesFromParent(_ contextPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let contextPtr else {
        return 0
    }
    let context: NSManagedObjectContext = cdBorrow(contextPtr)
    return context.automaticallyMergesChangesFromParent ? 1 : 0
}

@_cdecl("cd_managed_object_context_set_automatically_merges_changes_from_parent")
public func cdManagedObjectContextSetAutomaticallyMergesChangesFromParent(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ automaticallyMerges: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let contextPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object context")
        }
        let context: NSManagedObjectContext = cdBorrow(contextPtr)
        context.automaticallyMergesChangesFromParent = automaticallyMerges != 0
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_managed_object_context_get_transaction_author")
public func cdManagedObjectContextGetTransactionAuthor(_ contextPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let contextPtr else {
        return nil
    }
    let context: NSManagedObjectContext = cdBorrow(contextPtr)
    return context.transactionAuthor.flatMap { cdCString($0) }
}

@_cdecl("cd_managed_object_context_set_transaction_author")
public func cdManagedObjectContextSetTransactionAuthor(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ author: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let contextPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object context")
        }
        let context: NSManagedObjectContext = cdBorrow(contextPtr)
        context.transactionAuthor = cdOptionalString(author)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_managed_object_context_obtain_permanent_ids")
public func cdManagedObjectContextObtainPermanentIDs(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ objectPtrs: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let contextPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object context")
        }
        let context: NSManagedObjectContext = cdBorrow(contextPtr)
        let objects: [NSManagedObject] = cdObjects(from: objectPtrs, count: count)
        try context.obtainPermanentIDs(for: objects)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_managed_object_context_merge_changes_from_history_transaction")
public func cdManagedObjectContextMergeChangesFromHistoryTransaction(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ transactionPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let contextPtr, let transactionPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object context or history transaction")
        }
        let context: NSManagedObjectContext = cdBorrow(contextPtr)
        let transaction: NSPersistentHistoryTransaction = cdBorrow(transactionPtr)
        context.mergeChanges(fromContextDidSave: transaction.objectIDNotification())
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}
