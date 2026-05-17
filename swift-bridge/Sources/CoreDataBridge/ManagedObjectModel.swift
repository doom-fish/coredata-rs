import CoreData
import Foundation

@_cdecl("cd_managed_object_model_new")
public func cdManagedObjectModelNew(
    _ outModel: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outModel?.pointee = cdRetain(NSManagedObjectModel())
    return CDR_OK
}

@_cdecl("cd_managed_object_model_from_url")
public func cdManagedObjectModelFromURL(
    _ path: UnsafePointer<CChar>?,
    _ outModel: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let path else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object model path")
        }
        let url = URL(fileURLWithPath: String(cString: path))
        guard let model = NSManagedObjectModel(contentsOf: url) else {
            throw cdBridgeNSError(code: CDR_FAILURE, message: "Failed to load managed object model from \(url.path)")
        }
        outModel?.pointee = cdRetain(model)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_managed_object_model_add_entity")
public func cdManagedObjectModelAddEntity(
    _ modelPtr: UnsafeMutableRawPointer?,
    _ entityPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let modelPtr, let entityPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing model or entity")
        }
        let model: NSManagedObjectModel = cdBorrow(modelPtr)
        let entity: NSEntityDescription = cdBorrow(entityPtr)
        var entities = model.entities
        entities.removeAll { existing in existing.name == entity.name }
        entities.append(entity)
        model.entities = entities
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_managed_object_model_entities")
public func cdManagedObjectModelEntities(_ modelPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let modelPtr else {
        return nil
    }
    let model: NSManagedObjectModel = cdBorrow(modelPtr)
    let entities = model.entities.sorted { ($0.name ?? "") < ($1.name ?? "") }
    return cdRetain(entities as NSArray)
}

@_cdecl("cd_managed_object_model_get_version_checksum")
public func cdManagedObjectModelGetVersionChecksum(_ modelPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard #available(macOS 14.0, *), let modelPtr else {
        return nil
    }
    let model: NSManagedObjectModel = cdBorrow(modelPtr)
    return cdCString(model.versionChecksum)
}
