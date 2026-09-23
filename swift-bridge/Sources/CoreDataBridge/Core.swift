import CoreData
import CoreDataObjCBridge
import Foundation

class CDObjectBox: NSObject {
    let object: AnyObject

    init(_ object: AnyObject) {
        self.object = object
    }
}

let CDR_OK: Int32 = 0
let CDR_INVALID_ARGUMENT: Int32 = -1
let CDR_FAILURE: Int32 = -2
let CDR_TIMED_OUT: Int32 = -3
let CDR_WRONG_THREAD: Int32 = -4
let CDR_OBJC_EXCEPTION: Int32 = -5
let CDR_INVALIDATED: Int32 = -6
let CDR_BRIDGE_ERROR_DOMAIN = "CoreDataBridge"

public typealias CDOutError = UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
public typealias CDOutString = UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
public typealias CDOutObject = UnsafeMutablePointer<UnsafeMutableRawPointer?>?

struct CDErrorPayload: Codable {
    var domain: String
    var code: Int
    var message: String
}

@_cdecl("cd_string_free")
public func cdStringFree(_ string: UnsafeMutablePointer<CChar>?) {
    free(string)
}

@_cdecl("cd_retain_object")
public func cdRetainObject(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let ptr else {
        return nil
    }
    let box = cdBorrowBox(ptr)
    return Unmanaged.passRetained(box).toOpaque()
}

@_cdecl("cd_release_object")
public func cdReleaseObject(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else {
        return
    }
    let typed = ptr.assumingMemoryBound(to: CDObjectBox.self)
    Unmanaged<CDObjectBox>.fromOpaque(UnsafeRawPointer(typed)).release()
}

@_cdecl("cd_array_count")
public func cdArrayCount(_ arrayPtr: UnsafeMutableRawPointer?) -> Int64 {
    guard let arrayPtr, let array = cdBorrowBox(arrayPtr).object as? NSArray else {
        return 0
    }
    return Int64(array.count)
}

@_cdecl("cd_array_get_object")
public func cdArrayGetObject(_ arrayPtr: UnsafeMutableRawPointer?, _ index: Int64) -> UnsafeMutableRawPointer? {
    guard let arrayPtr,
          let array = cdBorrowBox(arrayPtr).object as? NSArray,
          let position = Int(exactly: index),
          position >= 0 && position < array.count
    else {
        return nil
    }
    let element = array[position] as AnyObject
    if let box = element as? CDObjectBox {
        return Unmanaged.passRetained(box).toOpaque()
    }
    return cdRetain(element)
}

@inline(__always)
func cdCString(_ string: String) -> UnsafeMutablePointer<CChar>? {
    string.withCString { strdup($0) }
}

@inline(__always)
func cdOptionalString(_ cString: UnsafePointer<CChar>?) -> String? {
    cString.map(String.init(cString:))
}

@inline(__always)
func cdURL(from path: UnsafePointer<CChar>?) -> URL? {
    guard let path else {
        return nil
    }
    return URL(fileURLWithPath: String(cString: path))
}

@inline(__always)
func cdRetain(_ object: some AnyObject) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(CDObjectBox(object)).toOpaque()
}

@inline(__always)
func cdBorrowBox(_ ptr: UnsafeMutableRawPointer) -> CDObjectBox {
    let typed = ptr.assumingMemoryBound(to: CDObjectBox.self)
    return Unmanaged<CDObjectBox>.fromOpaque(UnsafeRawPointer(typed)).takeUnretainedValue()
}

func cdArg<T: AnyObject>(_ ptr: UnsafeMutableRawPointer?, _ what: String) throws -> T {
    guard let ptr else {
        throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing \(what)")
    }
    guard let object = cdBorrowBox(ptr).object as? T else {
        throw cdBridgeNSError(
            code: CDR_INVALID_ARGUMENT,
            message: "Expected \(what), got \(type(of: cdBorrowBox(ptr).object))"
        )
    }
    return object
}

@inline(__always)
func cdBorrow<T: AnyObject>(_ ptr: UnsafeMutableRawPointer) -> T {
    guard let object = cdBorrowBox(ptr).object as? T else {
        fatalError("Unexpected Core Data bridge object type: \(type(of: cdBorrowBox(ptr).object))")
    }
    return object
}

