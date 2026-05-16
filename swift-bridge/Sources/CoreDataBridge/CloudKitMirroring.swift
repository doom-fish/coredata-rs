import CloudKit
import CoreData
import Foundation

@_cdecl("cd_persistent_cloudkit_container_options_new")
public func cdPersistentCloudKitContainerOptionsNew(
    _ containerIdentifier: UnsafePointer<CChar>?,
    _ outOptions: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let containerIdentifier else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing CloudKit container identifier")
        }
        outOptions?.pointee = cdRetain(NSPersistentCloudKitContainerOptions(containerIdentifier: String(cString: containerIdentifier)))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_persistent_cloudkit_container_options_get_container_identifier")
public func cdPersistentCloudKitContainerOptionsGetContainerIdentifier(_ optionsPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let optionsPtr else {
        return nil
    }
    let options: NSPersistentCloudKitContainerOptions = cdBorrow(optionsPtr)
    return cdCString(options.containerIdentifier)
}

@_cdecl("cd_persistent_cloudkit_container_options_get_database_scope")
public func cdPersistentCloudKitContainerOptionsGetDatabaseScope(_ optionsPtr: UnsafeMutableRawPointer?) -> Int64 {
    guard let optionsPtr else {
        return 0
    }
    let options: NSPersistentCloudKitContainerOptions = cdBorrow(optionsPtr)
    return Int64(options.databaseScope.rawValue)
}

@_cdecl("cd_persistent_cloudkit_container_options_set_database_scope")
public func cdPersistentCloudKitContainerOptionsSetDatabaseScope(
    _ optionsPtr: UnsafeMutableRawPointer?,
    _ databaseScope: Int64,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let optionsPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing CloudKit container options")
        }
        guard let scope = CKDatabase.Scope(rawValue: Int(databaseScope)) else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Invalid CloudKit database scope: \(databaseScope)")
        }
        let options: NSPersistentCloudKitContainerOptions = cdBorrow(optionsPtr)
        options.databaseScope = scope
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_persistent_store_description_get_cloudkit_container_options")
public func cdPersistentStoreDescriptionGetCloudKitContainerOptions(_ descriptionPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let descriptionPtr else {
        return nil
    }
    let description: NSPersistentStoreDescription = cdBorrow(descriptionPtr)
    return description.cloudKitContainerOptions.map(cdRetain)
}

@_cdecl("cd_persistent_store_description_set_cloudkit_container_options")
public func cdPersistentStoreDescriptionSetCloudKitContainerOptions(
    _ descriptionPtr: UnsafeMutableRawPointer?,
    _ optionsPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let descriptionPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing store description")
        }
        let description: NSPersistentStoreDescription = cdBorrow(descriptionPtr)
        description.cloudKitContainerOptions = optionsPtr.map { cdBorrow($0) as NSPersistentCloudKitContainerOptions }
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_persistent_cloudkit_container_new")
public func cdPersistentCloudKitContainerNew(
    _ name: UnsafePointer<CChar>?,
    _ modelPtr: UnsafeMutableRawPointer?,
    _ outContainer: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let name, let modelPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing CloudKit container name or model")
        }
        let model: NSManagedObjectModel = cdBorrow(modelPtr)
        let container = NSPersistentCloudKitContainer(name: String(cString: name), managedObjectModel: model)
        outContainer?.pointee = cdRetain(container)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_persistent_cloudkit_container_get_name")
public func cdPersistentCloudKitContainerGetName(_ containerPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let containerPtr else {
        return nil
    }
    let container: NSPersistentCloudKitContainer = cdBorrow(containerPtr)
    return cdCString(container.name)
}

@_cdecl("cd_persistent_cloudkit_container_managed_object_model")
public func cdPersistentCloudKitContainerManagedObjectModel(_ containerPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let containerPtr else {
        return nil
    }
    let container: NSPersistentCloudKitContainer = cdBorrow(containerPtr)
    return cdRetain(container.managedObjectModel)
}

@_cdecl("cd_persistent_cloudkit_container_persistent_store_coordinator")
public func cdPersistentCloudKitContainerPersistentStoreCoordinator(_ containerPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let containerPtr else {
        return nil
    }
    let container: NSPersistentCloudKitContainer = cdBorrow(containerPtr)
    return cdRetain(container.persistentStoreCoordinator)
}

@_cdecl("cd_persistent_cloudkit_container_persistent_store_descriptions")
public func cdPersistentCloudKitContainerPersistentStoreDescriptions(_ containerPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let containerPtr else {
        return nil
    }
    let container: NSPersistentCloudKitContainer = cdBorrow(containerPtr)
    return cdRetain(container.persistentStoreDescriptions as NSArray)
}

@_cdecl("cd_persistent_cloudkit_container_set_persistent_store_descriptions")
public func cdPersistentCloudKitContainerSetPersistentStoreDescriptions(
    _ containerPtr: UnsafeMutableRawPointer?,
    _ descriptionPtrs: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let containerPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing CloudKit container")
        }
        let container: NSPersistentCloudKitContainer = cdBorrow(containerPtr)
        let descriptions: [NSPersistentStoreDescription] = cdObjects(from: descriptionPtrs, count: count)
        container.persistentStoreDescriptions = descriptions
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_persistent_cloudkit_container_load_persistent_stores")
public func cdPersistentCloudKitContainerLoadPersistentStores(
    _ containerPtr: UnsafeMutableRawPointer?,
    _ timeoutSeconds: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let containerPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing CloudKit container")
        }
        let container: NSPersistentCloudKitContainer = cdBorrow(containerPtr)
        let tracker = CDLoadTracker(expectedCount: container.persistentStoreDescriptions.count)
        container.loadPersistentStores { _, error in
            tracker.record(error: error as NSError?)
        }
        if let error = tracker.wait(timeoutSeconds: timeoutSeconds) {
            throw error
        }
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_persistent_cloudkit_container_view_context")
public func cdPersistentCloudKitContainerViewContext(_ containerPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let containerPtr else {
        return nil
    }
    let container: NSPersistentCloudKitContainer = cdBorrow(containerPtr)
    return cdRetain(container.viewContext)
}

@_cdecl("cd_persistent_cloudkit_container_new_background_context")
public func cdPersistentCloudKitContainerNewBackgroundContext(_ containerPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let containerPtr else {
        return nil
    }
    let container: NSPersistentCloudKitContainer = cdBorrow(containerPtr)
    return cdRetain(container.newBackgroundContext())
}

@_cdecl("cd_persistent_cloudkit_container_initialize_schema")
public func cdPersistentCloudKitContainerInitializeSchema(
    _ containerPtr: UnsafeMutableRawPointer?,
    _ optionsRawValue: UInt64,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let containerPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing CloudKit container")
        }
        let container: NSPersistentCloudKitContainer = cdBorrow(containerPtr)
        let options = NSPersistentCloudKitContainerSchemaInitializationOptions(rawValue: UInt(optionsRawValue))
        try container.initializeCloudKitSchema(options: options)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}
