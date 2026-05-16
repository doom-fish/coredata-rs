import CoreData
import Foundation

@_cdecl("cd_fetch_request_new")
public func cdFetchRequestNew(
    _ entityName: UnsafePointer<CChar>?,
    _ outRequest: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let entityName else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing fetch request entity name")
        }
        let request = NSFetchRequest<NSManagedObject>(entityName: String(cString: entityName))
        outRequest?.pointee = cdRetain(request)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_fetch_request_get_entity")
public func cdFetchRequestGetEntity(_ requestPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let requestPtr else {
        return nil
    }
    let request: NSFetchRequest<NSFetchRequestResult> = cdBorrow(requestPtr)
    return request.entity.map(cdRetain)
}

@_cdecl("cd_fetch_request_set_entity")
public func cdFetchRequestSetEntity(
    _ requestPtr: UnsafeMutableRawPointer?,
    _ entityPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let requestPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing fetch request")
        }
        let request: NSFetchRequest<NSFetchRequestResult> = cdBorrow(requestPtr)
        request.entity = entityPtr.map { cdBorrow($0) as NSEntityDescription }
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_fetch_request_get_entity_name")
public func cdFetchRequestGetEntityName(_ requestPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let requestPtr else {
        return nil
    }
    let request: NSFetchRequest<NSFetchRequestResult> = cdBorrow(requestPtr)
    return request.entityName.flatMap { cdCString($0) }
}

@_cdecl("cd_fetch_request_set_predicate")
public func cdFetchRequestSetPredicate(
    _ requestPtr: UnsafeMutableRawPointer?,
    _ predicatePtr: UnsafeMutableRawPointer?
) {
    guard let requestPtr else {
        return
    }
    let request: NSFetchRequest<NSManagedObject> = cdBorrow(requestPtr)
    request.predicate = predicatePtr.map { cdBorrow($0) as Foundation.NSPredicate }
}

@_cdecl("cd_fetch_request_set_sort_descriptors_json")
public func cdFetchRequestSetSortDescriptorsJSON(
    _ requestPtr: UnsafeMutableRawPointer?,
    _ descriptorsJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let requestPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing fetch request")
        }
        let request: NSFetchRequest<NSManagedObject> = cdBorrow(requestPtr)
        request.sortDescriptors = try cdSortDescriptors(from: descriptorsJSON)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_fetch_request_set_fetch_limit")
public func cdFetchRequestSetFetchLimit(_ requestPtr: UnsafeMutableRawPointer?, _ fetchLimit: UInt64) {
    guard let requestPtr else {
        return
    }
    let request: NSFetchRequest<NSManagedObject> = cdBorrow(requestPtr)
    request.fetchLimit = Int(fetchLimit)
}

@_cdecl("cd_fetch_request_set_fetch_offset")
public func cdFetchRequestSetFetchOffset(_ requestPtr: UnsafeMutableRawPointer?, _ fetchOffset: UInt64) {
    guard let requestPtr else {
        return
    }
    let request: NSFetchRequest<NSManagedObject> = cdBorrow(requestPtr)
    request.fetchOffset = Int(fetchOffset)
}

@_cdecl("cd_fetch_request_get_result_type")
public func cdFetchRequestGetResultType(_ requestPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let requestPtr else {
        return 0
    }
    let request: NSFetchRequest<NSFetchRequestResult> = cdBorrow(requestPtr)
    return UInt64(request.resultType.rawValue)
}