func cdObjects<T: AnyObject>(
    from pointers: UnsafePointer<UnsafeMutableRawPointer?>?,
    count: Int32
) -> [T] {
    guard let pointers, count > 0 else {
        return []
    }
    let safeCount = Int(count)
    var objects: [T] = []
    objects.reserveCapacity(safeCount)
    for index in 0..<safeCount {
        guard let pointer = pointers[index] else {
            continue
        }
        objects.append(cdBorrow(pointer))
    }
    return objects
}

func cdHexString(_ data: Data) -> String {
    data.map { String(format: "%02x", $0) }.joined()
}

func cdBridgeNSError(code: Int32, message: String) -> NSError {
    NSError(domain: CDR_BRIDGE_ERROR_DOMAIN, code: Int(code), userInfo: [NSLocalizedDescriptionKey: message])
}

func cdUnavailableFeatureError(_ feature: String) -> NSError {
    cdBridgeNSError(code: CDR_FAILURE, message: "\(feature) requires macOS 14.0 or newer")
}

func cdSilentFailureError() -> NSError {
    cdBridgeNSError(code: CDR_FAILURE, message: "Core Data reported a failure without an error")
}

func cdErrorStatus(_ error: NSError) -> Int32 {
    let status = Int32(clamping: error.code)
    return status == CDR_OK ? CDR_FAILURE : status
}

func cdStatus(
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ body: () throws -> Void
) -> Int32 {
    do {
        try body()
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return cdErrorStatus(error)
    }
}

func cdObjCCall(_ body: (NSErrorPointer) -> Bool) throws {
    var error: NSError?
    guard body(&error) else {
        throw error ?? cdSilentFailureError()
    }
}

func cdObjCValue<T>(_ body: (NSErrorPointer) -> T?) throws -> T {
    var error: NSError?
    guard let value = body(&error) else {
        throw error ?? cdSilentFailureError()
    }
    return value
}

let CD_ATTRIBUTE_TYPE_RAW_VALUES: [Int64] = [0, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200, 1800, 2000, 2100]

func cdKnownRawValue<R: BinaryInteger>(_ raw: R, in known: [Int64], _ what: String) throws -> Int64 {
    guard let value = Int64(exactly: raw), known.contains(value) else {
        throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Unsupported \(what) \(raw)")
    }
    return value
}

func cdTrySetValue(_ value: Any?, forKey key: String, on object: NSObject) throws {
    try cdObjCCall { CDXSetValueForKey(object, value, key, $0) }
}

func cdWriteError(_ error: NSError, to outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?) {
    guard let outError else {
        return
    }
    let reported = error.domain == "Foundation._GenericObjCError" ? cdSilentFailureError() : error
    let payload = CDErrorPayload(domain: reported.domain, code: reported.code, message: reported.localizedDescription)
    let json = (try? cdEncodeJSON(payload))
        ?? "{\"domain\":\"CoreDataBridge\",\"code\":-2,\"message\":\"Unknown Core Data bridge error\"}"
    outError.pointee = cdCString(json)
}

func cdPersistentStoreOptionKey(_ key: String) -> String {
    switch key {
    case "NSReadOnlyPersistentStoreOption":
        return NSReadOnlyPersistentStoreOption
    case "NSMigratePersistentStoresAutomaticallyOption":
        return NSMigratePersistentStoresAutomaticallyOption
    case "NSInferMappingModelAutomaticallyOption":
        return NSInferMappingModelAutomaticallyOption
    case "NSSQLitePragmasOption":
        return NSSQLitePragmasOption
    case "NSSQLiteAnalyzeOption":
        return NSSQLiteAnalyzeOption
    case "NSSQLiteManualVacuumOption":
        return NSSQLiteManualVacuumOption
    case "NSIgnorePersistentStoreVersioningOption":
        return NSIgnorePersistentStoreVersioningOption
    case "NSPersistentStoreForceDestroyOption":
        return NSPersistentStoreForceDestroyOption
    case "NSPersistentStoreDeferredLightweightMigrationOptionKey":
        return NSPersistentStoreDeferredLightweightMigrationOptionKey
    case "NSPersistentHistoryTrackingKey":
        return NSPersistentHistoryTrackingKey
    case "NSPersistentStoreRemoteChangeNotificationPostOptionKey":
        return NSPersistentStoreRemoteChangeNotificationPostOptionKey
    case "NSValidateXMLStoreOption":
        return NSValidateXMLStoreOption
    default:
        return key
    }
}
