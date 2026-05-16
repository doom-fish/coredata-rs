import Foundation

enum CDValueKind: String, Codable {
    case null
    case string
    case int32
    case int64
    case double
    case bool
}

struct CDValuePayload: Codable {
    var kind: CDValueKind
    var stringValue: String?
    var int32Value: Int32?
    var int64Value: Int64?
    var doubleValue: Double?
    var boolValue: Bool?
}

struct CDSortDescriptorPayload: Codable {
    var key: String
    var ascending: Bool
}

struct CDValidationRulePayload: Codable {
    var predicateFormat: String
    var warning: String
}

func cdEncodeJSON<T: Encodable>(_ value: T) throws -> String {
    let data = try JSONEncoder().encode(value)
    guard let string = String(data: data, encoding: .utf8) else {
        throw cdBridgeNSError(code: CDR_FAILURE, message: "Failed to encode JSON as UTF-8")
    }
    return string
}

func cdDecodeJSON<T: Decodable>(_ cString: UnsafePointer<CChar>?, as _: T.Type) throws -> T {
    guard let cString else {
        throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing JSON payload")
    }
    let data = Data(String(cString: cString).utf8)
    do {
        return try JSONDecoder().decode(T.self, from: data)
    } catch {
        throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Invalid JSON payload: \(error.localizedDescription)")
    }
}

func cdFoundationValue(from payload: CDValuePayload) -> Any? {
    switch payload.kind {
    case .null:
        return nil
    case .string:
        return payload.stringValue
    case .int32:
        return payload.int32Value.map(NSNumber.init(value:))
    case .int64:
        return payload.int64Value.map(NSNumber.init(value:))
    case .double:
        return payload.doubleValue.map(NSNumber.init(value:))
    case .bool:
        return payload.boolValue.map(NSNumber.init(value:))
    }
}

func cdPredicateArgument(from payload: CDValuePayload) -> Any {
    cdFoundationValue(from: payload) ?? NSNull()
}

func cdValuePayload(from value: Any?) throws -> CDValuePayload {
    guard let value else {
        return CDValuePayload(kind: .null, stringValue: nil, int32Value: nil, int64Value: nil, doubleValue: nil, boolValue: nil)
    }

    if value is NSNull {
        return CDValuePayload(kind: .null, stringValue: nil, int32Value: nil, int64Value: nil, doubleValue: nil, boolValue: nil)
    }

    if let value = value as? String {
        return CDValuePayload(kind: .string, stringValue: value, int32Value: nil, int64Value: nil, doubleValue: nil, boolValue: nil)
    }

    if let number = value as? NSNumber {
        if CFGetTypeID(number) == CFBooleanGetTypeID() {
            return CDValuePayload(kind: .bool, stringValue: nil, int32Value: nil, int64Value: nil, doubleValue: nil, boolValue: number.boolValue)
        }

        let typeCode = String(cString: number.objCType)
        switch typeCode {
        case "c", "s", "i":
            return CDValuePayload(kind: .int32, stringValue: nil, int32Value: number.int32Value, int64Value: nil, doubleValue: nil, boolValue: nil)
        case "l", "q", "I", "L", "Q":
            return CDValuePayload(kind: .int64, stringValue: nil, int32Value: nil, int64Value: number.int64Value, doubleValue: nil, boolValue: nil)
        case "f", "d":
            return CDValuePayload(kind: .double, stringValue: nil, int32Value: nil, int64Value: nil, doubleValue: number.doubleValue, boolValue: nil)
        default:
            if number.doubleValue.rounded(.towardZero) == number.doubleValue {
                return CDValuePayload(kind: .int64, stringValue: nil, int32Value: nil, int64Value: number.int64Value, doubleValue: nil, boolValue: nil)
            }
            return CDValuePayload(kind: .double, stringValue: nil, int32Value: nil, int64Value: nil, doubleValue: number.doubleValue, boolValue: nil)
        }
    }

    throw cdBridgeNSError(code: CDR_FAILURE, message: "Unsupported value type: \(type(of: value))")
}

func cdSortDescriptors(from json: UnsafePointer<CChar>?) throws -> [NSSortDescriptor] {
    let payloads = try cdDecodeJSON(json, as: [CDSortDescriptorPayload].self)
    return payloads.map { NSSortDescriptor(key: $0.key, ascending: $0.ascending) }
}

func cdDictionary(from json: UnsafePointer<CChar>?) throws -> [String: Any]? {
    guard let json else {
        return nil
    }
    let payload = try cdDecodeJSON(json, as: [String: CDValuePayload].self)
    var dictionary: [String: Any] = [:]
    dictionary.reserveCapacity(payload.count)
    for (key, value) in payload {
        dictionary[key] = cdFoundationValue(from: value) ?? NSNull()
    }
    return dictionary
}

func cdPersistentStoreOptions(from json: UnsafePointer<CChar>?) throws -> [String: Any]? {
    guard let json else {
        return nil
    }
    let payload = try cdDecodeJSON(json, as: [String: CDValuePayload].self)
    var dictionary: [String: Any] = [:]
    dictionary.reserveCapacity(payload.count)
    for (key, value) in payload {
        dictionary[cdPersistentStoreOptionKey(key)] = cdFoundationValue(from: value) ?? NSNull()
    }
    return dictionary
}

func cdValuePayloadMap(from dictionary: [String: Any]) throws -> [String: CDValuePayload] {
    var payloads: [String: CDValuePayload] = [:]
    payloads.reserveCapacity(dictionary.count)
    for (key, value) in dictionary {
        payloads[key] = try cdValuePayload(from: value)
    }
    return payloads
}

func cdStringDictionary(from json: UnsafePointer<CChar>?) throws -> [String: String]? {
    guard let json else {
        return nil
    }
    return try cdDecodeJSON(json, as: [String: String].self)
}

func cdStringArray(from json: UnsafePointer<CChar>?) throws -> [String] {
    guard let json else {
        return []
    }
    return try cdDecodeJSON(json, as: [String].self)
}

func cdStringMatrix(from json: UnsafePointer<CChar>?) throws -> [[String]] {
    guard let json else {
        return []
    }
    return try cdDecodeJSON(json, as: [[String]].self)
}

func cdStringMapPayload(from dictionary: [String: Any]?) -> [String: String] {
    guard let dictionary else {
        return [:]
    }
    return dictionary.mapValues { String(describing: $0) }
}
