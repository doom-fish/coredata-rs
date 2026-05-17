import CoreData
import Foundation

@_cdecl("cd_entity_mapping_new")
public func cdEntityMappingNew(
    _ outMapping: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outMapping?.pointee = cdRetain(NSEntityMapping())
    return CDR_OK
}

@_cdecl("cd_entity_mapping_get_name")
public func cdEntityMappingGetName(_ mappingPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let mappingPtr else {
        return nil
    }
    let mapping: NSEntityMapping = cdBorrow(mappingPtr)
    return cdCString(mapping.name)
}

@_cdecl("cd_entity_mapping_set_name")
public func cdEntityMappingSetName(
    _ mappingPtr: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let mappingPtr, let name else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing entity mapping or name")
        }
        let mapping: NSEntityMapping = cdBorrow(mappingPtr)
        mapping.name = String(cString: name)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_entity_mapping_get_mapping_type")
public func cdEntityMappingGetMappingType(_ mappingPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let mappingPtr else {
        return 0
    }
    let mapping: NSEntityMapping = cdBorrow(mappingPtr)
    return UInt64(mapping.mappingType.rawValue)
}

@_cdecl("cd_entity_mapping_set_mapping_type")
public func cdEntityMappingSetMappingType(
    _ mappingPtr: UnsafeMutableRawPointer?,
    _ mappingType: UInt64,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let mappingPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing entity mapping")
        }
        guard let mappingType = NSEntityMappingType(rawValue: UInt(mappingType)) else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Invalid entity mapping type")
        }
        let mapping: NSEntityMapping = cdBorrow(mappingPtr)
        mapping.mappingType = mappingType
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_entity_mapping_get_source_entity_name")
public func cdEntityMappingGetSourceEntityName(_ mappingPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let mappingPtr else {
        return nil
    }
    let mapping: NSEntityMapping = cdBorrow(mappingPtr)
    return mapping.sourceEntityName.flatMap(cdCString)
}

@_cdecl("cd_entity_mapping_set_source_entity_name")
public func cdEntityMappingSetSourceEntityName(
    _ mappingPtr: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let mappingPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing entity mapping")
        }
        let mapping: NSEntityMapping = cdBorrow(mappingPtr)
        mapping.sourceEntityName = cdOptionalString(name)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_entity_mapping_get_destination_entity_name")
public func cdEntityMappingGetDestinationEntityName(_ mappingPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let mappingPtr else {
        return nil
    }
    let mapping: NSEntityMapping = cdBorrow(mappingPtr)
    return mapping.destinationEntityName.flatMap(cdCString)
}

@_cdecl("cd_entity_mapping_set_destination_entity_name")
public func cdEntityMappingSetDestinationEntityName(
    _ mappingPtr: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let mappingPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing entity mapping")
        }
        let mapping: NSEntityMapping = cdBorrow(mappingPtr)
        mapping.destinationEntityName = cdOptionalString(name)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_property_mapping_new")
public func cdPropertyMappingNew(
    _ outMapping: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outMapping?.pointee = cdRetain(NSPropertyMapping())
    return CDR_OK
}

@_cdecl("cd_property_mapping_get_name")
public func cdPropertyMappingGetName(_ mappingPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let mappingPtr else {
        return nil
    }
    let mapping: NSPropertyMapping = cdBorrow(mappingPtr)
    return mapping.name.flatMap(cdCString)
}

@_cdecl("cd_property_mapping_set_name")
public func cdPropertyMappingSetName(
    _ mappingPtr: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let mappingPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing property mapping")
        }
        let mapping: NSPropertyMapping = cdBorrow(mappingPtr)
        mapping.name = cdOptionalString(name)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_managed_object_model_reference_new_with_model")
public func cdManagedObjectModelReferenceNewWithModel(
    _ modelPtr: UnsafeMutableRawPointer?,
    _ versionChecksum: UnsafePointer<CChar>?,
    _ outReference: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 14.0, *) else {
            throw cdUnavailableFeatureError("NSManagedObjectModelReference")
        }
        guard let modelPtr, let versionChecksum else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing model or version checksum")
        }
        let model: NSManagedObjectModel = cdBorrow(modelPtr)
        outReference?.pointee = cdRetain(NSManagedObjectModelReference(
            model: model,
            versionChecksum: String(cString: versionChecksum)
        ))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_managed_object_model_reference_get_resolved_model")
public func cdManagedObjectModelReferenceGetResolvedModel(_ referencePtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard #available(macOS 14.0, *), let referencePtr else {
        return nil
    }
    let reference: NSManagedObjectModelReference = cdBorrow(referencePtr)
    return cdRetain(reference.resolvedModel)
}

