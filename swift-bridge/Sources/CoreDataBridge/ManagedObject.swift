import CoreData
import CoreDataObjCBridge
import Foundation

func cdAttributeKey(_ key: UnsafePointer<CChar>?, of object: NSManagedObject) throws -> String {
    guard let key else {
        throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing attribute name")
    }
    let name = String(cString: key)
    guard object.entity.attributesByName[name] != nil else {
        throw cdBridgeNSError(
            code: CDR_INVALID_ARGUMENT,
            message: "Entity '\(object.entity.name ?? "")' has no attribute named '\(name)'"
        )
    }
    return name
}

func cdRelationshipKey(_ key: UnsafePointer<CChar>?, of object: NSManagedObject) throws -> String {
    guard let key else {
        throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing relationship name")
    }
    let name = String(cString: key)
    guard object.entity.relationshipsByName[name] != nil else {
        throw cdBridgeNSError(
            code: CDR_INVALID_ARGUMENT,
            message: "Entity '\(object.entity.name ?? "")' has no relationship named '\(name)'"
        )
    }
    return name
}

func cdManagedObjectFlag(
    _ objectPtr: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<Int32>?,
    _ outError: CDOutError,
    _ read: (NSManagedObject) -> Bool
) -> Int32 {
    cdStatus(outError) {
        let value = try cdWithManagedObject(objectPtr) { object, _ in read(object) }
        outValue?.pointee = value ? 1 : 0
    }
}

func cdEnsureInstantiable(_ entity: NSEntityDescription, in context: NSManagedObjectContext?) throws {
    guard !entity.isAbstract else {
        throw cdBridgeNSError(
            code: CDR_INVALID_ARGUMENT,
            message: "Entity '\(entity.name ?? "")' is abstract and cannot be instantiated"
        )
    }
    guard let context else {
        return
    }
    guard let coordinator = context.persistentStoreCoordinator else {
        throw cdBridgeNSError(
            code: CDR_INVALID_ARGUMENT,
            message: "The managed object context has no persistent store coordinator or parent context"
        )
    }
    guard let name = entity.name, coordinator.managedObjectModel.entitiesByName[name] != nil else {
        throw cdBridgeNSError(
            code: CDR_INVALID_ARGUMENT,
            message: "The context's managed object model has no entity named '\(entity.name ?? "")'"
        )
    }
}

@_cdecl("cd_managed_object_new")
public func cdManagedObjectNew(
    _ entityPtr: UnsafeMutableRawPointer?,
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outObject: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let entity: NSEntityDescription = try cdArg(entityPtr, "entity description")
        guard let name = entity.name else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "The entity has no name")
        }
        outObject?.pointee = try cdWithContext(contextPtr) { context in
            guard let modelEntity = context.persistentStoreCoordinator?.managedObjectModel.entitiesByName[name] else {
                throw cdBridgeNSError(
                    code: CDR_INVALID_ARGUMENT,
                    message: "The context's managed object model has no entity named '\(name)'"
                )
            }
            try cdEnsureInstantiable(modelEntity, in: context)
            let object = try cdObjCValue { CDXNewManagedObject(modelEntity, context, $0) }
            return cdRetainManagedObject(object, in: context)
        }
    }
}

@_cdecl("cd_managed_object_entity")
public func cdManagedObjectEntity(
    _ objectPtr: UnsafeMutableRawPointer?,
    _ outEntity: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let entity = try cdWithManagedObject(objectPtr, requireRegistered: false) { object, _ in object.entity }
        outEntity?.pointee = cdRetain(entity)
    }
}

@_cdecl("cd_managed_object_get_managed_object_context")
public func cdManagedObjectGetManagedObjectContext(
    _ objectPtr: UnsafeMutableRawPointer?,
    _ outContext: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let context = try cdWithManagedObject(objectPtr, requireRegistered: false) { object, bound in
            bound.flatMap { object.managedObjectContext === $0 ? $0 : nil }
        }
        outContext?.pointee = context.map(cdRetain)
    }
}

@_cdecl("cd_managed_object_get_object_id")
public func cdManagedObjectGetObjectID(
    _ objectPtr: UnsafeMutableRawPointer?,
    _ outObjectID: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let objectID = try cdWithManagedObject(objectPtr, requireRegistered: false) { object, _ in object.objectID }
        outObjectID?.pointee = cdRetain(objectID)
    }
}

@_cdecl("cd_managed_object_get_inserted")
public func cdManagedObjectGetInserted(
    _ objectPtr: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<Int32>?,
    _ outError: CDOutError
) -> Int32 {
    cdManagedObjectFlag(objectPtr, outValue, outError) { $0.isInserted }
}

@_cdecl("cd_managed_object_get_updated")
public func cdManagedObjectGetUpdated(
    _ objectPtr: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<Int32>?,
    _ outError: CDOutError
) -> Int32 {
    cdManagedObjectFlag(objectPtr, outValue, outError) { $0.isUpdated }
}

@_cdecl("cd_managed_object_get_deleted")
public func cdManagedObjectGetDeleted(
    _ objectPtr: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<Int32>?,
    _ outError: CDOutError
) -> Int32 {
    cdManagedObjectFlag(objectPtr, outValue, outError) { $0.isDeleted }
}

