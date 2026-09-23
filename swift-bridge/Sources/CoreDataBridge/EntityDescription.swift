import CoreData
import CoreDataObjCBridge
import Foundation

@_cdecl("cd_entity_description_new")
public func cdEntityDescriptionNew(
    _ outEntity: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outEntity?.pointee = cdRetain(NSEntityDescription())
    return CDR_OK
}

@_cdecl("cd_entity_description_entity_for_name")
public func cdEntityDescriptionEntityForName(
    _ name: UnsafePointer<CChar>?,
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outEntity: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        guard let name else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing entity name")
        }
        let entityName = String(cString: name)
        let entity = try cdWithContext(contextPtr) { context -> NSEntityDescription? in
            guard let coordinator = context.persistentStoreCoordinator else {
                throw cdBridgeNSError(
                    code: CDR_INVALID_ARGUMENT,
                    message: "The managed object context has no persistent store coordinator or parent context"
                )
            }
            return coordinator.managedObjectModel.entitiesByName[entityName]
        }
        outEntity?.pointee = entity.map(cdRetain)
    }
}

@_cdecl("cd_entity_description_insert_new_object_for_name")
public func cdEntityDescriptionInsertNewObjectForName(
    _ name: UnsafePointer<CChar>?,
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outObject: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        guard let name else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing entity name")
        }
        let entityName = String(cString: name)
        outObject?.pointee = try cdWithContext(contextPtr) { context in
            guard let coordinator = context.persistentStoreCoordinator else {
                throw cdBridgeNSError(
                    code: CDR_INVALID_ARGUMENT,
                    message: "The managed object context has no persistent store coordinator or parent context"
                )
            }
            guard let entity = coordinator.managedObjectModel.entitiesByName[entityName] else {
                throw cdBridgeNSError(
                    code: CDR_INVALID_ARGUMENT,
                    message: "The managed object model has no entity named '\(entityName)'"
                )
            }
            try cdEnsureInstantiable(entity, in: context)
            let object = try cdObjCValue { CDXInsertNewObjectForEntityName(entityName, context, $0) }
            return cdRetainManagedObject(object, in: context)
        }
    }
}

@_cdecl("cd_entity_description_get_managed_object_model")
public func cdEntityDescriptionGetManagedObjectModel(_ entityPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let entityPtr else {
        return nil
    }
    let entity: NSEntityDescription = cdBorrow(entityPtr)
    return cdRetain(entity.managedObjectModel)
}

@_cdecl("cd_entity_description_get_name")
public func cdEntityDescriptionGetName(_ entityPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let entityPtr else {
        return nil
    }
    let entity: NSEntityDescription = cdBorrow(entityPtr)
    guard let name = entity.name else {
        return nil
    }
    return cdCString(name)
}

@_cdecl("cd_entity_description_set_name")
public func cdEntityDescriptionSetName(
    _ entityPtr: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let entityPtr, let name else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing entity or entity name")
        }
        let entity: NSEntityDescription = cdBorrow(entityPtr)
        try cdTrySetValue(String(cString: name), forKey: "name", on: entity)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return cdErrorStatus(error)
    }
}

@_cdecl("cd_entity_description_get_managed_object_class_name")
public func cdEntityDescriptionGetManagedObjectClassName(_ entityPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let entityPtr else {
        return nil
    }
    let entity: NSEntityDescription = cdBorrow(entityPtr)
    return cdCString(entity.managedObjectClassName)
}

@_cdecl("cd_entity_description_set_managed_object_class_name")
public func cdEntityDescriptionSetManagedObjectClassName(
    _ entityPtr: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let entityPtr, let name else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing entity or class name")
        }
        let entity: NSEntityDescription = cdBorrow(entityPtr)
        try cdTrySetValue(String(cString: name), forKey: "managedObjectClassName", on: entity)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return cdErrorStatus(error)
    }
}

@_cdecl("cd_entity_description_get_abstract")
public func cdEntityDescriptionGetAbstract(_ entityPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let entityPtr else {
        return 0
    }
    let entity: NSEntityDescription = cdBorrow(entityPtr)
    return entity.isAbstract ? 1 : 0
}