@_cdecl("cd_managed_object_model_reference_get_version_checksum")
public func cdManagedObjectModelReferenceGetVersionChecksum(_ referencePtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard #available(macOS 14.0, *), let referencePtr else {
        return nil
    }
    let reference: NSManagedObjectModelReference = cdBorrow(referencePtr)
    return cdCString(reference.versionChecksum)
}

@_cdecl("cd_migration_stage_get_label")
public func cdMigrationStageGetLabel(_ stagePtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard #available(macOS 14.0, *), let stagePtr else {
        return nil
    }
    let stage: NSMigrationStage = cdBorrow(stagePtr)
    return cdCString(stage.label)
}

@_cdecl("cd_migration_stage_set_label")
public func cdMigrationStageSetLabel(
    _ stagePtr: UnsafeMutableRawPointer?,
    _ label: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 14.0, *) else {
            throw cdUnavailableFeatureError("NSMigrationStage")
        }
        guard let stagePtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing migration stage")
        }
        let stage: NSMigrationStage = cdBorrow(stagePtr)
        stage.label = cdOptionalString(label) ?? ""
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_lightweight_migration_stage_new")
public func cdLightweightMigrationStageNew(
    _ versionChecksumsJSON: UnsafePointer<CChar>?,
    _ outStage: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 14.0, *) else {
            throw cdUnavailableFeatureError("NSLightweightMigrationStage")
        }
        let versionChecksums = try cdStringArray(from: versionChecksumsJSON)
        outStage?.pointee = cdRetain(NSLightweightMigrationStage(versionChecksums))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_lightweight_migration_stage_get_version_checksums_json")
public func cdLightweightMigrationStageGetVersionChecksumsJSON(
    _ stagePtr: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 14.0, *) else {
            throw cdUnavailableFeatureError("NSLightweightMigrationStage")
        }
        guard let stagePtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing lightweight migration stage")
        }
        let stage: NSLightweightMigrationStage = cdBorrow(stagePtr)
        outJSON?.pointee = cdCString(try cdEncodeJSON(stage.versionChecksums))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_custom_migration_stage_new")
public func cdCustomMigrationStageNew(
    _ currentModelPtr: UnsafeMutableRawPointer?,
    _ nextModelPtr: UnsafeMutableRawPointer?,
    _ outStage: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 14.0, *) else {
            throw cdUnavailableFeatureError("NSCustomMigrationStage")
        }
        guard let currentModelPtr, let nextModelPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing custom migration stage model references")
        }
        let currentModel: NSManagedObjectModelReference = cdBorrow(currentModelPtr)
        let nextModel: NSManagedObjectModelReference = cdBorrow(nextModelPtr)
        outStage?.pointee = cdRetain(NSCustomMigrationStage(
            __currentModelReference: currentModel,
            nextModelReference: nextModel
        ))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_custom_migration_stage_get_current_model")
public func cdCustomMigrationStageGetCurrentModel(_ stagePtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard #available(macOS 14.0, *), let stagePtr else {
        return nil
    }
    let stage: NSCustomMigrationStage = cdBorrow(stagePtr)
    return cdRetain(stage.currentModel)
}

@_cdecl("cd_custom_migration_stage_get_next_model")
public func cdCustomMigrationStageGetNextModel(_ stagePtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard #available(macOS 14.0, *), let stagePtr else {
        return nil
    }
    let stage: NSCustomMigrationStage = cdBorrow(stagePtr)
    return cdRetain(stage.nextModel)
}

@_cdecl("cd_staged_migration_manager_new")
public func cdStagedMigrationManagerNew(
    _ stagePtrs: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int32,
    _ outManager: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 14.0, *) else {
            throw cdUnavailableFeatureError("NSStagedMigrationManager")
        }
        let stages: [NSMigrationStage] = cdObjects(from: stagePtrs, count: count)
        outManager?.pointee = cdRetain(NSStagedMigrationManager(stages))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_staged_migration_manager_get_stages")
public func cdStagedMigrationManagerGetStages(_ managerPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard #available(macOS 14.0, *), let managerPtr else {
        return nil
    }
    let manager: NSStagedMigrationManager = cdBorrow(managerPtr)
    return cdRetain(manager.stages as NSArray)
}

@_cdecl("cd_staged_migration_manager_get_container")
public func cdStagedMigrationManagerGetContainer(_ managerPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard #available(macOS 14.0, *), let managerPtr else {
        return nil
    }
    let manager: NSStagedMigrationManager = cdBorrow(managerPtr)
    return manager.container.map(cdRetain)
}
