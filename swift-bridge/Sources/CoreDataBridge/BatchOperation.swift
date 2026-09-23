import CoreData
import CoreDataObjCBridge
import Foundation

func cdEnsureSQLiteStores(_ context: NSManagedObjectContext, _ what: String) throws {
    guard let coordinator = context.persistentStoreCoordinator, !coordinator.persistentStores.isEmpty else {
        throw cdBridgeNSError(
            code: CDR_INVALID_ARGUMENT,
            message: "\(what) requires a persistent store coordinator with at least one store"
        )
    }
    guard coordinator.persistentStores.allSatisfy({ $0.type == NSSQLiteStoreType }) else {
        throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "\(what) is only supported by SQLite stores")
    }
}

func cdSnapshotRequest(_ request: NSPersistentStoreRequest) throws -> NSPersistentStoreRequest {
    if let insert = request as? NSBatchInsertRequest {
        let snapshot = NSBatchInsertRequest(entityName: insert.entityName, objects: insert.objectsToInsert ?? [])
        snapshot.resultType = insert.resultType
        snapshot.affectedStores = insert.affectedStores
        return snapshot
    }
    if let update = request as? NSBatchUpdateRequest {
        let snapshot = NSBatchUpdateRequest(entityName: update.entityName)
        snapshot.predicate = update.predicate
        snapshot.includesSubentities = update.includesSubentities
        snapshot.resultType = update.resultType
        snapshot.propertiesToUpdate = update.propertiesToUpdate
        snapshot.affectedStores = update.affectedStores
        return snapshot
    }
    let copy = try cdObjCValue { CDXCopyObject(request, $0) }
    guard let snapshot = copy as? NSPersistentStoreRequest else {
        throw cdBridgeNSError(code: CDR_FAILURE, message: "Could not copy the persistent store request")
    }
    return snapshot
}

func cdExecuteRequest<T>(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ request: NSPersistentStoreRequest,
    as _: T.Type,
    requireSQLite what: String? = nil
) throws -> T? {
    try cdWithContext(contextPtr) { context in
        if let what {
            try cdEnsureSQLiteStores(context, what)
        }
        let result = try cdObjCValue { CDXExecuteRequest(context, request, $0) }
        return result as? T
    }
}

func cdFoundationObjectDictionaries(from json: UnsafePointer<CChar>?) throws -> [[String: Any]] {
    let payloads = try cdDecodeJSON(json, as: [[String: CDValuePayload]].self)
    return payloads.map { payload in
        payload.reduce(into: [String: Any]()) { dictionary, entry in
            dictionary[entry.key] = cdFoundationValue(from: entry.value) ?? NSNull()
        }
    }
}

func cdFoundationObjectDictionary(from json: UnsafePointer<CChar>?) throws -> [String: Any]? {
    guard let json else {
        return nil
    }
    let payload = try cdDecodeJSON(json, as: [String: CDValuePayload].self)
    return payload.reduce(into: [String: Any]()) { dictionary, entry in
        dictionary[entry.key] = cdFoundationValue(from: entry.value) ?? NSNull()
    }
}

@_cdecl("cd_batch_delete_request_new_with_fetch_request")
public func cdBatchDeleteRequestNewWithFetchRequest(
    _ fetchRequestPtr: UnsafeMutableRawPointer?,
    _ outRequest: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let fetchRequestPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing fetch request")
        }
        let fetchRequest: NSFetchRequest<NSFetchRequestResult> = cdBorrow(fetchRequestPtr)
        outRequest?.pointee = cdRetain(NSBatchDeleteRequest(fetchRequest: fetchRequest))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return cdErrorStatus(error)
    }
}

@_cdecl("cd_batch_delete_request_new_with_object_ids")
public func cdBatchDeleteRequestNewWithObjectIDs(
    _ objectIDPtrs: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int32,
    _ outRequest: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let objectIDs: [NSManagedObjectID] = cdObjects(from: objectIDPtrs, count: count)
    outRequest?.pointee = cdRetain(NSBatchDeleteRequest(objectIDs: objectIDs))
    return CDR_OK
}

@_cdecl("cd_batch_delete_request_get_fetch_request")
public func cdBatchDeleteRequestGetFetchRequest(_ requestPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let requestPtr else {
        return nil
    }
    let request: NSBatchDeleteRequest = cdBorrow(requestPtr)
    return cdRetain(request.fetchRequest)
}

