import CoreData
import CoreDataObjCBridge
import Foundation

public typealias CDVoidCallback = @convention(c) (UnsafeMutableRawPointer?) -> Void

func cdContextConcurrencyType(_ raw: Int32) throws -> NSManagedObjectContextConcurrencyType {
    switch raw {
    case Int32(NSManagedObjectContextConcurrencyType.privateQueueConcurrencyType.rawValue):
        return .privateQueueConcurrencyType
    case Int32(NSManagedObjectContextConcurrencyType.mainQueueConcurrencyType.rawValue):
        return .mainQueueConcurrencyType
    default:
        throw cdBridgeNSError(
            code: CDR_INVALID_ARGUMENT,
            message: "Unsupported managed object context concurrency type \(raw); use private or main queue"
        )
    }
}

func cdEnsureSavable(_ context: NSManagedObjectContext) throws {
    if context.parent != nil {
        return
    }
    guard let coordinator = context.persistentStoreCoordinator else {
        throw cdBridgeNSError(
            code: CDR_INVALID_ARGUMENT,
            message: "The managed object context has no persistent store coordinator or parent context"
        )
    }
    if coordinator.persistentStores.isEmpty {
        throw cdBridgeNSError(
            code: CDR_INVALID_ARGUMENT,
            message: "The persistent store coordinator has no persistent stores; load or add a store before saving"
        )
    }
}

func cdEnsureFetchable(_ request: NSFetchRequest<NSFetchRequestResult>, in context: NSManagedObjectContext) throws {
    guard let coordinator = context.persistentStoreCoordinator else {
        throw cdBridgeNSError(
            code: CDR_INVALID_ARGUMENT,
            message: "The managed object context has no persistent store coordinator or parent context"
        )
    }
    guard let entityName = request.entityName else {
        throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "The fetch request has no entity")
    }
    guard coordinator.managedObjectModel.entitiesByName[entityName] != nil else {
        throw cdBridgeNSError(
            code: CDR_INVALID_ARGUMENT,
            message: "The managed object model has no entity named '\(entityName)'"
        )
    }
}

func cdFetch(_ request: NSFetchRequest<NSFetchRequestResult>, in context: NSManagedObjectContext) throws -> [Any] {
    try cdEnsureFetchable(request, in: context)
    return try cdObjCValue { CDXExecuteFetchRequest(context, request, $0) }
}

@_cdecl("cd_managed_object_context_new")
public func cdManagedObjectContextNew(
    _ concurrencyType: Int32,
    _ outContext: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let type = try cdContextConcurrencyType(concurrencyType)
        outContext?.pointee = cdRetain(NSManagedObjectContext(concurrencyType: type))
    }
}

@_cdecl("cd_managed_object_context_get_name")
public func cdManagedObjectContextGetName(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outName: CDOutString,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let name = try cdWithContext(contextPtr) { $0.name }
        outName?.pointee = name.flatMap { cdCString($0) }
    }
}

@_cdecl("cd_managed_object_context_set_name")
public func cdManagedObjectContextSetName(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let value = cdOptionalString(name)
        try cdWithContext(contextPtr) { $0.name = value }
    }
}

@_cdecl("cd_managed_object_context_get_parent_context")
public func cdManagedObjectContextGetParentContext(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outParent: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let parent = try cdWithContext(contextPtr) { $0.parent }
        outParent?.pointee = parent.map(cdRetain)
    }
}

@_cdecl("cd_managed_object_context_set_parent_context")
public func cdManagedObjectContextSetParentContext(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ parentContextPtr: UnsafeMutableRawPointer?,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let parent: NSManagedObjectContext? = try parentContextPtr.map { try cdArg($0, "parent managed object context") }
        try cdWithContext(contextPtr) { context in
            if let parent {
                try cdEnsureQueueContext(parent)
                var ancestor: NSManagedObjectContext? = parent
                while let current = ancestor {
                    guard current !== context else {
                        throw cdBridgeNSError(
                            code: CDR_INVALID_ARGUMENT,
                            message: "A managed object context cannot be its own ancestor"
                        )
                    }
                    ancestor = current.parent
                }
            }
            try cdTrySetValue(parent, forKey: "parentContext", on: context)
        }
    }
}

@_cdecl("cd_managed_object_context_get_concurrency_type")
public func cdManagedObjectContextGetConcurrencyType(_ contextPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let contextPtr, let context = cdBorrowBox(contextPtr).object as? NSManagedObjectContext else {
        return 0
    }
    return Int32(clamping: context.concurrencyType.rawValue)
}

@_cdecl("cd_managed_object_context_set_persistent_store_coordinator")
public func cdManagedObjectContextSetPersistentStoreCoordinator(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ coordinatorPtr: UnsafeMutableRawPointer?,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let coordinator: NSPersistentStoreCoordinator = try cdArg(coordinatorPtr, "persistent store coordinator")
        try cdWithContext(contextPtr) { context in
            try cdTrySetValue(coordinator, forKey: "persistentStoreCoordinator", on: context)
        }
    }
}

