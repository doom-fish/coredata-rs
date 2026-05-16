import CoreData
import Foundation

func cdValidationRulePayloads(for property: NSPropertyDescription) -> [CDValidationRulePayload] {
    let predicates = property.validationPredicates
    let warnings = property.validationWarnings.compactMap { $0 as? String }
    return zip(predicates, warnings).map { predicate, warning in
        CDValidationRulePayload(predicateFormat: predicate.predicateFormat, warning: warning)
    }
}

func cdSetValidationRules(_ rules: [CDValidationRulePayload], on property: NSPropertyDescription) {
    let predicates = rules.map { NSPredicate(format: $0.predicateFormat) }
    let warnings = rules.map(\.warning)
    property.setValidationPredicates(predicates, withValidationWarnings: warnings)
}

@_cdecl("cd_attribute_description_get_validation_rules_json")
public func cdAttributeDescriptionGetValidationRulesJSON(
    _ attributePtr: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let attributePtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing attribute")
        }
        let attribute: NSAttributeDescription = cdBorrow(attributePtr)
        outJSON?.pointee = cdCString(try cdEncodeJSON(cdValidationRulePayloads(for: attribute)))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_attribute_description_set_validation_rules_json")
public func cdAttributeDescriptionSetValidationRulesJSON(
    _ attributePtr: UnsafeMutableRawPointer?,
    _ rulesJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let attributePtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing attribute")
        }
        let attribute: NSAttributeDescription = cdBorrow(attributePtr)
        let rules = try cdDecodeJSON(rulesJSON, as: [CDValidationRulePayload].self)
        cdSetValidationRules(rules, on: attribute)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_relationship_description_get_validation_rules_json")
public func cdRelationshipDescriptionGetValidationRulesJSON(
    _ relationshipPtr: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let relationshipPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing relationship")
        }
        let relationship: NSRelationshipDescription = cdBorrow(relationshipPtr)
        outJSON?.pointee = cdCString(try cdEncodeJSON(cdValidationRulePayloads(for: relationship)))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_relationship_description_set_validation_rules_json")
public func cdRelationshipDescriptionSetValidationRulesJSON(
    _ relationshipPtr: UnsafeMutableRawPointer?,
    _ rulesJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let relationshipPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing relationship")
        }
        let relationship: NSRelationshipDescription = cdBorrow(relationshipPtr)
        let rules = try cdDecodeJSON(rulesJSON, as: [CDValidationRulePayload].self)
        cdSetValidationRules(rules, on: relationship)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_managed_object_validate_value_json")
public func cdManagedObjectValidateValueJSON(
    _ objectPtr: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ valueJSON: UnsafePointer<CChar>?,
    _ outValidatedJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let objectPtr, let key else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object or validation key")
        }
        let object: NSManagedObject = cdBorrow(objectPtr)
        let payload = try cdDecodeJSON(valueJSON, as: CDValuePayload.self)
        var candidate: AnyObject? = cdFoundationValue(from: payload) as AnyObject?
        try object.validateValue(&candidate, forKey: String(cString: key))
        let validated = try cdValuePayload(from: candidate)
        outValidatedJSON?.pointee = cdCString(try cdEncodeJSON(validated))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_managed_object_validate_for_insert")
public func cdManagedObjectValidateForInsert(
    _ objectPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let objectPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object")
        }
        let object: NSManagedObject = cdBorrow(objectPtr)
        try object.validateForInsert()
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_managed_object_validate_for_update")
public func cdManagedObjectValidateForUpdate(
    _ objectPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let objectPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object")
        }
        let object: NSManagedObject = cdBorrow(objectPtr)
        try object.validateForUpdate()
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_managed_object_validate_for_delete")
public func cdManagedObjectValidateForDelete(
    _ objectPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let objectPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object")
        }
        let object: NSManagedObject = cdBorrow(objectPtr)
        try object.validateForDelete()
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}
