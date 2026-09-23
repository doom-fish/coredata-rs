import CoreDataObjCBridge
import Foundation

@_cdecl("cd_predicate_new_with_format")
public func cdPredicateNewWithFormat(
    _ format: UnsafePointer<CChar>?,
    _ argumentsJSON: UnsafePointer<CChar>?,
    _ outPredicate: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        guard let format else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing predicate format")
        }
        let formatString = String(cString: format)
        let arguments = try argumentsJSON.map {
            try cdDecodeJSON($0, as: [CDValuePayload].self).map(cdPredicateArgument)
        }
        let predicate = try cdObjCValue { CDXPredicateWithFormat(formatString, arguments, $0) }
        outPredicate?.pointee = cdRetain(predicate)
    }
}

func cdPredicateOperatorType(_ raw: UInt64) throws -> NSComparisonPredicate.Operator {
    let supported: [NSComparisonPredicate.Operator] = [
        .lessThan, .lessThanOrEqualTo, .greaterThan, .greaterThanOrEqualTo, .equalTo, .notEqualTo,
        .matches, .like, .beginsWith, .endsWith, .contains,
    ]
    guard let match = supported.first(where: { UInt64($0.rawValue) == raw }) else {
        throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Unsupported predicate operator \(raw)")
    }
    return match
}

@_cdecl("cd_predicate_new_comparison")
public func cdPredicateNewComparison(
    _ keyPath: UnsafePointer<CChar>?,
    _ operatorType: UInt64,
    _ valueJSON: UnsafePointer<CChar>?,
    _ options: UInt64,
    _ outPredicate: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        guard let keyPath else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing predicate key path")
        }
        let keyPathString = String(cString: keyPath)
        guard !keyPathString.isEmpty else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Empty predicate key path")
        }
        let allowedOptions: UInt64 = UInt64(NSComparisonPredicate.Options.caseInsensitive.rawValue)
            | UInt64(NSComparisonPredicate.Options.diacriticInsensitive.rawValue)
            | UInt64(NSComparisonPredicate.Options.normalized.rawValue)
        guard options & ~allowedOptions == 0 else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Unsupported predicate options \(options)")
        }
        let operatorKind = try cdPredicateOperatorType(operatorType)
        let payload = try cdDecodeJSON(valueJSON, as: CDValuePayload.self)
        let predicate = NSComparisonPredicate(
            leftExpression: NSExpression(forKeyPath: keyPathString),
            rightExpression: NSExpression(forConstantValue: cdFoundationValue(from: payload)),
            modifier: .direct,
            type: operatorKind,
            options: NSComparisonPredicate.Options(rawValue: UInt(options))
        )
        outPredicate?.pointee = cdRetain(predicate)
    }
}

@_cdecl("cd_predicate_new_compound")
public func cdPredicateNewCompound(
    _ compoundType: UInt64,
    _ predicatePtrs: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int32,
    _ outPredicate: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        guard count >= 0, count == 0 || predicatePtrs != nil else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Invalid predicate list")
        }
        let subpredicates: [Foundation.NSPredicate] = try (0..<Int(count)).map {
            try cdArg(predicatePtrs?[$0], "predicate")
        }
        let predicate: NSCompoundPredicate
        switch compoundType {
        case UInt64(NSCompoundPredicate.LogicalType.not.rawValue):
            guard subpredicates.count == 1 else {
                throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "A NOT predicate takes exactly one subpredicate")
            }
            predicate = NSCompoundPredicate(notPredicateWithSubpredicate: subpredicates[0])
        case UInt64(NSCompoundPredicate.LogicalType.and.rawValue):
            predicate = NSCompoundPredicate(andPredicateWithSubpredicates: subpredicates)
        case UInt64(NSCompoundPredicate.LogicalType.or.rawValue):
            predicate = NSCompoundPredicate(orPredicateWithSubpredicates: subpredicates)
        default:
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Unsupported compound predicate type \(compoundType)")
        }
        outPredicate?.pointee = cdRetain(predicate)
    }
}

@_cdecl("cd_predicate_new_with_value")
public func cdPredicateNewWithValue(
    _ value: Int32,
    _ outPredicate: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outPredicate?.pointee = cdRetain(Foundation.NSPredicate(value: value != 0))
    return CDR_OK
}

@_cdecl("cd_predicate_get_format")
public func cdPredicateGetFormat(_ predicatePtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let predicatePtr else {
        return nil
    }
    let predicate: Foundation.NSPredicate = cdBorrow(predicatePtr)
    return cdCString(predicate.predicateFormat)
}

@_cdecl("cd_predicate_with_substitution_variables")
public func cdPredicateWithSubstitutionVariables(
    _ predicatePtr: UnsafeMutableRawPointer?,
    _ variablesJSON: UnsafePointer<CChar>?,
    _ outPredicate: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let predicate: Foundation.NSPredicate = try cdArg(predicatePtr, "predicate")
        let variables = try cdDictionary(from: variablesJSON) ?? [:]
        let substituted = try cdObjCValue { CDXPredicateWithSubstitutionVariables(predicate, variables, $0) }
        outPredicate?.pointee = cdRetain(substituted)
    }
}

@_cdecl("cd_predicate_evaluate_with_object_json")
public func cdPredicateEvaluateWithObjectJSON(
    _ predicatePtr: UnsafeMutableRawPointer?,
    _ objectJSON: UnsafePointer<CChar>?,
    _ substitutionVariablesJSON: UnsafePointer<CChar>?,
    _ outResult: UnsafeMutablePointer<Int32>?,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let predicate: Foundation.NSPredicate = try cdArg(predicatePtr, "predicate")
        let object = try cdDictionary(from: objectJSON)
        let substitutionVariables = try cdDictionary(from: substitutionVariablesJSON)
        var result: ObjCBool = false
        try cdObjCCall { CDXEvaluatePredicate(predicate, object, substitutionVariables, &result, $0) }
        outResult?.pointee = result.boolValue ? 1 : 0
    }
}
