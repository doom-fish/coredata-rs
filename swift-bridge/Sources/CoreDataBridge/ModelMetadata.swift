import CoreData
import Foundation

@_cdecl("cd_property_description_new")
public func cdPropertyDescriptionNew(
    _ outProperty: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outProperty?.pointee = cdRetain(NSPropertyDescription())
    return CDR_OK
}

@_cdecl("cd_property_description_get_name")
public func cdPropertyDescriptionGetName(_ propertyPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let propertyPtr else {
        return nil
    }
    let property: NSPropertyDescription = cdBorrow(propertyPtr)
    return cdCString(property.name)
}

@_cdecl("cd_property_description_set_name")
public func cdPropertyDescriptionSetName(
    _ propertyPtr: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let propertyPtr, let name else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing property description or name")
        }
        let property: NSPropertyDescription = cdBorrow(propertyPtr)
        property.name = String(cString: name)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_property_description_get_optional")
public func cdPropertyDescriptionGetOptional(_ propertyPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let propertyPtr else {
        return 0
    }
    let property: NSPropertyDescription = cdBorrow(propertyPtr)
    return property.isOptional ? 1 : 0
}

@_cdecl("cd_property_description_set_optional")
public func cdPropertyDescriptionSetOptional(
    _ propertyPtr: UnsafeMutableRawPointer?,
    _ optional: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let propertyPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing property description")
        }
        let property: NSPropertyDescription = cdBorrow(propertyPtr)
        property.isOptional = optional != 0
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_property_description_get_transient")
public func cdPropertyDescriptionGetTransient(_ propertyPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let propertyPtr else {
        return 0
    }
    let property: NSPropertyDescription = cdBorrow(propertyPtr)
    return property.isTransient ? 1 : 0
}

@_cdecl("cd_property_description_set_transient")
public func cdPropertyDescriptionSetTransient(
    _ propertyPtr: UnsafeMutableRawPointer?,
    _ transient: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let propertyPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing property description")
        }
        let property: NSPropertyDescription = cdBorrow(propertyPtr)
        property.isTransient = transient != 0
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_fetched_property_description_new")
public func cdFetchedPropertyDescriptionNew(
    _ outProperty: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outProperty?.pointee = cdRetain(NSFetchedPropertyDescription())
    return CDR_OK
}

@_cdecl("cd_fetched_property_description_get_fetch_request")
public func cdFetchedPropertyDescriptionGetFetchRequest(_ propertyPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let propertyPtr else {
        return nil
    }
    let property: NSFetchedPropertyDescription = cdBorrow(propertyPtr)
    return property.fetchRequest.map(cdRetain)
}

@_cdecl("cd_fetched_property_description_set_fetch_request")
public func cdFetchedPropertyDescriptionSetFetchRequest(
    _ propertyPtr: UnsafeMutableRawPointer?,
    _ fetchRequestPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let propertyPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing fetched property description")
        }
        let property: NSFetchedPropertyDescription = cdBorrow(propertyPtr)
        property.fetchRequest = fetchRequestPtr.map { cdBorrow($0) as NSFetchRequest<NSFetchRequestResult> }
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_expression_description_new")
public func cdExpressionDescriptionNew(
    _ outProperty: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outProperty?.pointee = cdRetain(NSExpressionDescription())
    return CDR_OK
}

@_cdecl("cd_expression_description_get_result_type")
public func cdExpressionDescriptionGetResultType(_ propertyPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let propertyPtr else {
        return 0
    }
    let property: NSExpressionDescription = cdBorrow(propertyPtr)
    return UInt64(property.expressionResultType.rawValue)
}

@_cdecl("cd_expression_description_set_result_type")
public func cdExpressionDescriptionSetResultType(
    _ propertyPtr: UnsafeMutableRawPointer?,
    _ resultType: UInt64,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let propertyPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing expression description")
        }
        let property: NSExpressionDescription = cdBorrow(propertyPtr)
        guard let resultType = NSAttributeType(rawValue: UInt(resultType)) else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Invalid expression result type")
        }
        property.expressionResultType = resultType
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_derived_attribute_description_new")
public func cdDerivedAttributeDescriptionNew(
    _ outAttribute: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outAttribute?.pointee = cdRetain(NSDerivedAttributeDescription())
    return CDR_OK
}

@_cdecl("cd_composite_attribute_description_new")
public func cdCompositeAttributeDescriptionNew(
    _ outAttribute: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 14.0, *) else {
        cdWriteError(cdUnavailableFeatureError("NSCompositeAttributeDescription"), to: outError)
        return CDR_FAILURE
    }
    outAttribute?.pointee = cdRetain(NSCompositeAttributeDescription())
    return CDR_OK
}