@_cdecl("cd_entity_description_set_abstract")
public func cdEntityDescriptionSetAbstract(
    _ entityPtr: UnsafeMutableRawPointer?,
    _ abstractFlag: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let entityPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing entity")
        }
        let entity: NSEntityDescription = cdBorrow(entityPtr)
        try cdTrySetValue(abstractFlag != 0, forKey: "abstract", on: entity)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return cdErrorStatus(error)
    }
}

@_cdecl("cd_entity_description_get_user_info_json")
public func cdEntityDescriptionGetUserInfoJSON(
    _ entityPtr: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let entityPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing entity")
        }
        let entity: NSEntityDescription = cdBorrow(entityPtr)
        let payload = cdStringMapPayload(from: entity.userInfo as? [String: Any])
        outJSON?.pointee = cdCString(try cdEncodeJSON(payload))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return cdErrorStatus(error)
    }
}

@_cdecl("cd_entity_description_set_user_info_json")
public func cdEntityDescriptionSetUserInfoJSON(
    _ entityPtr: UnsafeMutableRawPointer?,
    _ userInfoJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let entityPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing entity")
        }
        let entity: NSEntityDescription = cdBorrow(entityPtr)
        try cdTrySetValue(try cdStringDictionary(from: userInfoJSON), forKey: "userInfo", on: entity)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return cdErrorStatus(error)
    }
}

@_cdecl("cd_entity_description_get_version_hash")
public func cdEntityDescriptionGetVersionHash(_ entityPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let entityPtr else {
        return nil
    }
    let entity: NSEntityDescription = cdBorrow(entityPtr)
    return cdCString(cdHexString(entity.versionHash))
}

@_cdecl("cd_entity_description_get_version_hash_modifier")
public func cdEntityDescriptionGetVersionHashModifier(_ entityPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let entityPtr else {
        return nil
    }
    let entity: NSEntityDescription = cdBorrow(entityPtr)
    return entity.versionHashModifier.flatMap { cdCString($0) }
}

@_cdecl("cd_entity_description_set_version_hash_modifier")
public func cdEntityDescriptionSetVersionHashModifier(
    _ entityPtr: UnsafeMutableRawPointer?,
    _ modifier: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let entityPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing entity")
        }
        let entity: NSEntityDescription = cdBorrow(entityPtr)
        try cdTrySetValue(cdOptionalString(modifier), forKey: "versionHashModifier", on: entity)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return cdErrorStatus(error)
    }
}

@_cdecl("cd_entity_description_get_renaming_identifier")
public func cdEntityDescriptionGetRenamingIdentifier(_ entityPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let entityPtr else {
        return nil
    }
    let entity: NSEntityDescription = cdBorrow(entityPtr)
    return entity.renamingIdentifier.flatMap { cdCString($0) }
}

@_cdecl("cd_entity_description_set_renaming_identifier")
public func cdEntityDescriptionSetRenamingIdentifier(
    _ entityPtr: UnsafeMutableRawPointer?,
    _ identifier: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let entityPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing entity")
        }
        let entity: NSEntityDescription = cdBorrow(entityPtr)
        try cdTrySetValue(cdOptionalString(identifier), forKey: "renamingIdentifier", on: entity)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return cdErrorStatus(error)
    }
}

@_cdecl("cd_entity_description_get_uniqueness_constraints_json")
public func cdEntityDescriptionGetUniquenessConstraintsJSON(
    _ entityPtr: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let entityPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing entity")
        }
        let entity: NSEntityDescription = cdBorrow(entityPtr)
        let payload = entity.uniquenessConstraints.map { constraint in
            constraint.map { component -> String in
                if let attribute = component as? NSAttributeDescription {
                    return attribute.name
                }
                if let string = component as? String {
                    return string
                }
                return String(describing: component)
            }
        }
        outJSON?.pointee = cdCString(try cdEncodeJSON(payload))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return cdErrorStatus(error)
    }
}

