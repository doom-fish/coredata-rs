import CoreData
import Foundation

@_cdecl("cd_managed_object_model_new")
public func cdManagedObjectModelNew(
    _ outModel: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outModel?.pointee = cdRetain(NSManagedObjectModel())
    return CDR_OK
}

@_cdecl("cd_managed_object_model_from_url")
public func cdManagedObjectModelFromURL(
    _ path: UnsafePointer<CChar>?,
    _ outModel: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let path else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object model path")
        }
        let url = URL(fileURLWithPath: String(cString: path))
        guard let model = NSManagedObjectModel(contentsOf: url) else {
            throw cdBridgeNSError(code: CDR_FAILURE, message: "Failed to load managed object model from \(url.path)")
        }
        outModel?.pointee = cdRetain(model)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_managed_object_model_add_entity")
public func cdManagedObjectModelAddEntity(
    _ modelPtr: UnsafeMutableRawPointer?,
    _ entityPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let modelPtr, let entityPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing model or entity")
        }
        let model: NSManagedObjectModel = cdBorrow(modelPtr)
        let entity: NSEntityDescription = cdBorrow(entityPtr)
        var entities = model.entities
        entities.removeAll { existing in existing.name == entity.name }
        entities.append(entity)
        model.entities = entities
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_managed_object_model_entities")
public func cdManagedObjectModelEntities(_ modelPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let modelPtr else {
        return nil
    }
    let model: NSManagedObjectModel = cdBorrow(modelPtr)
    let entities = model.entities.sorted { ($0.name ?? "") < ($1.name ?? "") }
    return cdRetain(entities as NSArray)
}

@_cdecl("cd_entity_description_new")
public func cdEntityDescriptionNew(
    _ outEntity: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outEntity?.pointee = cdRetain(NSEntityDescription())
    return CDR_OK
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
        entity.name = String(cString: name)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
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
        entity.managedObjectClassName = String(cString: name)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
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
        entity.properties = properties
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
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
        entity.properties = properties
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
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
        attribute.name = String(cString: name)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
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
        attribute.isOptional = optional != 0
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
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
        attribute.isTransient = transient != 0
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
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
        guard let type = NSAttributeType(rawValue: UInt(attributeType)) else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Invalid attribute type: \(attributeType)")
        }
        let attribute: NSAttributeDescription = cdBorrow(attributePtr)
        attribute.attributeType = type
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_relationship_description_new")
public func cdRelationshipDescriptionNew(
    _ outRelationship: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outRelationship?.pointee = cdRetain(NSRelationshipDescription())
    return CDR_OK
}

@_cdecl("cd_relationship_description_get_name")
public func cdRelationshipDescriptionGetName(_ relationshipPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let relationshipPtr else {
        return nil
    }
    let relationship: NSRelationshipDescription = cdBorrow(relationshipPtr)
    return cdCString(relationship.name)
}

@_cdecl("cd_relationship_description_set_name")
public func cdRelationshipDescriptionSetName(
    _ relationshipPtr: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let relationshipPtr, let name else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing relationship or relationship name")
        }
        let relationship: NSRelationshipDescription = cdBorrow(relationshipPtr)
        relationship.name = String(cString: name)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_relationship_description_get_optional")
public func cdRelationshipDescriptionGetOptional(_ relationshipPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let relationshipPtr else {
        return 0
    }
    let relationship: NSRelationshipDescription = cdBorrow(relationshipPtr)
    return relationship.isOptional ? 1 : 0
}

@_cdecl("cd_relationship_description_set_optional")
public func cdRelationshipDescriptionSetOptional(
    _ relationshipPtr: UnsafeMutableRawPointer?,
    _ optional: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let relationshipPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing relationship")
        }
        let relationship: NSRelationshipDescription = cdBorrow(relationshipPtr)
        relationship.isOptional = optional != 0
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_relationship_description_get_transient")
public func cdRelationshipDescriptionGetTransient(_ relationshipPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let relationshipPtr else {
        return 0
    }
    let relationship: NSRelationshipDescription = cdBorrow(relationshipPtr)
    return relationship.isTransient ? 1 : 0
}

