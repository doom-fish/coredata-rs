import CoreData
import Foundation

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

@_cdecl("cd_managed_object_get_managed_object_context")
public func cdManagedObjectGetManagedObjectContext(_ objectPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let objectPtr else {
        return nil
    }
    let object: NSManagedObject = cdBorrow(objectPtr)
    return object.managedObjectContext.map(cdRetain)
}

@_cdecl("cd_managed_object_get_object_id")
public func cdManagedObjectGetObjectID(_ objectPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let objectPtr else {
        return nil
    }
    let object: NSManagedObject = cdBorrow(objectPtr)
    return cdRetain(object.objectID)
}

@_cdecl("cd_managed_object_get_inserted")
public func cdManagedObjectGetInserted(_ objectPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let objectPtr else {
        return 0
    }
    let object: NSManagedObject = cdBorrow(objectPtr)
    return object.isInserted ? 1 : 0
}

@_cdecl("cd_managed_object_get_updated")
public func cdManagedObjectGetUpdated(_ objectPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let objectPtr else {
        return 0
    }
    let object: NSManagedObject = cdBorrow(objectPtr)
    return object.isUpdated ? 1 : 0
}

@_cdecl("cd_managed_object_get_deleted")
public func cdManagedObjectGetDeleted(_ objectPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let objectPtr else {
        return 0
    }
    let object: NSManagedObject = cdBorrow(objectPtr)
    return object.isDeleted ? 1 : 0
}

@_cdecl("cd_managed_object_get_has_changes")
public func cdManagedObjectGetHasChanges(_ objectPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let objectPtr else {
        return 0
    }
    let object: NSManagedObject = cdBorrow(objectPtr)
    return object.hasChanges ? 1 : 0
}

@_cdecl("cd_managed_object_get_has_persistent_changed_values")
public func cdManagedObjectGetHasPersistentChangedValues(_ objectPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let objectPtr else {
        return 0
    }
    let object: NSManagedObject = cdBorrow(objectPtr)
    return object.hasPersistentChangedValues ? 1 : 0
}

@_cdecl("cd_managed_object_get_fault")
public func cdManagedObjectGetFault(_ objectPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let objectPtr else {
        return 0
    }
    let object: NSManagedObject = cdBorrow(objectPtr)
    return object.isFault ? 1 : 0
}

@_cdecl("cd_managed_object_has_fault_for_relationship_named")
public func cdManagedObjectHasFaultForRelationshipNamed(
    _ objectPtr: UnsafeMutableRawPointer?,
    _ relationshipName: UnsafePointer<CChar>?
) -> Int32 {
    guard let objectPtr, let relationshipName else {
        return 0
    }
    let object: NSManagedObject = cdBorrow(objectPtr)
    return object.hasFault(forRelationshipNamed: String(cString: relationshipName)) ? 1 : 0
}

@_cdecl("cd_managed_object_object_ids_for_relationship_named")
public func cdManagedObjectObjectIDsForRelationshipNamed(
    _ objectPtr: UnsafeMutableRawPointer?,
    _ relationshipName: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let objectPtr, let relationshipName else {
        return nil
    }
    let object: NSManagedObject = cdBorrow(objectPtr)
    return cdRetain(object.objectIDs(forRelationshipNamed: String(cString: relationshipName)) as NSArray)
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

@_cdecl("cd_managed_object_committed_values_json")
public func cdManagedObjectCommittedValuesJSON(
    _ objectPtr: UnsafeMutableRawPointer?,
    _ keysJSON: UnsafePointer<CChar>?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let objectPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object")
        }
        let object: NSManagedObject = cdBorrow(objectPtr)
        let keys = try cdStringArray(from: keysJSON)
        let committedValues = object.committedValues(forKeys: keys.isEmpty ? nil : keys)
        let payload = try cdValuePayloadMap(from: committedValues)
        outJSON?.pointee = cdCString(try cdEncodeJSON(payload))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_managed_object_changed_values_json")
public func cdManagedObjectChangedValuesJSON(
    _ objectPtr: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let objectPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object")
        }
        let object: NSManagedObject = cdBorrow(objectPtr)
        let payload = try cdValuePayloadMap(from: object.changedValues())
        outJSON?.pointee = cdCString(try cdEncodeJSON(payload))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_managed_object_changed_values_for_current_event_json")
public func cdManagedObjectChangedValuesForCurrentEventJSON(
    _ objectPtr: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let objectPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object")
        }
        let object: NSManagedObject = cdBorrow(objectPtr)
        let payload = try cdValuePayloadMap(from: object.changedValuesForCurrentEvent())
        outJSON?.pointee = cdCString(try cdEncodeJSON(payload))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_managed_object_id_get_entity")
public func cdManagedObjectIDGetEntity(_ objectIDPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let objectIDPtr else {
        return nil
    }
    let objectID: NSManagedObjectID = cdBorrow(objectIDPtr)
    return cdRetain(objectID.entity)
}

@_cdecl("cd_managed_object_id_get_temporary")
public func cdManagedObjectIDGetTemporary(_ objectIDPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let objectIDPtr else {
        return 0
    }
    let objectID: NSManagedObjectID = cdBorrow(objectIDPtr)
    return objectID.isTemporaryID ? 1 : 0
}

@_cdecl("cd_managed_object_id_get_uri")
public func cdManagedObjectIDGetURI(_ objectIDPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let objectIDPtr else {
        return nil
    }
    let objectID: NSManagedObjectID = cdBorrow(objectIDPtr)
    return cdCString(objectID.uriRepresentation().absoluteString)
}