@_cdecl("cd_entity_description_set_uniqueness_constraints_json")
public func cdEntityDescriptionSetUniquenessConstraintsJSON(
    _ entityPtr: UnsafeMutableRawPointer?,
    _ constraintsJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let entityPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing entity")
        }
        let entity: NSEntityDescription = cdBorrow(entityPtr)
        let constraints = try cdStringMatrix(from: constraintsJSON)
        try cdTrySetValue(constraints.map { $0.map { $0 as NSString } }, forKey: "uniquenessConstraints", on: entity)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return cdErrorStatus(error)
    }
}

@_cdecl("cd_entity_description_add_attribute")
public func cdEntityDescriptionAddAttribute(
    _ entityPtr: UnsafeMutableRawPointer?,
    _ attributePtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let entityPtr, let attributePtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing entity or attribute")
        }
        let entity: NSEntityDescription = cdBorrow(entityPtr)
        let attribute: NSAttributeDescription = cdBorrow(attributePtr)
        var properties = entity.properties
        properties.removeAll { existing in existing.name == attribute.name }
        properties.append(attribute)
        try cdTrySetValue(properties, forKey: "properties", on: entity)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return cdErrorStatus(error)
    }
}

@_cdecl("cd_entity_description_add_relationship")
public func cdEntityDescriptionAddRelationship(
    _ entityPtr: UnsafeMutableRawPointer?,
    _ relationshipPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let entityPtr, let relationshipPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing entity or relationship")
        }
        let entity: NSEntityDescription = cdBorrow(entityPtr)
        let relationship: NSRelationshipDescription = cdBorrow(relationshipPtr)
        var properties = entity.properties
        properties.removeAll { existing in existing.name == relationship.name }
        properties.append(relationship)
        try cdTrySetValue(properties, forKey: "properties", on: entity)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return cdErrorStatus(error)
    }
}

@_cdecl("cd_entity_description_attributes")
public func cdEntityDescriptionAttributes(_ entityPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let entityPtr else {
        return nil
    }
    let entity: NSEntityDescription = cdBorrow(entityPtr)
    let attributes = entity.attributesByName.values.sorted { $0.name < $1.name }
    return cdRetain(attributes as NSArray)
}

@_cdecl("cd_entity_description_relationships")
public func cdEntityDescriptionRelationships(_ entityPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let entityPtr else {
        return nil
    }
    let entity: NSEntityDescription = cdBorrow(entityPtr)
    let relationships = entity.relationshipsByName.values.sorted { $0.name < $1.name }
    return cdRetain(relationships as NSArray)
}

@_cdecl("cd_entity_description_relationships_with_destination_entity")
public func cdEntityDescriptionRelationshipsWithDestinationEntity(
    _ entityPtr: UnsafeMutableRawPointer?,
    _ destinationEntityPtr: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let entityPtr, let destinationEntityPtr else {
        return nil
    }
    let entity: NSEntityDescription = cdBorrow(entityPtr)
    let destination: NSEntityDescription = cdBorrow(destinationEntityPtr)
    return cdRetain(entity.relationships(forDestination: destination) as NSArray)
}

@_cdecl("cd_entity_description_is_kind_of_entity")
public func cdEntityDescriptionIsKindOfEntity(
    _ entityPtr: UnsafeMutableRawPointer?,
    _ otherEntityPtr: UnsafeMutableRawPointer?
) -> Int32 {
    guard let entityPtr, let otherEntityPtr else {
        return 0
    }
    let entity: NSEntityDescription = cdBorrow(entityPtr)
    let otherEntity: NSEntityDescription = cdBorrow(otherEntityPtr)
    var current: NSEntityDescription? = entity
    while let candidate = current {
        if candidate.name == otherEntity.name {
            return 1
        }
        current = candidate.superentity
    }
    return 0
}

@_cdecl("cd_attribute_description_new")
public func cdAttributeDescriptionNew(
    _ outAttribute: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outAttribute?.pointee = cdRetain(NSAttributeDescription())
    return CDR_OK
}

@_cdecl("cd_attribute_description_get_name")
public func cdAttributeDescriptionGetName(_ attributePtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let attributePtr else {
        return nil
    }
    let attribute: NSAttributeDescription = cdBorrow(attributePtr)
    return cdCString(attribute.name)
}