@_cdecl("cd_managed_object_context_has_changes")
public func cdManagedObjectContextHasChanges(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<Int32>?,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let value = try cdWithContext(contextPtr) { $0.hasChanges }
        outValue?.pointee = value ? 1 : 0
    }
}

@_cdecl("cd_managed_object_context_get_inserted_objects")
public func cdManagedObjectContextGetInsertedObjects(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outArray: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        outArray?.pointee = try cdWithContext(contextPtr) { context in
            try cdRetainManagedObjects(Array(context.insertedObjects), in: context)
        }
    }
}

@_cdecl("cd_managed_object_context_get_updated_objects")
public func cdManagedObjectContextGetUpdatedObjects(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outArray: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        outArray?.pointee = try cdWithContext(contextPtr) { context in
            try cdRetainManagedObjects(Array(context.updatedObjects), in: context)
        }
    }
}

@_cdecl("cd_managed_object_context_get_deleted_objects")
public func cdManagedObjectContextGetDeletedObjects(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outArray: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        outArray?.pointee = try cdWithContext(contextPtr) { context in
            try cdRetainManagedObjects(Array(context.deletedObjects), in: context)
        }
    }
}

@_cdecl("cd_managed_object_context_get_registered_objects")
public func cdManagedObjectContextGetRegisteredObjects(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outArray: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        outArray?.pointee = try cdWithContext(contextPtr) { context in
            try cdRetainManagedObjects(Array(context.registeredObjects), in: context)
        }
    }
}

@_cdecl("cd_managed_object_context_save")
public func cdManagedObjectContextSave(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        try cdWithContext(contextPtr) { context in
            guard context.hasChanges else {
                return
            }
            try cdEnsureSavable(context)
            try cdObjCCall { CDXSave(context, $0) }
        }
    }
}

@_cdecl("cd_managed_object_context_insert_object")
public func cdManagedObjectContextInsertObject(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ objectPtr: UnsafeMutableRawPointer?,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let box = try cdManagedObjectBox(objectPtr)
        try cdWithContext(contextPtr) { context in
            cdUnboundObjectLock.lock()
            defer { cdUnboundObjectLock.unlock() }
            if let bound = box.boundContextLocked {
                guard bound === context else {
                    throw cdBridgeNSError(
                        code: CDR_INVALID_ARGUMENT,
                        message: "The managed object belongs to a different managed object context"
                    )
                }
                try cdEnsureRegistered(box.managedObject, in: context)
            }
            try cdObjCCall { CDXInsertObject(context, box.managedObject, $0) }
            box.bindLocked(context)
        }
    }
}

@_cdecl("cd_managed_object_context_delete_object")
public func cdManagedObjectContextDeleteObject(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ objectPtr: UnsafeMutableRawPointer?,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        try cdWithContext(contextPtr) { context in
            let object = try cdObjectRegistered(objectPtr, in: context)
            try cdObjCCall { CDXDeleteObject(context, object, $0) }
        }
    }
}

@_cdecl("cd_managed_object_context_object_registered_for_id")
public func cdManagedObjectContextObjectRegisteredForID(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ objectIDPtr: UnsafeMutableRawPointer?,
    _ outObject: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let objectID: NSManagedObjectID = try cdArg(objectIDPtr, "managed object ID")
        outObject?.pointee = try cdWithContext(contextPtr) { context in
            context.registeredObject(for: objectID).map { cdRetainManagedObject($0, in: context) }
        }
    }
}

@_cdecl("cd_managed_object_context_object_with_id")
public func cdManagedObjectContextObjectWithID(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ objectIDPtr: UnsafeMutableRawPointer?,
    _ outObject: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let objectID: NSManagedObjectID = try cdArg(objectIDPtr, "managed object ID")
        outObject?.pointee = try cdWithContext(contextPtr) { context in
            let object = try cdObjCValue { CDXObjectWithID(context, objectID, $0) }
            return cdRetainManagedObject(object, in: context)
        }
    }
}

@_cdecl("cd_managed_object_context_existing_object_with_id")
public func cdManagedObjectContextExistingObjectWithID(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ objectIDPtr: UnsafeMutableRawPointer?,
    _ outObject: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let objectID: NSManagedObjectID = try cdArg(objectIDPtr, "managed object ID")
        outObject?.pointee = try cdWithContext(contextPtr) { context in
            let object = try cdObjCValue { CDXExistingObjectWithID(context, objectID, $0) }
            return cdRetainManagedObject(object, in: context)
        }
    }
}

@_cdecl("cd_managed_object_context_perform")
public func cdManagedObjectContextPerform(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ callback: @escaping CDVoidCallback,
    _ refcon: UnsafeMutableRawPointer?,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let context: NSManagedObjectContext = try cdArg(contextPtr, "managed object context")
        try cdEnsureQueueContext(context)
        context.perform {
            callback(refcon)
        }
    }
}

