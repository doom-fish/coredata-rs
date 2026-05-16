import CoreData
import Foundation

final class CDObjectBox: NSObject {
    let object: AnyObject

    init(_ object: AnyObject) {
        self.object = object
    }
}

let CDR_OK: Int32 = 0
let CDR_INVALID_ARGUMENT: Int32 = -1
let CDR_FAILURE: Int32 = -2
let CDR_TIMED_OUT: Int32 = -3
let CDR_BRIDGE_ERROR_DOMAIN = "CoreDataBridge"

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
public func cdArrayCount(_ arrayPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let arrayPtr else {
        return 0
    }
    let array: NSArray = cdBorrow(arrayPtr)
    return Int32(array.count)
}

@_cdecl("cd_array_get_object")
public func cdArrayGetObject(_ arrayPtr: UnsafeMutableRawPointer?, _ index: Int32) -> UnsafeMutableRawPointer? {
    guard let arrayPtr else {
        return nil
    }
    let array: NSArray = cdBorrow(arrayPtr)
    let position = Int(index)
    guard position >= 0 && position < array.count,
          let object = array[position] as AnyObject?
    else {
        return nil
    }
    return cdRetain(object)
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

func cdWriteError(_ error: NSError, to outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?) {
    guard let outError else {
        return
    }
    let payload = CDErrorPayload(domain: error.domain, code: error.code, message: error.localizedDescription)
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
    case "NSPersistentHistoryTrackingKey":
        return NSPersistentHistoryTrackingKey
    case "NSPersistentStoreRemoteChangeNotificationPostOptionKey":
        return NSPersistentStoreRemoteChangeNotificationPostOptionKey
    default:
        return key
    }
}