@_cdecl("cd_attribute_description_set_name")
public func cdAttributeDescriptionSetName(
    _ attributePtr: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let attributePtr, let name else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing attribute or attribute name")
        }
        let attribute: NSAttributeDescription = cdBorrow(attributePtr)
        try cdTrySetValue(String(cString: name), forKey: "name", on: attribute)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return cdErrorStatus(error)
    }
}

@_cdecl("cd_attribute_description_get_optional")
public func cdAttributeDescriptionGetOptional(_ attributePtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let attributePtr else {
        return 0
    }
    let attribute: NSAttributeDescription = cdBorrow(attributePtr)
    return attribute.isOptional ? 1 : 0
}

@_cdecl("cd_attribute_description_set_optional")
public func cdAttributeDescriptionSetOptional(
    _ attributePtr: UnsafeMutableRawPointer?,
    _ optional: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let attributePtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing attribute")
        }
        let attribute: NSAttributeDescription = cdBorrow(attributePtr)
        try cdTrySetValue(optional != 0, forKey: "optional", on: attribute)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return cdErrorStatus(error)
    }
}

@_cdecl("cd_attribute_description_get_transient")
public func cdAttributeDescriptionGetTransient(_ attributePtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let attributePtr else {
        return 0
    }
    let attribute: NSAttributeDescription = cdBorrow(attributePtr)
    return attribute.isTransient ? 1 : 0
}

@_cdecl("cd_attribute_description_set_transient")
public func cdAttributeDescriptionSetTransient(
    _ attributePtr: UnsafeMutableRawPointer?,
    _ transient: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let attributePtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing attribute")
        }
        let attribute: NSAttributeDescription = cdBorrow(attributePtr)
        try cdTrySetValue(transient != 0, forKey: "transient", on: attribute)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return cdErrorStatus(error)
    }
}

@_cdecl("cd_attribute_description_get_attribute_type")
public func cdAttributeDescriptionGetAttributeType(_ attributePtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let attributePtr else {
        return 0
    }
    let attribute: NSAttributeDescription = cdBorrow(attributePtr)
    return UInt64(attribute.attributeType.rawValue)
}

@_cdecl("cd_attribute_description_set_attribute_type")
public func cdAttributeDescriptionSetAttributeType(
    _ attributePtr: UnsafeMutableRawPointer?,
    _ attributeType: UInt64,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let attributePtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing attribute")
        }
        let typeRaw = try cdKnownRawValue(attributeType, in: CD_ATTRIBUTE_TYPE_RAW_VALUES, "attribute type")
        guard let type = NSAttributeType(rawValue: UInt(typeRaw)) else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Invalid attribute type: \(attributeType)")
        }
        let attribute: NSAttributeDescription = cdBorrow(attributePtr)
        try cdTrySetValue(type.rawValue, forKey: "attributeType", on: attribute)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return cdErrorStatus(error)
    }
}

@_cdecl("cd_attribute_description_get_attribute_value_class_name")
public func cdAttributeDescriptionGetAttributeValueClassName(_ attributePtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let attributePtr else {
        return nil
    }
    let attribute: NSAttributeDescription = cdBorrow(attributePtr)
    return attribute.attributeValueClassName.flatMap { cdCString($0) }
}

@_cdecl("cd_attribute_description_set_attribute_value_class_name")
public func cdAttributeDescriptionSetAttributeValueClassName(
    _ attributePtr: UnsafeMutableRawPointer?,
    _ className: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let attributePtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing attribute")
        }
        let attribute: NSAttributeDescription = cdBorrow(attributePtr)
        try cdTrySetValue(cdOptionalString(className), forKey: "attributeValueClassName", on: attribute)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return cdErrorStatus(error)
    }
}

@_cdecl("cd_attribute_description_get_default_value_json")
public func cdAttributeDescriptionGetDefaultValueJSON(
    _ attributePtr: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let attributePtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing attribute")
        }
        let attribute: NSAttributeDescription = cdBorrow(attributePtr)
        let payload = try cdValuePayload(from: attribute.defaultValue)
        outJSON?.pointee = cdCString(try cdEncodeJSON(payload))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return cdErrorStatus(error)
    }
}