@_cdecl("cd_batch_delete_request_get_result_type")
public func cdBatchDeleteRequestGetResultType(_ requestPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let requestPtr else {
        return 0
    }
    let request: NSBatchDeleteRequest = cdBorrow(requestPtr)
    return UInt64(request.resultType.rawValue)
}

@_cdecl("cd_batch_delete_request_set_result_type")
public func cdBatchDeleteRequestSetResultType(
    _ requestPtr: UnsafeMutableRawPointer?,
    _ resultType: UInt64,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let request: NSBatchDeleteRequest = try cdArg(requestPtr, "batch delete request")
        let raw = try cdKnownRawValue(resultType, in: [0, 1, 2], "batch delete result type")
        request.resultType = NSBatchDeleteRequestResultType(rawValue: UInt(raw)) ?? .resultTypeStatusOnly
    }
}

@_cdecl("cd_managed_object_context_execute_batch_delete_request")
public func cdManagedObjectContextExecuteBatchDeleteRequest(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ requestPtr: UnsafeMutableRawPointer?,
    _ outResult: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let request: NSBatchDeleteRequest = try cdArg(requestPtr, "batch delete request")
        let result = try cdExecuteRequest(contextPtr, request, as: NSBatchDeleteResult.self, requireSQLite: "Batch delete")
        outResult?.pointee = result.map(cdRetain)
    }
}

@_cdecl("cd_batch_delete_result_get_result_type")
public func cdBatchDeleteResultGetResultType(_ resultPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let resultPtr else {
        return 0
    }
    let result: NSBatchDeleteResult = cdBorrow(resultPtr)
    return UInt64(result.resultType.rawValue)
}

@_cdecl("cd_batch_delete_result_get_status")
public func cdBatchDeleteResultGetStatus(_ resultPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let resultPtr else {
        return 0
    }
    let result: NSBatchDeleteResult = cdBorrow(resultPtr)
    guard let number = result.result as? NSNumber else {
        return 0
    }
    return number.boolValue ? 1 : 0
}

@_cdecl("cd_batch_delete_result_get_count")
public func cdBatchDeleteResultGetCount(_ resultPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let resultPtr else {
        return 0
    }
    let result: NSBatchDeleteResult = cdBorrow(resultPtr)
    guard let number = result.result as? NSNumber else {
        return 0
    }
    return number.uint64Value
}

@_cdecl("cd_batch_delete_result_get_object_ids")
public func cdBatchDeleteResultGetObjectIDs(_ resultPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let resultPtr else {
        return nil
    }
    let result: NSBatchDeleteResult = cdBorrow(resultPtr)
    guard let objectIDs = result.result as? [NSManagedObjectID] else {
        return nil
    }
    return cdRetain(objectIDs as NSArray)
}

@_cdecl("cd_batch_insert_request_new_with_entity_name")
public func cdBatchInsertRequestNewWithEntityName(
    _ entityName: UnsafePointer<CChar>?,
    _ objectsJSON: UnsafePointer<CChar>?,
    _ outRequest: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let entityName else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing entity name")
        }
        let dictionaries = try cdFoundationObjectDictionaries(from: objectsJSON)
        outRequest?.pointee = cdRetain(NSBatchInsertRequest(entityName: String(cString: entityName), objects: dictionaries))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return cdErrorStatus(error)
    }
}

@_cdecl("cd_batch_insert_request_new_with_entity")
public func cdBatchInsertRequestNewWithEntity(
    _ entityPtr: UnsafeMutableRawPointer?,
    _ objectsJSON: UnsafePointer<CChar>?,
    _ outRequest: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let entityPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing entity")
        }
        let entity: NSEntityDescription = cdBorrow(entityPtr)
        let dictionaries = try cdFoundationObjectDictionaries(from: objectsJSON)
        outRequest?.pointee = cdRetain(NSBatchInsertRequest(entity: entity, objects: dictionaries))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return cdErrorStatus(error)
    }
}

@_cdecl("cd_batch_insert_request_get_entity_name")
public func cdBatchInsertRequestGetEntityName(_ requestPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let requestPtr else {
        return nil
    }
    let request: NSBatchInsertRequest = cdBorrow(requestPtr)
    return cdCString(request.entityName)
}

