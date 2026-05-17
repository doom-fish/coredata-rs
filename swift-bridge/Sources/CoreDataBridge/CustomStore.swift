import CoreData
import Foundation

@_cdecl("cd_atomic_store_cache_node_new")
public func cdAtomicStoreCacheNodeNew(
    _ objectIDPtr: UnsafeMutableRawPointer?,
    _ outNode: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let objectIDPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object ID")
        }
        let objectID: NSManagedObjectID = cdBorrow(objectIDPtr)
        outNode?.pointee = cdRetain(NSAtomicStoreCacheNode(objectID: objectID))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_atomic_store_cache_node_get_object_id")
public func cdAtomicStoreCacheNodeGetObjectID(_ nodePtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let nodePtr else {
        return nil
    }
    let node: NSAtomicStoreCacheNode = cdBorrow(nodePtr)
    return cdRetain(node.objectID)
}

@_cdecl("cd_incremental_store_node_new")
public func cdIncrementalStoreNodeNew(
    _ objectIDPtr: UnsafeMutableRawPointer?,
    _ valuesJSON: UnsafePointer<CChar>?,
    _ version: UInt64,
    _ outNode: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let objectIDPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object ID")
        }
        let objectID: NSManagedObjectID = cdBorrow(objectIDPtr)
        let values = try cdDictionary(from: valuesJSON) ?? [:]
        outNode?.pointee = cdRetain(NSIncrementalStoreNode(objectID: objectID, withValues: values, version: version))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_incremental_store_node_get_object_id")
public func cdIncrementalStoreNodeGetObjectID(_ nodePtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let nodePtr else {
        return nil
    }
    let node: NSIncrementalStoreNode = cdBorrow(nodePtr)
    return cdRetain(node.objectID)
}

@_cdecl("cd_incremental_store_node_get_version")
public func cdIncrementalStoreNodeGetVersion(_ nodePtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let nodePtr else {
        return 0
    }
    let node: NSIncrementalStoreNode = cdBorrow(nodePtr)
    return node.version
}

@_cdecl("cd_incremental_store_node_update")
public func cdIncrementalStoreNodeUpdate(
    _ nodePtr: UnsafeMutableRawPointer?,
    _ valuesJSON: UnsafePointer<CChar>?,
    _ version: UInt64,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let nodePtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing incremental store node")
        }
        let node: NSIncrementalStoreNode = cdBorrow(nodePtr)
        let values = try cdDictionary(from: valuesJSON) ?? [:]
        node.update(withValues: values, version: version)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}
