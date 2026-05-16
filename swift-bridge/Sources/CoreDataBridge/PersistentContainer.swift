import CoreData
import Foundation

final class CDLoadTracker {
    private let lock = NSLock()
    private let expectedCount: Int
    private var completionCount = 0
    private var firstError: NSError?
    private let semaphore = DispatchSemaphore(value: 0)

    init(expectedCount: Int) {
        self.expectedCount = max(expectedCount, 1)
    }

    func record(error: NSError?) {
        lock.lock()
        defer { lock.unlock() }
        if firstError == nil {
            firstError = error
        }
        completionCount += 1
        if completionCount >= expectedCount {
            semaphore.signal()
        }
    }

    func wait(timeoutSeconds: Int32) -> NSError? {
        if semaphore.wait(timeout: .now() + .seconds(Int(timeoutSeconds))) == .timedOut {
            return cdBridgeNSError(code: CDR_TIMED_OUT, message: "Timed out waiting for loadPersistentStores")
        }
        return firstError
    }
}

@_cdecl("cd_persistent_store_description_new")
public func cdPersistentStoreDescriptionNew(
    _ path: UnsafePointer<CChar>?,
    _ outDescription: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let description = cdURL(from: path).map(NSPersistentStoreDescription.init(url:)) ?? NSPersistentStoreDescription()
    outDescription?.pointee = cdRetain(description)
    return CDR_OK
}

@_cdecl("cd_persistent_store_description_get_type")
public func cdPersistentStoreDescriptionGetType(_ descriptionPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let descriptionPtr else {
        return nil
    }
    let description: NSPersistentStoreDescription = cdBorrow(descriptionPtr)
    return cdCString(description.type)
}

@_cdecl("cd_persistent_store_description_set_type")
public func cdPersistentStoreDescriptionSetType(
    _ descriptionPtr: UnsafeMutableRawPointer?,
    _ storeType: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let descriptionPtr, let storeType else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing store description or store type")
        }
        let description: NSPersistentStoreDescription = cdBorrow(descriptionPtr)
        description.type = String(cString: storeType)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_persistent_store_description_get_configuration")
public func cdPersistentStoreDescriptionGetConfiguration(_ descriptionPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let descriptionPtr else {
        return nil
    }
    let description: NSPersistentStoreDescription = cdBorrow(descriptionPtr)
    return description.configuration.flatMap { cdCString($0) }
}

@_cdecl("cd_persistent_store_description_set_configuration")
public func cdPersistentStoreDescriptionSetConfiguration(
    _ descriptionPtr: UnsafeMutableRawPointer?,
    _ configuration: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let descriptionPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing store description")
        }
        let description: NSPersistentStoreDescription = cdBorrow(descriptionPtr)
        description.configuration = cdOptionalString(configuration)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_persistent_store_description_get_url")
public func cdPersistentStoreDescriptionGetURL(_ descriptionPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let descriptionPtr else {
        return nil
    }
    let description: NSPersistentStoreDescription = cdBorrow(descriptionPtr)
    return description.url.flatMap { cdCString($0.path) }
}

@_cdecl("cd_persistent_store_description_set_url")
public func cdPersistentStoreDescriptionSetURL(
    _ descriptionPtr: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let descriptionPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing store description")
        }
        let description: NSPersistentStoreDescription = cdBorrow(descriptionPtr)
        description.url = cdURL(from: path)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_persistent_store_description_get_read_only")
public func cdPersistentStoreDescriptionGetReadOnly(_ descriptionPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let descriptionPtr else {
        return 0
    }
    let description: NSPersistentStoreDescription = cdBorrow(descriptionPtr)
    return description.isReadOnly ? 1 : 0
}

