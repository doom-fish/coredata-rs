import CoreData
import CoreDataObjCBridge
import Foundation

func cdValidationRulePayloads(for property: NSPropertyDescription) -> [CDValidationRulePayload] {
    let predicates = property.validationPredicates
    let warnings = property.validationWarnings.compactMap { $0 as? String }
    return zip(predicates, warnings).map { predicate, warning in
        CDValidationRulePayload(predicateFormat: predicate.predicateFormat, warning: warning)
    }
}

func cdSetValidationRules(_ rules: [CDValidationRulePayload], on property: NSPropertyDescription) throws {
    let predicates = try rules.map { rule in
        try cdObjCValue { CDXPredicateWithFormat(rule.predicateFormat, nil, $0) }
    }
    let warnings = rules.map(\.warning)
    try cdObjCCall { CDXSetValidationPredicates(property, predicates, warnings, $0) }
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
        return cdErrorStatus(error)
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
        try cdSetValidationRules(rules, on: attribute)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return cdErrorStatus(error)
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
        return cdErrorStatus(error)
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
        try cdSetValidationRules(rules, on: relationship)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return cdErrorStatus(error)
    }
}

@_cdecl("cd_managed_object_validate_value_json")
public func cdManagedObjectValidateValueJSON(
    _ objectPtr: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ valueJSON: UnsafePointer<CChar>?,
    _ outValidatedJSON: CDOutString,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let payload = try cdDecodeJSON(valueJSON, as: CDValuePayload.self)
        let validated = try cdWithManagedObject(objectPtr) { object, _ -> CDValuePayload in
            let name = try cdAttributeKey(key, of: object)
            var candidate: AnyObject? = cdFoundationValue(from: payload) as AnyObject?
            try object.validateValue(&candidate, forKey: name)
            return try cdValuePayload(from: candidate)
        }
        outValidatedJSON?.pointee = cdCString(try cdEncodeJSON(validated))
    }
}

@_cdecl("cd_managed_object_validate_for_insert")
public func cdManagedObjectValidateForInsert(
    _ objectPtr: UnsafeMutableRawPointer?,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        try cdWithManagedObject(objectPtr) { object, _ in try object.validateForInsert() }
    }
}

@_cdecl("cd_managed_object_validate_for_update")
public func cdManagedObjectValidateForUpdate(
    _ objectPtr: UnsafeMutableRawPointer?,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        try cdWithManagedObject(objectPtr) { object, _ in try object.validateForUpdate() }
    }
}

@_cdecl("cd_managed_object_validate_for_delete")
public func cdManagedObjectValidateForDelete(
    _ objectPtr: UnsafeMutableRawPointer?,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        try cdWithManagedObject(objectPtr) { object, _ in try object.validateForDelete() }
    }
}