@_cdecl("cd_batch_insert_request_get_entity")
public func cdBatchInsertRequestGetEntity(_ requestPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let requestPtr else {
        return nil
    }
    let request: NSBatchInsertRequest = cdBorrow(requestPtr)
    return request.entity.map(cdRetain)
}

@_cdecl("cd_batch_insert_request_get_result_type")
public func cdBatchInsertRequestGetResultType(_ requestPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let requestPtr else {
        return 0
    }
    let request: NSBatchInsertRequest = cdBorrow(requestPtr)
    return UInt64(request.resultType.rawValue)
}

@_cdecl("cd_batch_insert_request_set_result_type")
public func cdBatchInsertRequestSetResultType(
    _ requestPtr: UnsafeMutableRawPointer?,
    _ resultType: UInt64,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let request: NSBatchInsertRequest = try cdArg(requestPtr, "batch insert request")
        let raw = try cdKnownRawValue(resultType, in: [0, 1, 2], "batch insert result type")
        request.resultType = NSBatchInsertRequestResultType(rawValue: UInt(raw)) ?? .statusOnly
    }
}

@_cdecl("cd_managed_object_context_execute_batch_insert_request")
public func cdManagedObjectContextExecuteBatchInsertRequest(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ requestPtr: UnsafeMutableRawPointer?,
    _ outResult: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let request: NSBatchInsertRequest = try cdArg(requestPtr, "batch insert request")
        let result = try cdExecuteRequest(contextPtr, request, as: NSBatchInsertResult.self, requireSQLite: "Batch insert")
        outResult?.pointee = result.map(cdRetain)
    }
}

@_cdecl("cd_batch_insert_result_get_result_type")
public func cdBatchInsertResultGetResultType(_ resultPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let resultPtr else {
        return 0
    }
    let result: NSBatchInsertResult = cdBorrow(resultPtr)
    return UInt64(result.resultType.rawValue)
}

@_cdecl("cd_batch_insert_result_get_status")
public func cdBatchInsertResultGetStatus(_ resultPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let resultPtr else {
        return 0
    }
    let result: NSBatchInsertResult = cdBorrow(resultPtr)
    guard let number = result.result as? NSNumber else {
        return 0
    }
    return number.boolValue ? 1 : 0
}

@_cdecl("cd_batch_insert_result_get_count")
public func cdBatchInsertResultGetCount(_ resultPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let resultPtr else {
        return 0
    }
    let result: NSBatchInsertResult = cdBorrow(resultPtr)
    guard let number = result.result as? NSNumber else {
        return 0
    }
    return number.uint64Value
}

@_cdecl("cd_batch_insert_result_get_object_ids")
public func cdBatchInsertResultGetObjectIDs(_ resultPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let resultPtr else {
        return nil
    }
    let result: NSBatchInsertResult = cdBorrow(resultPtr)
    guard let objectIDs = result.result as? [NSManagedObjectID] else {
        return nil
    }
    return cdRetain(objectIDs as NSArray)
}

@_cdecl("cd_batch_update_request_new_with_entity_name")
public func cdBatchUpdateRequestNewWithEntityName(
    _ entityName: UnsafePointer<CChar>?,
    _ outRequest: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let entityName else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing entity name")
        }
        outRequest?.pointee = cdRetain(NSBatchUpdateRequest(entityName: String(cString: entityName)))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return cdErrorStatus(error)
    }
}

@_cdecl("cd_batch_update_request_get_entity_name")
public func cdBatchUpdateRequestGetEntityName(_ requestPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let requestPtr else {
        return nil
    }
    let request: NSBatchUpdateRequest = cdBorrow(requestPtr)
    return cdCString(request.entityName)
}

@_cdecl("cd_batch_update_request_get_entity")
public func cdBatchUpdateRequestGetEntity(_ requestPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let requestPtr else {
        return nil
    }
    let request: NSBatchUpdateRequest = cdBorrow(requestPtr)
    return cdRetain(request.entity)
}

