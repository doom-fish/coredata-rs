import Foundation

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
    _ outPredicate: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let predicatePtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing predicate")
        }
        let predicate: Foundation.NSPredicate = cdBorrow(predicatePtr)
        let variables = try cdDictionary(from: variablesJSON) ?? [:]
        outPredicate?.pointee = cdRetain(predicate.withSubstitutionVariables(variables))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_predicate_evaluate_with_object_json")
public func cdPredicateEvaluateWithObjectJSON(
    _ predicatePtr: UnsafeMutableRawPointer?,
    _ objectJSON: UnsafePointer<CChar>?,
    _ substitutionVariablesJSON: UnsafePointer<CChar>?,
    _ outResult: UnsafeMutablePointer<Int32>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let predicatePtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing predicate")
        }
        let predicate: Foundation.NSPredicate = cdBorrow(predicatePtr)
        let object = try cdDictionary(from: objectJSON)
        let substitutionVariables = try cdDictionary(from: substitutionVariablesJSON)
        let result: Bool
        if let substitutionVariables {
            result = predicate.evaluate(with: object, substitutionVariables: substitutionVariables)
        } else {
            result = predicate.evaluate(with: object)
        }
        outResult?.pointee = result ? 1 : 0
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}