@_cdecl("cd_composite_attribute_description_get_elements")
public func cdCompositeAttributeDescriptionGetElements(_ attributePtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard #available(macOS 14.0, *), let attributePtr else {
        return nil
    }
    let attribute: NSCompositeAttributeDescription = cdBorrow(attributePtr)
    return cdRetain(attribute.elements as NSArray)
}

@_cdecl("cd_composite_attribute_description_set_elements")
public func cdCompositeAttributeDescriptionSetElements(
    _ attributePtr: UnsafeMutableRawPointer?,
    _ elementPtrs: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 14.0, *) else {
            throw cdUnavailableFeatureError("NSCompositeAttributeDescription")
        }
        guard let attributePtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing composite attribute description")
        }
        let attribute: NSCompositeAttributeDescription = cdBorrow(attributePtr)
        let elements: [NSAttributeDescription] = cdObjects(from: elementPtrs, count: count)
        attribute.elements = elements
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_fetch_index_element_description_new")
public func cdFetchIndexElementDescriptionNew(
    _ propertyPtr: UnsafeMutableRawPointer?,
    _ collationType: UInt64,
    _ outElement: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let propertyPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing fetch index element property")
        }
        guard let collationType = NSFetchIndexElementType(rawValue: UInt(collationType)) else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Invalid fetch index element collation type")
        }
        let property: NSPropertyDescription = cdBorrow(propertyPtr)
        outElement?.pointee = cdRetain(NSFetchIndexElementDescription(property: property, collationType: collationType))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_fetch_index_element_description_get_property_name")
public func cdFetchIndexElementDescriptionGetPropertyName(_ elementPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let elementPtr else {
        return nil
    }
    let element: NSFetchIndexElementDescription = cdBorrow(elementPtr)
    return element.propertyName.flatMap(cdCString)
}

@_cdecl("cd_fetch_index_element_description_get_collation_type")
public func cdFetchIndexElementDescriptionGetCollationType(_ elementPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let elementPtr else {
        return 0
    }
    let element: NSFetchIndexElementDescription = cdBorrow(elementPtr)
    return UInt64(element.collationType.rawValue)
}

@_cdecl("cd_fetch_index_element_description_set_collation_type")
public func cdFetchIndexElementDescriptionSetCollationType(
    _ elementPtr: UnsafeMutableRawPointer?,
    _ collationType: UInt64,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let elementPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing fetch index element description")
        }
        guard let collationType = NSFetchIndexElementType(rawValue: UInt(collationType)) else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Invalid fetch index element collation type")
        }
        let element: NSFetchIndexElementDescription = cdBorrow(elementPtr)
        element.collationType = collationType
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_fetch_index_element_description_get_ascending")
public func cdFetchIndexElementDescriptionGetAscending(_ elementPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let elementPtr else {
        return 0
    }
    let element: NSFetchIndexElementDescription = cdBorrow(elementPtr)
    return element.isAscending ? 1 : 0
}

@_cdecl("cd_fetch_index_element_description_set_ascending")
public func cdFetchIndexElementDescriptionSetAscending(
    _ elementPtr: UnsafeMutableRawPointer?,
    _ ascending: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let elementPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing fetch index element description")
        }
        let element: NSFetchIndexElementDescription = cdBorrow(elementPtr)
        element.isAscending = ascending != 0
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_fetch_index_description_new")
public func cdFetchIndexDescriptionNew(
    _ name: UnsafePointer<CChar>?,
    _ elementPtrs: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int32,
    _ outIndex: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let name else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing fetch index description name")
        }
        let elements: [NSFetchIndexElementDescription] = cdObjects(from: elementPtrs, count: count)
        outIndex?.pointee = cdRetain(NSFetchIndexDescription(name: String(cString: name), elements: elements))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_fetch_index_description_get_name")
public func cdFetchIndexDescriptionGetName(_ indexPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let indexPtr else {
        return nil
    }
    let index: NSFetchIndexDescription = cdBorrow(indexPtr)
    return cdCString(index.name)
}

@_cdecl("cd_fetch_index_description_set_name")
public func cdFetchIndexDescriptionSetName(
    _ indexPtr: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let indexPtr, let name else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing fetch index description or name")
        }
        let index: NSFetchIndexDescription = cdBorrow(indexPtr)
        index.name = String(cString: name)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_fetch_index_description_get_elements")
public func cdFetchIndexDescriptionGetElements(_ indexPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let indexPtr else {
        return nil
    }
    let index: NSFetchIndexDescription = cdBorrow(indexPtr)
    return cdRetain(index.elements as NSArray)
}

@_cdecl("cd_fetch_index_description_set_elements")
public func cdFetchIndexDescriptionSetElements(
    _ indexPtr: UnsafeMutableRawPointer?,
    _ elementPtrs: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let indexPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing fetch index description")
        }
        let index: NSFetchIndexDescription = cdBorrow(indexPtr)
        let elements: [NSFetchIndexElementDescription] = cdObjects(from: elementPtrs, count: count)
        index.elements = elements
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}
