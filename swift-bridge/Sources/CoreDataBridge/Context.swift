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

@_cdecl("cd_managed_object_new")
public func cdManagedObjectNew(
    _ entityPtr: UnsafeMutableRawPointer?,
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outObject: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let entityPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing entity description")
        }
        let entity: NSEntityDescription = cdBorrow(entityPtr)
        let context = contextPtr.map { cdBorrow($0) as NSManagedObjectContext }
        outObject?.pointee = cdRetain(NSManagedObject(entity: entity, insertInto: context))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_managed_object_entity")
public func cdManagedObjectEntity(_ objectPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let objectPtr else {
        return nil
    }
    let object: NSManagedObject = cdBorrow(objectPtr)
    return cdRetain(object.entity)
}

@_cdecl("cd_managed_object_set_value_json")
public func cdManagedObjectSetValueJSON(
    _ objectPtr: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ valueJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let objectPtr, let key else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object or key")
        }
        let object: NSManagedObject = cdBorrow(objectPtr)
        let payload = try cdDecodeJSON(valueJSON, as: CDValuePayload.self)
        object.setValue(cdFoundationValue(from: payload), forKey: String(cString: key))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_managed_object_get_value_json")
public func cdManagedObjectGetValueJSON(
    _ objectPtr: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let objectPtr, let key else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object or key")
        }
        let object: NSManagedObject = cdBorrow(objectPtr)
        let payload = try cdValuePayload(from: object.value(forKey: String(cString: key)))
        outJSON?.pointee = cdCString(try cdEncodeJSON(payload))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

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

@_cdecl("cd_predicate_new_with_format")
public func cdPredicateNewWithFormat(
    _ format: UnsafePointer<CChar>?,
    _ argumentsJSON: UnsafePointer<CChar>?,
    _ outPredicate: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let format else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing predicate format")
        }
        let formatString = String(cString: format)
        let predicate: Foundation.NSPredicate
        if let argumentsJSON {
            let arguments = try cdDecodeJSON(argumentsJSON, as: [CDValuePayload].self)
                .map(cdPredicateArgument)
            predicate = Foundation.NSPredicate(format: formatString, argumentArray: arguments)
        } else {
            predicate = Foundation.NSPredicate(format: formatString)
        }
        outPredicate?.pointee = cdRetain(predicate)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}
