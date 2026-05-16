import CoreData
import Foundation

@_cdecl("cd_mapping_model_inferred")
public func cdMappingModelInferred(
    _ sourceModelPtr: UnsafeMutableRawPointer?,
    _ destinationModelPtr: UnsafeMutableRawPointer?,
    _ outModel: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let sourceModelPtr, let destinationModelPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing source or destination model")
        }
        let sourceModel: NSManagedObjectModel = cdBorrow(sourceModelPtr)
        let destinationModel: NSManagedObjectModel = cdBorrow(destinationModelPtr)
        let mappingModel = try NSMappingModel.inferredMappingModel(
            forSourceModel: sourceModel,
            destinationModel: destinationModel
        )
        outModel?.pointee = cdRetain(mappingModel)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_mapping_model_from_url")
public func cdMappingModelFromURL(
    _ path: UnsafePointer<CChar>?,
    _ outModel: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let path, let url = cdURL(from: path) else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing mapping model URL")
        }
        outModel?.pointee = NSMappingModel(contentsOf: url).map(cdRetain)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_mapping_model_get_entity_mapping_names_json")
public func cdMappingModelGetEntityMappingNamesJSON(
    _ modelPtr: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let modelPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing mapping model")
        }
        let model: NSMappingModel = cdBorrow(modelPtr)
        outJSON?.pointee = cdCString(try cdEncodeJSON(model.entityMappings.map(\.name)))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_migration_manager_new")
public func cdMigrationManagerNew(
    _ sourceModelPtr: UnsafeMutableRawPointer?,
    _ destinationModelPtr: UnsafeMutableRawPointer?,
    _ outManager: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let sourceModelPtr, let destinationModelPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing source or destination model")
        }
        let sourceModel: NSManagedObjectModel = cdBorrow(sourceModelPtr)
        let destinationModel: NSManagedObjectModel = cdBorrow(destinationModelPtr)
        outManager?.pointee = cdRetain(NSMigrationManager(sourceModel: sourceModel, destinationModel: destinationModel))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_migration_manager_get_source_model")
public func cdMigrationManagerGetSourceModel(_ managerPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let managerPtr else {
        return nil
    }
    let manager: NSMigrationManager = cdBorrow(managerPtr)
    return cdRetain(manager.sourceModel)
}

@_cdecl("cd_migration_manager_get_destination_model")
public func cdMigrationManagerGetDestinationModel(_ managerPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let managerPtr else {
        return nil
    }
    let manager: NSMigrationManager = cdBorrow(managerPtr)
    return cdRetain(manager.destinationModel)
}

@_cdecl("cd_migration_manager_get_source_context")
public func cdMigrationManagerGetSourceContext(_ managerPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let managerPtr else {
        return nil
    }
    let manager: NSMigrationManager = cdBorrow(managerPtr)
    return cdRetain(manager.sourceContext)
}

@_cdecl("cd_migration_manager_get_destination_context")
public func cdMigrationManagerGetDestinationContext(_ managerPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let managerPtr else {
        return nil
    }
    let manager: NSMigrationManager = cdBorrow(managerPtr)
    return cdRetain(manager.destinationContext)
}

@_cdecl("cd_migration_manager_get_uses_store_specific_migration_manager")
public func cdMigrationManagerGetUsesStoreSpecificMigrationManager(_ managerPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let managerPtr else {
        return 0
    }
    let manager: NSMigrationManager = cdBorrow(managerPtr)
    return manager.usesStoreSpecificMigrationManager ? 1 : 0
}

@_cdecl("cd_migration_manager_set_uses_store_specific_migration_manager")
public func cdMigrationManagerSetUsesStoreSpecificMigrationManager(
    _ managerPtr: UnsafeMutableRawPointer?,
    _ usesStoreSpecificMigrationManager: Int32
) {
    guard let managerPtr else {
        return
    }
    let manager: NSMigrationManager = cdBorrow(managerPtr)
    manager.usesStoreSpecificMigrationManager = usesStoreSpecificMigrationManager != 0
}

@_cdecl("cd_migration_manager_get_migration_progress")
public func cdMigrationManagerGetMigrationProgress(_ managerPtr: UnsafeMutableRawPointer?) -> Float {
    guard let managerPtr else {
        return 0
    }
    let manager: NSMigrationManager = cdBorrow(managerPtr)
    return manager.migrationProgress
}

@_cdecl("cd_migration_manager_migrate_store")
public func cdMigrationManagerMigrateStore(
    _ managerPtr: UnsafeMutableRawPointer?,
    _ sourceURLPath: UnsafePointer<CChar>?,
    _ sourceStoreType: UnsafePointer<CChar>?,
    _ sourceOptionsJSON: UnsafePointer<CChar>?,
    _ mappingModelPtr: UnsafeMutableRawPointer?,
    _ destinationURLPath: UnsafePointer<CChar>?,
    _ destinationStoreType: UnsafePointer<CChar>?,
    _ destinationOptionsJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let managerPtr,
              let sourceURLPath,
              let sourceURL = cdURL(from: sourceURLPath),
              let sourceStoreType,
              let destinationURLPath,
              let destinationURL = cdURL(from: destinationURLPath),
              let destinationStoreType
        else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing migration manager or migration URLs/store types")
        }
        let manager: NSMigrationManager = cdBorrow(managerPtr)
        let sourceOptions = try cdPersistentStoreOptions(from: sourceOptionsJSON)
        let destinationOptions = try cdPersistentStoreOptions(from: destinationOptionsJSON)
        let mappingModel = mappingModelPtr.map { cdBorrow($0) as NSMappingModel }
        try manager.migrateStore(
            from: sourceURL,
            sourceType: String(cString: sourceStoreType),
            options: sourceOptions,
            with: mappingModel,
            toDestinationURL: destinationURL,
            destinationType: String(cString: destinationStoreType),
            destinationOptions: destinationOptions
        )
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}