@_cdecl("cd_persistent_store_description_set_read_only")
public func cdPersistentStoreDescriptionSetReadOnly(
    _ descriptionPtr: UnsafeMutableRawPointer?,
    _ readOnly: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let descriptionPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing store description")
        }
        let description: NSPersistentStoreDescription = cdBorrow(descriptionPtr)
        description.isReadOnly = readOnly != 0
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_persistent_store_description_get_timeout")
public func cdPersistentStoreDescriptionGetTimeout(_ descriptionPtr: UnsafeMutableRawPointer?) -> Double {
    guard let descriptionPtr else {
        return 0
    }
    let description: NSPersistentStoreDescription = cdBorrow(descriptionPtr)
    return description.timeout
}

@_cdecl("cd_persistent_store_description_set_timeout")
public func cdPersistentStoreDescriptionSetTimeout(
    _ descriptionPtr: UnsafeMutableRawPointer?,
    _ timeout: Double,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let descriptionPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing store description")
        }
        let description: NSPersistentStoreDescription = cdBorrow(descriptionPtr)
        description.timeout = timeout
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_persistent_store_description_sqlite_pragmas_json")
public func cdPersistentStoreDescriptionSQLitePragmasJSON(
    _ descriptionPtr: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let descriptionPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing store description")
        }
        let description: NSPersistentStoreDescription = cdBorrow(descriptionPtr)
        let payload = cdStringMapPayload(from: description.sqlitePragmas)
        outJSON?.pointee = cdCString(try cdEncodeJSON(payload))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_persistent_store_description_set_sqlite_pragma")
public func cdPersistentStoreDescriptionSetSQLitePragma(
    _ descriptionPtr: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ value: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let descriptionPtr, let name else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing store description or pragma name")
        }
        let description: NSPersistentStoreDescription = cdBorrow(descriptionPtr)
        description.setValue(cdOptionalString(value) as NSString?, forPragmaNamed: String(cString: name))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_persistent_store_description_get_should_add_asynchronously")
public func cdPersistentStoreDescriptionGetShouldAddAsynchronously(_ descriptionPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let descriptionPtr else {
        return 0
    }
    let description: NSPersistentStoreDescription = cdBorrow(descriptionPtr)
    return description.shouldAddStoreAsynchronously ? 1 : 0
}

@_cdecl("cd_persistent_store_description_set_should_add_asynchronously")
public func cdPersistentStoreDescriptionSetShouldAddAsynchronously(_ descriptionPtr: UnsafeMutableRawPointer?, _ asynchronous: Int32) {
    guard let descriptionPtr else {
        return
    }
    let description: NSPersistentStoreDescription = cdBorrow(descriptionPtr)
    description.shouldAddStoreAsynchronously = asynchronous != 0
}

@_cdecl("cd_persistent_store_description_get_should_migrate_automatically")
public func cdPersistentStoreDescriptionGetShouldMigrateAutomatically(_ descriptionPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let descriptionPtr else {
        return 0
    }
    let description: NSPersistentStoreDescription = cdBorrow(descriptionPtr)
    return description.shouldMigrateStoreAutomatically ? 1 : 0
}

@_cdecl("cd_persistent_store_description_set_should_migrate_automatically")
public func cdPersistentStoreDescriptionSetShouldMigrateAutomatically(_ descriptionPtr: UnsafeMutableRawPointer?, _ automaticallyMigrate: Int32) {
    guard let descriptionPtr else {
        return
    }
    let description: NSPersistentStoreDescription = cdBorrow(descriptionPtr)
    description.shouldMigrateStoreAutomatically = automaticallyMigrate != 0
}

@_cdecl("cd_persistent_store_description_get_should_infer_mapping_model_automatically")
public func cdPersistentStoreDescriptionGetShouldInferMappingModelAutomatically(_ descriptionPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let descriptionPtr else {
        return 0
    }
    let description: NSPersistentStoreDescription = cdBorrow(descriptionPtr)
    return description.shouldInferMappingModelAutomatically ? 1 : 0
}

@_cdecl("cd_persistent_store_description_set_should_infer_mapping_model_automatically")
public func cdPersistentStoreDescriptionSetShouldInferMappingModelAutomatically(_ descriptionPtr: UnsafeMutableRawPointer?, _ automaticallyInfer: Int32) {
    guard let descriptionPtr else {
        return
    }
    let description: NSPersistentStoreDescription = cdBorrow(descriptionPtr)
    description.shouldInferMappingModelAutomatically = automaticallyInfer != 0
}