@_cdecl("cd_fetch_request_set_result_type")
public func cdFetchRequestSetResultType(
    _ requestPtr: UnsafeMutableRawPointer?,
    _ resultType: UInt64,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let requestPtr else {
        cdWriteError(cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing fetch request"), to: outError)
        return CDR_INVALID_ARGUMENT
    }
    let resolvedResultType = NSFetchRequestResultType(rawValue: UInt(resultType))
    let request: NSFetchRequest<NSFetchRequestResult> = cdBorrow(requestPtr)
    request.resultType = resolvedResultType
    return CDR_OK
}

@_cdecl("cd_fetch_request_get_includes_subentities")
public func cdFetchRequestGetIncludesSubentities(_ requestPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let requestPtr else {
        return 0
    }
    let request: NSFetchRequest<NSFetchRequestResult> = cdBorrow(requestPtr)
    return request.includesSubentities ? 1 : 0
}

@_cdecl("cd_fetch_request_set_includes_subentities")
public func cdFetchRequestSetIncludesSubentities(_ requestPtr: UnsafeMutableRawPointer?, _ includesSubentities: Int32) {
    guard let requestPtr else {
        return
    }
    let request: NSFetchRequest<NSFetchRequestResult> = cdBorrow(requestPtr)
    request.includesSubentities = includesSubentities != 0
}

@_cdecl("cd_fetch_request_get_includes_property_values")
public func cdFetchRequestGetIncludesPropertyValues(_ requestPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let requestPtr else {
        return 0
    }
    let request: NSFetchRequest<NSFetchRequestResult> = cdBorrow(requestPtr)
    return request.includesPropertyValues ? 1 : 0
}

@_cdecl("cd_fetch_request_set_includes_property_values")
public func cdFetchRequestSetIncludesPropertyValues(_ requestPtr: UnsafeMutableRawPointer?, _ includesPropertyValues: Int32) {
    guard let requestPtr else {
        return
    }
    let request: NSFetchRequest<NSFetchRequestResult> = cdBorrow(requestPtr)
    request.includesPropertyValues = includesPropertyValues != 0
}

@_cdecl("cd_fetch_request_get_returns_objects_as_faults")
public func cdFetchRequestGetReturnsObjectsAsFaults(_ requestPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let requestPtr else {
        return 0
    }
    let request: NSFetchRequest<NSFetchRequestResult> = cdBorrow(requestPtr)
    return request.returnsObjectsAsFaults ? 1 : 0
}

@_cdecl("cd_fetch_request_set_returns_objects_as_faults")
public func cdFetchRequestSetReturnsObjectsAsFaults(_ requestPtr: UnsafeMutableRawPointer?, _ returnsObjectsAsFaults: Int32) {
    guard let requestPtr else {
        return
    }
    let request: NSFetchRequest<NSFetchRequestResult> = cdBorrow(requestPtr)
    request.returnsObjectsAsFaults = returnsObjectsAsFaults != 0
}

@_cdecl("cd_fetch_request_get_relationship_key_paths_for_prefetching_json")
public func cdFetchRequestGetRelationshipKeyPathsForPrefetchingJSON(
    _ requestPtr: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let requestPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing fetch request")
        }
        let request: NSFetchRequest<NSFetchRequestResult> = cdBorrow(requestPtr)
        outJSON?.pointee = cdCString(try cdEncodeJSON(request.relationshipKeyPathsForPrefetching))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_fetch_request_set_relationship_key_paths_for_prefetching_json")
public func cdFetchRequestSetRelationshipKeyPathsForPrefetchingJSON(
    _ requestPtr: UnsafeMutableRawPointer?,
    _ keyPathsJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let requestPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing fetch request")
        }
        let request: NSFetchRequest<NSFetchRequestResult> = cdBorrow(requestPtr)
        request.relationshipKeyPathsForPrefetching = try cdStringArray(from: keyPathsJSON)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_fetch_request_get_includes_pending_changes")
public func cdFetchRequestGetIncludesPendingChanges(_ requestPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let requestPtr else {
        return 0
    }
    let request: NSFetchRequest<NSFetchRequestResult> = cdBorrow(requestPtr)
    return request.includesPendingChanges ? 1 : 0
}

@_cdecl("cd_fetch_request_set_includes_pending_changes")
public func cdFetchRequestSetIncludesPendingChanges(_ requestPtr: UnsafeMutableRawPointer?, _ includesPendingChanges: Int32) {
    guard let requestPtr else {
        return
    }
    let request: NSFetchRequest<NSFetchRequestResult> = cdBorrow(requestPtr)
    request.includesPendingChanges = includesPendingChanges != 0
}

@_cdecl("cd_fetch_request_get_returns_distinct_results")
public func cdFetchRequestGetReturnsDistinctResults(_ requestPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let requestPtr else {
        return 0
    }
    let request: NSFetchRequest<NSFetchRequestResult> = cdBorrow(requestPtr)
    return request.returnsDistinctResults ? 1 : 0
}

@_cdecl("cd_fetch_request_set_returns_distinct_results")
public func cdFetchRequestSetReturnsDistinctResults(_ requestPtr: UnsafeMutableRawPointer?, _ returnsDistinctResults: Int32) {
    guard let requestPtr else {
        return
    }
    let request: NSFetchRequest<NSFetchRequestResult> = cdBorrow(requestPtr)
    request.returnsDistinctResults = returnsDistinctResults != 0
}

@_cdecl("cd_fetch_request_get_fetch_batch_size")
public func cdFetchRequestGetFetchBatchSize(_ requestPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let requestPtr else {
        return 0
    }
    let request: NSFetchRequest<NSFetchRequestResult> = cdBorrow(requestPtr)
    return UInt64(request.fetchBatchSize)
}

@_cdecl("cd_fetch_request_set_fetch_batch_size")
public func cdFetchRequestSetFetchBatchSize(_ requestPtr: UnsafeMutableRawPointer?, _ fetchBatchSize: UInt64) {
    guard let requestPtr else {
        return
    }
    let request: NSFetchRequest<NSFetchRequestResult> = cdBorrow(requestPtr)
    request.fetchBatchSize = Int(fetchBatchSize)
}

@_cdecl("cd_fetch_request_get_should_refresh_refetched_objects")
public func cdFetchRequestGetShouldRefreshRefetchedObjects(_ requestPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let requestPtr else {
        return 0
    }
    let request: NSFetchRequest<NSFetchRequestResult> = cdBorrow(requestPtr)
    return request.shouldRefreshRefetchedObjects ? 1 : 0
}

@_cdecl("cd_fetch_request_set_should_refresh_refetched_objects")
public func cdFetchRequestSetShouldRefreshRefetchedObjects(_ requestPtr: UnsafeMutableRawPointer?, _ shouldRefresh: Int32) {
    guard let requestPtr else {
        return
    }
    let request: NSFetchRequest<NSFetchRequestResult> = cdBorrow(requestPtr)
    request.shouldRefreshRefetchedObjects = shouldRefresh != 0
}

@_cdecl("cd_fetch_request_execute_object_ids")
public func cdFetchRequestExecuteObjectIDs(
    _ requestPtr: UnsafeMutableRawPointer?,
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let requestPtr, let contextPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing fetch request or managed object context")
        }
        let request: NSFetchRequest<NSManagedObjectID> = cdBorrow(requestPtr)
        let context: NSManagedObjectContext = cdBorrow(contextPtr)
        let originalResultType = request.resultType
        request.resultType = .managedObjectIDResultType
        defer {
            request.resultType = originalResultType
        }
        let results = try context.fetch(request)
        outArray?.pointee = cdRetain(results as NSArray)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}
