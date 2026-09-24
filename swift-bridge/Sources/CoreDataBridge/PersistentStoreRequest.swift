import CoreData
import CoreDataObjCBridge
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
        return cdErrorStatus(error)
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
        outRequest?.pointee = cdRetain(NSAsynchronousFetchRequest(fetchRequest: cdCopyFetchRequest(fetchRequest), completionBlock: nil))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return cdErrorStatus(error)
    }
}

@_cdecl("cd_asynchronous_fetch_request_get_fetch_request")
public func cdAsynchronousFetchRequestGetFetchRequest(_ requestPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let requestPtr else {
        return nil
    }
    let request: NSAsynchronousFetchRequest<NSFetchRequestResult> = cdBorrow(requestPtr)
    return cdRetain(cdCopyFetchRequest(request.fetchRequest))
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
    request.estimatedResultCount = Int(clamping: estimatedResultCount)
}

@_cdecl("cd_managed_object_context_execute_asynchronous_fetch_request")
public func cdManagedObjectContextExecuteAsynchronousFetchRequest(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ requestPtr: UnsafeMutableRawPointer?,
    _ outResult: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let request: NSAsynchronousFetchRequest<NSFetchRequestResult> = try cdArg(requestPtr, "asynchronous fetch request")
        let context: NSManagedObjectContext = try cdArg(contextPtr, "managed object context")
        let inner = request.fetchRequest
        try cdOnContextQueue(context) {
            try cdEnsureFetchable(inner, in: context)
        }
        let result = try cdExecuteRequest(contextPtr, request, as: NSAsynchronousFetchResult<NSFetchRequestResult>.self)
        outResult?.pointee = result.map { cdRetainContextBound($0, in: context) }
    }
}

func cdWithAsyncResult<T>(
    _ resultPtr: UnsafeMutableRawPointer?,
    _ body: (NSPersistentStoreAsynchronousResult) throws -> T
) throws -> T {
    guard let resultPtr else {
        throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing asynchronous result")
    }
    let box = cdBorrowBox(resultPtr)
    guard let result = box.object as? NSPersistentStoreAsynchronousResult else {
        throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Expected an asynchronous persistent store result")
    }
    if let bound = box as? CDContextBoundBox {
        return try cdOnContextQueue(bound.context) { try body(result) }
    }
    return try cdOnContextQueue(result.managedObjectContext) { try body(result) }
}

@_cdecl("cd_persistent_store_result_is_asynchronous")
public func cdPersistentStoreResultIsAsynchronous(_ resultPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let resultPtr else {
        return 0
    }
    return cdBorrowBox(resultPtr).object is NSPersistentStoreAsynchronousResult ? 1 : 0
}

@_cdecl("cd_persistent_store_async_result_get_managed_object_context")
public func cdPersistentStoreAsyncResultGetManagedObjectContext(_ resultPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let resultPtr, let result = cdBorrowBox(resultPtr).object as? NSPersistentStoreAsynchronousResult else {
        return nil
    }
    return cdRetain(result.managedObjectContext)
}

@_cdecl("cd_persistent_store_async_result_get_operation_error_json")
public func cdPersistentStoreAsyncResultGetOperationErrorJSON(
    _ resultPtr: UnsafeMutableRawPointer?,
    _ outJSON: CDOutString,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let operationError = try cdWithAsyncResult(resultPtr) { $0.operationError as NSError? }
        guard let operationError else {
            outJSON?.pointee = nil
            return
        }
        let payload = CDErrorPayload(domain: operationError.domain, code: operationError.code, message: operationError.localizedDescription)
        outJSON?.pointee = cdCString(try cdEncodeJSON(payload))
    }
}

@_cdecl("cd_persistent_store_async_result_get_progress_fraction_completed")
public func cdPersistentStoreAsyncResultGetProgressFractionCompleted(_ resultPtr: UnsafeMutableRawPointer?) -> Double {
    guard let resultPtr, let result = cdBorrowBox(resultPtr).object as? NSPersistentStoreAsynchronousResult else {
        return 0
    }
    return result.progress?.fractionCompleted ?? 0
}

@_cdecl("cd_persistent_store_async_result_has_progress")
public func cdPersistentStoreAsyncResultHasProgress(_ resultPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let resultPtr, let result = cdBorrowBox(resultPtr).object as? NSPersistentStoreAsynchronousResult else {
        return 0
    }
    return result.progress == nil ? 0 : 1
}

@_cdecl("cd_asynchronous_fetch_result_get_fetch_request")
public func cdAsynchronousFetchResultGetFetchRequest(_ resultPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let snapshot = try? cdWithAsyncResult(resultPtr) { result -> NSAsynchronousFetchRequest<NSFetchRequestResult> in
        guard let fetchResult = result as? NSAsynchronousFetchResult<NSFetchRequestResult> else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Expected an asynchronous fetch result")
        }
        return cdCopyAsynchronousFetchRequest(fetchResult.fetchRequest)
    }
    return snapshot.map(cdRetain)
}

@_cdecl("cd_asynchronous_fetch_result_get_final_result_count")
public func cdAsynchronousFetchResultGetFinalResultCount(
    _ resultPtr: UnsafeMutableRawPointer?,
    _ outCount: UnsafeMutablePointer<UInt64>?,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let count = try cdWithAsyncResult(resultPtr) { result -> Int in
            guard let fetchResult = result as? NSAsynchronousFetchResult<NSFetchRequestResult> else {
                throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Expected an asynchronous fetch result")
            }
            return fetchResult.finalResult?.count ?? 0
        }
        outCount?.pointee = UInt64(clamping: count)
    }
}

