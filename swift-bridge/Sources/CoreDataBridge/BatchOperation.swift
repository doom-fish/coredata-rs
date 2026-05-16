import CoreData
import Foundation

func cdFoundationObjectDictionaries(from json: UnsafePointer<CChar>?) throws -> [[String: Any]] {
    let payloads = try cdDecodeJSON(json, as: [[String: CDValuePayload]].self)
    return payloads.map { payload in
        payload.reduce(into: [String: Any]()) { dictionary, entry in
            dictionary[entry.key] = cdFoundationValue(from: entry.value) ?? NSNull()
        }
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
        return Int32(error.code)
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

@_cdecl("cd_batch_delete_request_get_result_type")
public func cdBatchDeleteRequestGetResultType(_ requestPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let requestPtr else {
        return 0
    }
    let request: NSBatchDeleteRequest = cdBorrow(requestPtr)
    return UInt64(request.resultType.rawValue)
}

@_cdecl("cd_batch_delete_request_set_result_type")
public func cdBatchDeleteRequestSetResultType(_ requestPtr: UnsafeMutableRawPointer?, _ resultType: UInt64) {
    guard let requestPtr else {
        return
    }
    let request: NSBatchDeleteRequest = cdBorrow(requestPtr)
    request.resultType = NSBatchDeleteRequestResultType(rawValue: UInt(resultType)) ?? request.resultType
}

@_cdecl("cd_managed_object_context_execute_batch_delete_request")
public func cdManagedObjectContextExecuteBatchDeleteRequest(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ requestPtr: UnsafeMutableRawPointer?,
    _ outResult: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let contextPtr, let requestPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object context or batch delete request")
        }
        let context: NSManagedObjectContext = cdBorrow(contextPtr)
        let request: NSBatchDeleteRequest = cdBorrow(requestPtr)
        let result = try context.execute(request) as? NSBatchDeleteResult
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
        return Int32(error.code)
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

@_cdecl("cd_batch_insert_request_get_result_type")
public func cdBatchInsertRequestGetResultType(_ requestPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let requestPtr else {
        return 0
    }
    let request: NSBatchInsertRequest = cdBorrow(requestPtr)
    return UInt64(request.resultType.rawValue)
}

@_cdecl("cd_batch_insert_request_set_result_type")
public func cdBatchInsertRequestSetResultType(_ requestPtr: UnsafeMutableRawPointer?, _ resultType: UInt64) {
    guard let requestPtr else {
        return
    }
    let request: NSBatchInsertRequest = cdBorrow(requestPtr)
    request.resultType = NSBatchInsertRequestResultType(rawValue: UInt(resultType)) ?? request.resultType
}

@_cdecl("cd_managed_object_context_execute_batch_insert_request")
public func cdManagedObjectContextExecuteBatchInsertRequest(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ requestPtr: UnsafeMutableRawPointer?,
    _ outResult: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let contextPtr, let requestPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object context or batch insert request")
        }
        let context: NSManagedObjectContext = cdBorrow(contextPtr)
        let request: NSBatchInsertRequest = cdBorrow(requestPtr)
        let result = try context.execute(request) as? NSBatchInsertResult
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
