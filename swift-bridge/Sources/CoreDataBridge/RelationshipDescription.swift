import CoreData
import Foundation

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
    return relationship.destinationEntity.map(cdRetain)
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
    return relationship.inverseRelationship.map(cdRetain)
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

@_cdecl("cd_relationship_description_get_to_many")
public func cdRelationshipDescriptionGetToMany(_ relationshipPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let relationshipPtr else {
        return 0
    }
    let relationship: NSRelationshipDescription = cdBorrow(relationshipPtr)
    return relationship.isToMany ? 1 : 0
}

@_cdecl("cd_relationship_description_get_ordered")
public func cdRelationshipDescriptionGetOrdered(_ relationshipPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let relationshipPtr else {
        return 0
    }
    let relationship: NSRelationshipDescription = cdBorrow(relationshipPtr)
    return relationship.isOrdered ? 1 : 0
}

@_cdecl("cd_relationship_description_set_ordered")
public func cdRelationshipDescriptionSetOrdered(
    _ relationshipPtr: UnsafeMutableRawPointer?,
    _ ordered: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let relationshipPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing relationship")
        }
        let relationship: NSRelationshipDescription = cdBorrow(relationshipPtr)
        relationship.isOrdered = ordered != 0
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_relationship_description_get_version_hash")
public func cdRelationshipDescriptionGetVersionHash(_ relationshipPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let relationshipPtr else {
        return nil
    }
    let relationship: NSRelationshipDescription = cdBorrow(relationshipPtr)
    return cdCString(cdHexString(relationship.versionHash))
}