@_cdecl("cd_attribute_description_set_default_value_json")
public func cdAttributeDescriptionSetDefaultValueJSON(
    _ attributePtr: UnsafeMutableRawPointer?,
    _ valueJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let attributePtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing attribute")
        }
        let attribute: NSAttributeDescription = cdBorrow(attributePtr)
        let payload = try cdDecodeJSON(valueJSON, as: CDValuePayload.self)
        try cdTrySetValue(cdFoundationValue(from: payload), forKey: "defaultValue", on: attribute)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return cdErrorStatus(error)
    }
}

@_cdecl("cd_attribute_description_get_value_transformer_name")
public func cdAttributeDescriptionGetValueTransformerName(_ attributePtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let attributePtr else {
        return nil
    }
    let attribute: NSAttributeDescription = cdBorrow(attributePtr)
    return attribute.valueTransformerName.flatMap { cdCString($0) }
}

@_cdecl("cd_attribute_description_set_value_transformer_name")
public func cdAttributeDescriptionSetValueTransformerName(
    _ attributePtr: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let attributePtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing attribute")
        }
        let attribute: NSAttributeDescription = cdBorrow(attributePtr)
        try cdTrySetValue(cdOptionalString(name), forKey: "valueTransformerName", on: attribute)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return cdErrorStatus(error)
    }
}

@_cdecl("cd_attribute_description_get_allows_external_binary_data_storage")
public func cdAttributeDescriptionGetAllowsExternalBinaryDataStorage(_ attributePtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let attributePtr else {
        return 0
    }
    let attribute: NSAttributeDescription = cdBorrow(attributePtr)
    return attribute.allowsExternalBinaryDataStorage ? 1 : 0
}

@_cdecl("cd_attribute_description_set_allows_external_binary_data_storage")
public func cdAttributeDescriptionSetAllowsExternalBinaryDataStorage(
    _ attributePtr: UnsafeMutableRawPointer?,
    _ allowsExternalStorage: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let attributePtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing attribute")
        }
        let attribute: NSAttributeDescription = cdBorrow(attributePtr)
        try cdTrySetValue(allowsExternalStorage != 0, forKey: "allowsExternalBinaryDataStorage", on: attribute)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return cdErrorStatus(error)
    }
}

@_cdecl("cd_attribute_description_get_preserves_value_in_history_on_deletion")
public func cdAttributeDescriptionGetPreservesValueInHistoryOnDeletion(_ attributePtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let attributePtr else {
        return 0
    }
    let attribute: NSAttributeDescription = cdBorrow(attributePtr)
    return attribute.preservesValueInHistoryOnDeletion ? 1 : 0
}

@_cdecl("cd_attribute_description_set_preserves_value_in_history_on_deletion")
public func cdAttributeDescriptionSetPreservesValueInHistoryOnDeletion(
    _ attributePtr: UnsafeMutableRawPointer?,
    _ preservesValue: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let attributePtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing attribute")
        }
        let attribute: NSAttributeDescription = cdBorrow(attributePtr)
        try cdTrySetValue(preservesValue != 0, forKey: "preservesValueInHistoryOnDeletion", on: attribute)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return cdErrorStatus(error)
    }
}

@_cdecl("cd_attribute_description_get_allows_cloud_encryption")
public func cdAttributeDescriptionGetAllowsCloudEncryption(_ attributePtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let attributePtr else {
        return 0
    }
    let attribute: NSAttributeDescription = cdBorrow(attributePtr)
    return attribute.allowsCloudEncryption ? 1 : 0
}

@_cdecl("cd_attribute_description_set_allows_cloud_encryption")
public func cdAttributeDescriptionSetAllowsCloudEncryption(
    _ attributePtr: UnsafeMutableRawPointer?,
    _ allowsEncryption: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let attributePtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing attribute")
        }
        let attribute: NSAttributeDescription = cdBorrow(attributePtr)
        try cdTrySetValue(allowsEncryption != 0, forKey: "allowsCloudEncryption", on: attribute)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return cdErrorStatus(error)
    }
}