@_cdecl("cd_persistent_store_description_set_option_json")
public func cdPersistentStoreDescriptionSetOptionJSON(
    _ descriptionPtr: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ valueJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let descriptionPtr, let key else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing store description or option key")
        }
        let description: NSPersistentStoreDescription = cdBorrow(descriptionPtr)
        let payload = try valueJSON.map { try cdDecodeJSON($0, as: CDValuePayload.self) }
        let foundationValue = payload.flatMap(cdFoundationValue)
        description.setOption(foundationValue as? NSObject, forKey: cdPersistentStoreOptionKey(String(cString: key)))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_persistent_container_default_directory")
public func cdPersistentContainerDefaultDirectory() -> UnsafeMutablePointer<CChar>? {
    cdCString(NSPersistentContainer.defaultDirectoryURL().path)
}

@_cdecl("cd_persistent_container_new")
public func cdPersistentContainerNew(
    _ name: UnsafePointer<CChar>?,
    _ modelPtr: UnsafeMutableRawPointer?,
    _ outContainer: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let name, let modelPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing persistent container name or model")
        }
        let model: NSManagedObjectModel = cdBorrow(modelPtr)
        let container = NSPersistentContainer(name: String(cString: name), managedObjectModel: model)
        outContainer?.pointee = cdRetain(container)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_persistent_container_get_name")
public func cdPersistentContainerGetName(_ containerPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let containerPtr else {
        return nil
    }
    let container: NSPersistentContainer = cdBorrow(containerPtr)
    return cdCString(container.name)
}

@_cdecl("cd_persistent_container_managed_object_model")
public func cdPersistentContainerManagedObjectModel(_ containerPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let containerPtr else {
        return nil
    }
    let container: NSPersistentContainer = cdBorrow(containerPtr)
    return cdRetain(container.managedObjectModel)
}

@_cdecl("cd_persistent_container_persistent_store_coordinator")
public func cdPersistentContainerPersistentStoreCoordinator(_ containerPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let containerPtr else {
        return nil
    }
    let container: NSPersistentContainer = cdBorrow(containerPtr)
    return cdRetain(container.persistentStoreCoordinator)
}

@_cdecl("cd_persistent_container_persistent_store_descriptions")
public func cdPersistentContainerPersistentStoreDescriptions(_ containerPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let containerPtr else {
        return nil
    }
    let container: NSPersistentContainer = cdBorrow(containerPtr)
    return cdRetain(container.persistentStoreDescriptions as NSArray)
}

@_cdecl("cd_persistent_container_set_persistent_store_descriptions")
public func cdPersistentContainerSetPersistentStoreDescriptions(
    _ containerPtr: UnsafeMutableRawPointer?,
    _ descriptionPtrs: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let containerPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing persistent container")
        }
        let container: NSPersistentContainer = cdBorrow(containerPtr)
        let descriptions: [NSPersistentStoreDescription] = cdObjects(from: descriptionPtrs, count: count)
        container.persistentStoreDescriptions = descriptions
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_persistent_container_load_persistent_stores")
public func cdPersistentContainerLoadPersistentStores(
    _ containerPtr: UnsafeMutableRawPointer?,
    _ timeoutSeconds: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let containerPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing persistent container")
        }
        let container: NSPersistentContainer = cdBorrow(containerPtr)
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

@_cdecl("cd_persistent_container_view_context")
public func cdPersistentContainerViewContext(_ containerPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let containerPtr else {
        return nil
    }
    let container: NSPersistentContainer = cdBorrow(containerPtr)
    return cdRetain(container.viewContext)
}

@_cdecl("cd_persistent_container_new_background_context")
public func cdPersistentContainerNewBackgroundContext(_ containerPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let containerPtr else {
        return nil
    }
    let container: NSPersistentContainer = cdBorrow(containerPtr)
    return cdRetain(container.newBackgroundContext())
}