private let cdSaveChangesBoxesKey = UnsafeMutableRawPointer.allocate(byteCount: 1, alignment: 1)

final class CDSaveChangesBoxes: NSObject {
    let inserted: [CDManagedObjectBox]
    let updated: [CDManagedObjectBox]
    let deleted: [CDManagedObjectBox]
    let locked: [CDManagedObjectBox]

    init(inserted: [CDManagedObjectBox], updated: [CDManagedObjectBox], deleted: [CDManagedObjectBox], locked: [CDManagedObjectBox]) {
        self.inserted = inserted
        self.updated = updated
        self.deleted = deleted
        self.locked = locked
    }
}

func cdUniqueManagedObjectBoxes(
    _ pointers: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int32
) throws -> [CDManagedObjectBox] {
    guard count >= 0, count == 0 || pointers != nil else {
        throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Invalid managed object list")
    }
    var seen = Set<ObjectIdentifier>()
    var boxes: [CDManagedObjectBox] = []
    for index in 0..<Int(count) {
        let box = try cdManagedObjectBox(pointers?[index])
        if seen.insert(ObjectIdentifier(box.managedObject)).inserted {
            boxes.append(box)
        }
    }
    return boxes
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
    _ outRequest: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let boxes = CDSaveChangesBoxes(
            inserted: try cdUniqueManagedObjectBoxes(insertedObjectPtrs, insertedCount),
            updated: try cdUniqueManagedObjectBoxes(updatedObjectPtrs, updatedCount),
            deleted: try cdUniqueManagedObjectBoxes(deletedObjectPtrs, deletedCount),
            locked: try cdUniqueManagedObjectBoxes(lockedObjectPtrs, lockedCount)
        )
        func objects(_ list: [CDManagedObjectBox]) -> Set<NSManagedObject>? {
            list.isEmpty ? nil : Set(list.map(\.managedObject))
        }
        let request = NSSaveChangesRequest(
            inserted: objects(boxes.inserted),
            updated: objects(boxes.updated),
            deleted: objects(boxes.deleted),
            locked: objects(boxes.locked)
        )
        objc_setAssociatedObject(request, cdSaveChangesBoxesKey, boxes, .OBJC_ASSOCIATION_RETAIN)
        outRequest?.pointee = cdRetain(request)
    }
}

func cdCopySaveChangesRequest(_ request: NSSaveChangesRequest) -> NSSaveChangesRequest {
    let copy = NSSaveChangesRequest(
        inserted: request.insertedObjects,
        updated: request.updatedObjects,
        deleted: request.deletedObjects,
        locked: request.lockedObjects
    )
    if let boxes = objc_getAssociatedObject(request, cdSaveChangesBoxesKey) {
        objc_setAssociatedObject(copy, cdSaveChangesBoxesKey, boxes, .OBJC_ASSOCIATION_RETAIN)
    }
    copy.affectedStores = request.affectedStores
    return copy
}

private func cdSaveChangesBoxList(
    _ requestPtr: UnsafeMutableRawPointer?,
    _ select: (CDSaveChangesBoxes) -> [CDManagedObjectBox]
) -> UnsafeMutableRawPointer? {
    guard let requestPtr,
          let request = cdBorrowBox(requestPtr).object as? NSSaveChangesRequest,
          let boxes = objc_getAssociatedObject(request, cdSaveChangesBoxesKey) as? CDSaveChangesBoxes
    else {
        return nil
    }
    return cdRetain(select(boxes) as NSArray)
}

@_cdecl("cd_save_changes_request_get_inserted_objects")
public func cdSaveChangesRequestGetInsertedObjects(_ requestPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    cdSaveChangesBoxList(requestPtr) { $0.inserted }
}

@_cdecl("cd_save_changes_request_get_updated_objects")
public func cdSaveChangesRequestGetUpdatedObjects(_ requestPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    cdSaveChangesBoxList(requestPtr) { $0.updated }
}

@_cdecl("cd_save_changes_request_get_deleted_objects")
public func cdSaveChangesRequestGetDeletedObjects(_ requestPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    cdSaveChangesBoxList(requestPtr) { $0.deleted }
}

@_cdecl("cd_save_changes_request_get_locked_objects")
public func cdSaveChangesRequestGetLockedObjects(_ requestPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    cdSaveChangesBoxList(requestPtr) { $0.locked }
}

@_cdecl("cd_fetch_request_expression_new")
public func cdFetchRequestExpressionNew(
    _ fetchRequestPtr: UnsafeMutableRawPointer?,
    _ contextPtr: UnsafeMutableRawPointer?,
    _ countOnly: Int32,
    _ outExpression: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let fetchRequest: NSFetchRequest<NSFetchRequestResult> = try cdArg(fetchRequestPtr, "fetch request")
        let context: NSManagedObjectContext = try cdArg(contextPtr, "managed object context")
        let expression = NSFetchRequestExpression.expression(
            forFetch: NSExpression(forConstantValue: cdCopyFetchRequest(fetchRequest)),
            context: NSExpression(forConstantValue: context),
            countOnly: countOnly != 0
        )
        outExpression?.pointee = cdRetain(expression)
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