@_cdecl("cd_managed_object_get_has_changes")
public func cdManagedObjectGetHasChanges(
    _ objectPtr: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<Int32>?,
    _ outError: CDOutError
) -> Int32 {
    cdManagedObjectFlag(objectPtr, outValue, outError) { $0.hasChanges }
}

@_cdecl("cd_managed_object_get_has_persistent_changed_values")
public func cdManagedObjectGetHasPersistentChangedValues(
    _ objectPtr: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<Int32>?,
    _ outError: CDOutError
) -> Int32 {
    cdManagedObjectFlag(objectPtr, outValue, outError) { $0.hasPersistentChangedValues }
}

@_cdecl("cd_managed_object_get_fault")
public func cdManagedObjectGetFault(
    _ objectPtr: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<Int32>?,
    _ outError: CDOutError
) -> Int32 {
    cdManagedObjectFlag(objectPtr, outValue, outError) { $0.isFault }
}

@_cdecl("cd_managed_object_has_fault_for_relationship_named")
public func cdManagedObjectHasFaultForRelationshipNamed(
    _ objectPtr: UnsafeMutableRawPointer?,
    _ relationshipName: UnsafePointer<CChar>?,
    _ outValue: UnsafeMutablePointer<Int32>?,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let value = try cdWithManagedObject(objectPtr) { object, _ -> Bool in
            let name = try cdRelationshipKey(relationshipName, of: object)
            var result: ObjCBool = false
            try cdObjCCall { CDXHasFaultForRelationshipNamed(object, name, &result, $0) }
            return result.boolValue
        }
        outValue?.pointee = value ? 1 : 0
    }
}

@_cdecl("cd_managed_object_object_ids_for_relationship_named")
public func cdManagedObjectObjectIDsForRelationshipNamed(
    _ objectPtr: UnsafeMutableRawPointer?,
    _ relationshipName: UnsafePointer<CChar>?,
    _ outArray: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let objectIDs = try cdWithManagedObject(objectPtr) { object, _ -> [NSManagedObjectID] in
            let name = try cdRelationshipKey(relationshipName, of: object)
            return try cdObjCValue { CDXObjectIDsForRelationshipNamed(object, name, $0) }
        }
        outArray?.pointee = cdRetain(objectIDs as NSArray)
    }
}

@_cdecl("cd_managed_object_set_value_json")
public func cdManagedObjectSetValueJSON(
    _ objectPtr: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ valueJSON: UnsafePointer<CChar>?,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let payload = try cdDecodeJSON(valueJSON, as: CDValuePayload.self)
        try cdWithManagedObject(objectPtr) { object, _ in
            let name = try cdAttributeKey(key, of: object)
            try cdTrySetValue(cdFoundationValue(from: payload), forKey: name, on: object)
        }
    }
}

@_cdecl("cd_managed_object_get_value_json")
public func cdManagedObjectGetValueJSON(
    _ objectPtr: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ outJSON: CDOutString,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let payload = try cdWithManagedObject(objectPtr) { object, _ -> CDValuePayload in
            let name = try cdAttributeKey(key, of: object)
            var value: AnyObject?
            try cdObjCCall { CDXValueForKey(object, name, &value, $0) }
            return try cdValuePayload(from: value)
        }
        outJSON?.pointee = cdCString(try cdEncodeJSON(payload))
    }
}

@_cdecl("cd_managed_object_committed_values_json")
public func cdManagedObjectCommittedValuesJSON(
    _ objectPtr: UnsafeMutableRawPointer?,
    _ keysJSON: UnsafePointer<CChar>?,
    _ outJSON: CDOutString,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let keys = try cdStringArray(from: keysJSON)
        let payload = try cdWithManagedObject(objectPtr) { object, _ in
            try cdValuePayloadMap(from: object.committedValues(forKeys: keys.isEmpty ? nil : keys))
        }
        outJSON?.pointee = cdCString(try cdEncodeJSON(payload))
    }
}

@_cdecl("cd_managed_object_changed_values_json")
public func cdManagedObjectChangedValuesJSON(
    _ objectPtr: UnsafeMutableRawPointer?,
    _ outJSON: CDOutString,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let payload = try cdWithManagedObject(objectPtr) { object, _ in
            try cdValuePayloadMap(from: object.changedValues())
        }
        outJSON?.pointee = cdCString(try cdEncodeJSON(payload))
    }
}

@_cdecl("cd_managed_object_changed_values_for_current_event_json")
public func cdManagedObjectChangedValuesForCurrentEventJSON(
    _ objectPtr: UnsafeMutableRawPointer?,
    _ outJSON: CDOutString,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let payload = try cdWithManagedObject(objectPtr) { object, _ in
            try cdValuePayloadMap(from: object.changedValuesForCurrentEvent())
        }
        outJSON?.pointee = cdCString(try cdEncodeJSON(payload))
    }
}

@_cdecl("cd_managed_object_id_get_entity_name")
public func cdManagedObjectIDGetEntityName(_ objectIDPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let objectIDPtr else {
        return nil
    }
    let objectID: NSManagedObjectID = cdBorrow(objectIDPtr)
    return objectID.entity.name.flatMap { cdCString($0) }
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