@_cdecl("cd_managed_object_context_perform_and_wait")
public func cdManagedObjectContextPerformAndWait(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ callback: CDVoidCallback,
    _ refcon: UnsafeMutableRawPointer?,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let context: NSManagedObjectContext = try cdArg(contextPtr, "managed object context")
        try cdEnsureQueueContext(context)
        context.performAndWait {
            callback(refcon)
        }
    }
}

@_cdecl("cd_managed_object_context_execute_fetch_request")
public func cdManagedObjectContextExecuteFetchRequest(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ requestPtr: UnsafeMutableRawPointer?,
    _ outArray: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let request: NSFetchRequest<NSFetchRequestResult> = try cdArg(requestPtr, "fetch request")
        outArray?.pointee = try cdWithContext(contextPtr) { context in
            try cdRetainManagedObjects(try cdFetch(request, in: context), in: context)
        }
    }
}

@_cdecl("cd_managed_object_context_count_for_fetch_request")
public func cdManagedObjectContextCountForFetchRequest(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ requestPtr: UnsafeMutableRawPointer?,
    _ outCount: UnsafeMutablePointer<UInt64>?,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let request: NSFetchRequest<NSFetchRequestResult> = try cdArg(requestPtr, "fetch request")
        let count = try cdWithContext(contextPtr) { context -> UInt in
            try cdEnsureFetchable(request, in: context)
            var count: UInt = 0
            try cdObjCCall { CDXCountForFetchRequest(context, request, &count, $0) }
            return count
        }
        outCount?.pointee = UInt64(count)
    }
}

@_cdecl("cd_managed_object_context_refresh_object")
public func cdManagedObjectContextRefreshObject(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ objectPtr: UnsafeMutableRawPointer?,
    _ mergeChanges: Int32,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        try cdWithContext(contextPtr) { context in
            let object = try cdObjectRegistered(objectPtr, in: context)
            try cdObjCCall { CDXRefreshObject(context, object, mergeChanges != 0, $0) }
        }
    }
}

@_cdecl("cd_managed_object_context_process_pending_changes")
public func cdManagedObjectContextProcessPendingChanges(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        try cdWithContext(contextPtr) { $0.processPendingChanges() }
    }
}

@_cdecl("cd_managed_object_context_reset")
public func cdManagedObjectContextReset(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        try cdWithContext(contextPtr) { $0.reset() }
    }
}

@_cdecl("cd_managed_object_context_rollback")
public func cdManagedObjectContextRollback(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        try cdWithContext(contextPtr) { $0.rollback() }
    }
}

@_cdecl("cd_managed_object_context_refresh_all_objects")
public func cdManagedObjectContextRefreshAllObjects(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        try cdWithContext(contextPtr) { $0.refreshAllObjects() }
    }
}

@_cdecl("cd_managed_object_context_get_automatically_merges_changes_from_parent")
public func cdManagedObjectContextGetAutomaticallyMergesChangesFromParent(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<Int32>?,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let value = try cdWithContext(contextPtr) { $0.automaticallyMergesChangesFromParent }
        outValue?.pointee = value ? 1 : 0
    }
}

@_cdecl("cd_managed_object_context_set_automatically_merges_changes_from_parent")
public func cdManagedObjectContextSetAutomaticallyMergesChangesFromParent(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ automaticallyMerges: Int32,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        try cdWithContext(contextPtr) { $0.automaticallyMergesChangesFromParent = automaticallyMerges != 0 }
    }
}

@_cdecl("cd_managed_object_context_get_transaction_author")
public func cdManagedObjectContextGetTransactionAuthor(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outAuthor: CDOutString,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let author = try cdWithContext(contextPtr) { $0.transactionAuthor }
        outAuthor?.pointee = author.flatMap { cdCString($0) }
    }
}

@_cdecl("cd_managed_object_context_set_transaction_author")
public func cdManagedObjectContextSetTransactionAuthor(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ author: UnsafePointer<CChar>?,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let value = cdOptionalString(author)
        try cdWithContext(contextPtr) { $0.transactionAuthor = value }
    }
}

@_cdecl("cd_managed_object_context_obtain_permanent_ids")
public func cdManagedObjectContextObtainPermanentIDs(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ objectPtrs: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int32,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        guard count >= 0, count == 0 || objectPtrs != nil else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Invalid managed object list")
        }
        let pointers = (0..<Int(count)).map { objectPtrs?[$0] }
        try cdWithContext(contextPtr) { context in
            let objects = try pointers.map { try cdObjectRegistered($0, in: context) }
            try cdObjCCall { CDXObtainPermanentIDs(context, objects, $0) }
        }
    }
}

@_cdecl("cd_managed_object_context_merge_changes_from_history_transaction")
public func cdManagedObjectContextMergeChangesFromHistoryTransaction(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ transactionPtr: UnsafeMutableRawPointer?,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let transaction: NSPersistentHistoryTransaction = try cdArg(transactionPtr, "history transaction")
        try cdWithContext(contextPtr) { context in
            let notification = transaction.objectIDNotification()
            try cdObjCCall { CDXMergeChangesFromContextDidSave(context, notification, $0) }
        }
    }
}