@_cdecl("cd_relationship_description_set_transient")
public func cdRelationshipDescriptionSetTransient(
    _ relationshipPtr: UnsafeMutableRawPointer?,
    _ transient: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let relationshipPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing relationship")
        }
        let relationship: NSRelationshipDescription = cdBorrow(relationshipPtr)
        relationship.isTransient = transient != 0
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_relationship_description_get_destination_entity")
public func cdRelationshipDescriptionGetDestinationEntity(_ relationshipPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let relationshipPtr else {
        return nil
    }
    let relationship: NSRelationshipDescription = cdBorrow(relationshipPtr)
    guard let entity = relationship.destinationEntity else {
        return nil
    }
    return cdRetain(entity)
}

@_cdecl("cd_relationship_description_set_destination_entity")
public func cdRelationshipDescriptionSetDestinationEntity(
    _ relationshipPtr: UnsafeMutableRawPointer?,
    _ entityPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let relationshipPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing relationship")
        }
        let relationship: NSRelationshipDescription = cdBorrow(relationshipPtr)
        relationship.destinationEntity = entityPtr.map { cdBorrow($0) as NSEntityDescription }
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_relationship_description_get_inverse_relationship")
public func cdRelationshipDescriptionGetInverseRelationship(_ relationshipPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let relationshipPtr else {
        return nil
    }
    let relationship: NSRelationshipDescription = cdBorrow(relationshipPtr)
    guard let inverse = relationship.inverseRelationship else {
        return nil
    }
    return cdRetain(inverse)
}

@_cdecl("cd_relationship_description_set_inverse_relationship")
public func cdRelationshipDescriptionSetInverseRelationship(
    _ relationshipPtr: UnsafeMutableRawPointer?,
    _ inverseRelationshipPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let relationshipPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing relationship")
        }
        let relationship: NSRelationshipDescription = cdBorrow(relationshipPtr)
        relationship.inverseRelationship = inverseRelationshipPtr.map { cdBorrow($0) as NSRelationshipDescription }
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_relationship_description_get_min_count")
public func cdRelationshipDescriptionGetMinCount(_ relationshipPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let relationshipPtr else {
        return 0
    }
    let relationship: NSRelationshipDescription = cdBorrow(relationshipPtr)
    return UInt64(relationship.minCount)
}

@_cdecl("cd_relationship_description_set_min_count")
public func cdRelationshipDescriptionSetMinCount(
    _ relationshipPtr: UnsafeMutableRawPointer?,
    _ minCount: UInt64,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let relationshipPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing relationship")
        }
        let relationship: NSRelationshipDescription = cdBorrow(relationshipPtr)
        relationship.minCount = Int(minCount)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_relationship_description_get_max_count")
public func cdRelationshipDescriptionGetMaxCount(_ relationshipPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let relationshipPtr else {
        return 0
    }
    let relationship: NSRelationshipDescription = cdBorrow(relationshipPtr)
    return UInt64(relationship.maxCount)
}

@_cdecl("cd_relationship_description_set_max_count")
public func cdRelationshipDescriptionSetMaxCount(
    _ relationshipPtr: UnsafeMutableRawPointer?,
    _ maxCount: UInt64,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let relationshipPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing relationship")
        }
        let relationship: NSRelationshipDescription = cdBorrow(relationshipPtr)
        relationship.maxCount = Int(maxCount)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_relationship_description_get_delete_rule")
public func cdRelationshipDescriptionGetDeleteRule(_ relationshipPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let relationshipPtr else {
        return 0
    }
    let relationship: NSRelationshipDescription = cdBorrow(relationshipPtr)
    return UInt64(relationship.deleteRule.rawValue)
}

@_cdecl("cd_relationship_description_set_delete_rule")
public func cdRelationshipDescriptionSetDeleteRule(
    _ relationshipPtr: UnsafeMutableRawPointer?,
    _ deleteRule: UInt64,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let relationshipPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing relationship")
        }
        guard let deleteRule = NSDeleteRule(rawValue: UInt(deleteRule)) else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Invalid delete rule: \(deleteRule)")
        }
        let relationship: NSRelationshipDescription = cdBorrow(relationshipPtr)
        relationship.deleteRule = deleteRule
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}