@_cdecl("cd_batch_update_request_get_includes_subentities")
public func cdBatchUpdateRequestGetIncludesSubentities(_ requestPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let requestPtr else {
        return 0
    }
    let request: NSBatchUpdateRequest = cdBorrow(requestPtr)
    return request.includesSubentities ? 1 : 0
}

@_cdecl("cd_batch_update_request_set_includes_subentities")
public func cdBatchUpdateRequestSetIncludesSubentities(_ requestPtr: UnsafeMutableRawPointer?, _ includesSubentities: Int32) {
    guard let requestPtr else {
        return
    }
    let request: NSBatchUpdateRequest = cdBorrow(requestPtr)
    request.includesSubentities = includesSubentities != 0
}

@_cdecl("cd_batch_update_request_get_result_type")
public func cdBatchUpdateRequestGetResultType(_ requestPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let requestPtr else {
        return 0
    }
    let request: NSBatchUpdateRequest = cdBorrow(requestPtr)
    return UInt64(request.resultType.rawValue)
}

@_cdecl("cd_batch_update_request_set_result_type")
public func cdBatchUpdateRequestSetResultType(
    _ requestPtr: UnsafeMutableRawPointer?,
    _ resultType: UInt64,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let request: NSBatchUpdateRequest = try cdArg(requestPtr, "batch update request")
        let raw = try cdKnownRawValue(resultType, in: [0, 1, 2], "batch update result type")
        request.resultType = NSBatchUpdateRequestResultType(rawValue: UInt(raw)) ?? .statusOnlyResultType
    }
}

@_cdecl("cd_batch_update_request_set_predicate")
public func cdBatchUpdateRequestSetPredicate(_ requestPtr: UnsafeMutableRawPointer?, _ predicatePtr: UnsafeMutableRawPointer?) {
    guard let requestPtr else {
        return
    }
    let request: NSBatchUpdateRequest = cdBorrow(requestPtr)
    request.predicate = predicatePtr.map { cdBorrow($0) as NSPredicate }
}

@_cdecl("cd_batch_update_request_set_properties_to_update_json")
public func cdBatchUpdateRequestSetPropertiesToUpdateJSON(
    _ requestPtr: UnsafeMutableRawPointer?,
    _ propertiesJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let requestPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing batch update request")
        }
        let request: NSBatchUpdateRequest = cdBorrow(requestPtr)
        request.propertiesToUpdate = try cdFoundationObjectDictionary(from: propertiesJSON)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return cdErrorStatus(error)
    }
}

@_cdecl("cd_managed_object_context_execute_batch_update_request")
public func cdManagedObjectContextExecuteBatchUpdateRequest(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ requestPtr: UnsafeMutableRawPointer?,
    _ outResult: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let request: NSBatchUpdateRequest = try cdArg(requestPtr, "batch update request")
        let result = try cdExecuteRequest(contextPtr, request, as: NSBatchUpdateResult.self, requireSQLite: "Batch update")
        outResult?.pointee = result.map(cdRetain)
    }
}

@_cdecl("cd_batch_update_result_get_result_type")
public func cdBatchUpdateResultGetResultType(_ resultPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let resultPtr else {
        return 0
    }
    let result: NSBatchUpdateResult = cdBorrow(resultPtr)
    return UInt64(result.resultType.rawValue)
}

@_cdecl("cd_batch_update_result_get_status")
public func cdBatchUpdateResultGetStatus(_ resultPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let resultPtr else {
        return 0
    }
    let result: NSBatchUpdateResult = cdBorrow(resultPtr)
    guard let number = result.result as? NSNumber else {
        return 0
    }
    return number.boolValue ? 1 : 0
}

@_cdecl("cd_batch_update_result_get_count")
public func cdBatchUpdateResultGetCount(_ resultPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let resultPtr else {
        return 0
    }
    let result: NSBatchUpdateResult = cdBorrow(resultPtr)
    guard let number = result.result as? NSNumber else {
        return 0
    }
    return number.uint64Value
}

@_cdecl("cd_batch_update_result_get_object_ids")
public func cdBatchUpdateResultGetObjectIDs(_ resultPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let resultPtr else {
        return nil
    }
    let result: NSBatchUpdateResult = cdBorrow(resultPtr)
    guard let objectIDs = result.result as? [NSManagedObjectID] else {
        return nil
    }
    return cdRetain(objectIDs as NSArray)
}
