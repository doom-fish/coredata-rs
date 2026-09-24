import CoreData
import CoreDataObjCBridge
import Foundation

final class CDStoreAddTracker {
    private let semaphore = DispatchSemaphore(value: 0)
    private let lock = NSLock()
    private var store: NSPersistentStore?
    private var error: NSError?

    func record(store: NSPersistentStore?, error: NSError?) {
        lock.lock()
        defer { lock.unlock() }
        if self.store == nil {
            self.store = store
        }
        if self.error == nil {
            self.error = error
        }
        semaphore.signal()
    }

    func wait(timeoutSeconds: Int32) -> (NSPersistentStore?, NSError?) {
        if semaphore.wait(timeout: .now() + .seconds(Int(timeoutSeconds))) == .timedOut {
            return (nil, cdBridgeNSError(code: CDR_TIMED_OUT, message: "Timed out waiting for addPersistentStore(with:completionHandler:)") )
        }
        return (store, error)
    }
}

func cdRegisteredStoreType(_ storeType: UnsafePointer<CChar>?) throws -> String {
    guard let storeType else {
        throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing store type")
    }
    let type = String(cString: storeType)
    guard NSPersistentStoreCoordinator.registeredStoreTypes[type] != nil else {
        throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Unsupported persistent store type '\(type)'")
    }
    return type
}

let cdStoreLock = NSRecursiveLock()

func cdOnCoordinatorQueue<T>(_ coordinator: NSPersistentStoreCoordinator, _ body: () throws -> T) throws -> T {
    var outcome: Result<T, Error>?
    coordinator.performAndWait {
        outcome = Result { try body() }
    }
    guard let outcome else {
        throw cdBridgeNSError(code: CDR_FAILURE, message: "The persistent store coordinator did not run the block")
    }
    return try outcome.get()
}

func cdWithCoordinator<T>(
    _ coordinatorPtr: UnsafeMutableRawPointer?,
    _ body: (NSPersistentStoreCoordinator) throws -> T
) throws -> T {
    let coordinator: NSPersistentStoreCoordinator = try cdArg(coordinatorPtr, "persistent store coordinator")
    return try cdOnCoordinatorQueue(coordinator) { try body(coordinator) }
}

func cdLockedStore<T>(_ store: NSPersistentStore, _ body: (NSPersistentStore) throws -> T) rethrows -> T {
    cdStoreLock.lock()
    defer { cdStoreLock.unlock() }
    return try body(store)
}

func cdWithStore<T>(_ storePtr: UnsafeMutableRawPointer?, _ body: (NSPersistentStore) throws -> T) throws -> T {
    let store: NSPersistentStore = try cdArg(storePtr, "persistent store")
    if let coordinator = store.persistentStoreCoordinator {
        return try cdOnCoordinatorQueue(coordinator) { try cdLockedStore(store, body) }
    }
    return try cdLockedStore(store, body)
}

@_cdecl("cd_persistent_store_coordinator_new")
public func cdPersistentStoreCoordinatorNew(
    _ modelPtr: UnsafeMutableRawPointer?,
    _ outCoordinator: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    cdStatus(outError) {
        let model: NSManagedObjectModel = try cdArg(modelPtr, "managed object model")
        outCoordinator?.pointee = cdRetain(NSPersistentStoreCoordinator(managedObjectModel: try cdHandoffModel(model)))
    }
}

@_cdecl("cd_persistent_store_coordinator_get_name")
public func cdPersistentStoreCoordinatorGetName(_ coordinatorPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    let name = try? cdWithCoordinator(coordinatorPtr) { $0.name }
    return name.flatMap { cdCString($0) }
}

@_cdecl("cd_persistent_store_coordinator_set_name")
public func cdPersistentStoreCoordinatorSetName(
    _ coordinatorPtr: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    cdStatus(outError) {
        let value = cdOptionalString(name)
        try cdWithCoordinator(coordinatorPtr) { $0.name = value }
    }
}

@_cdecl("cd_persistent_store_coordinator_managed_object_model")
public func cdPersistentStoreCoordinatorManagedObjectModel(_ coordinatorPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let coordinator = try? cdArg(coordinatorPtr, "persistent store coordinator") as NSPersistentStoreCoordinator else {
        return nil
    }
    return cdRetain(coordinator.managedObjectModel)
}

@_cdecl("cd_persistent_store_coordinator_persistent_stores")
public func cdPersistentStoreCoordinatorPersistentStores(_ coordinatorPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let stores = try? cdWithCoordinator(coordinatorPtr, { $0.persistentStores }) else {
        return nil
    }
    return cdRetain(stores as NSArray)
}

@_cdecl("cd_persistent_store_coordinator_add_persistent_store")
public func cdPersistentStoreCoordinatorAddPersistentStore(
    _ coordinatorPtr: UnsafeMutableRawPointer?,
    _ storeType: UnsafePointer<CChar>?,
    _ configuration: UnsafePointer<CChar>?,
    _ urlPath: UnsafePointer<CChar>?,
    _ optionsJSON: UnsafePointer<CChar>?,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let type = try cdRegisteredStoreType(storeType)
        let configurationName = configuration.map(String.init(cString:))
        let url = cdURL(from: urlPath)
        let options = try cdPersistentStoreOptions(from: optionsJSON)
        _ = try cdWithCoordinator(coordinatorPtr) { coordinator in
            try cdObjCValue { CDXAddPersistentStore(coordinator, type, configurationName, url, options, $0) }
        }
    }
}

@_cdecl("cd_persistent_store_coordinator_add_persistent_store_with_description")
public func cdPersistentStoreCoordinatorAddPersistentStoreWithDescription(
    _ coordinatorPtr: UnsafeMutableRawPointer?,
    _ descriptionPtr: UnsafeMutableRawPointer?,
    _ timeoutSeconds: Int32,
    _ outStore: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let coordinator: NSPersistentStoreCoordinator = try cdArg(coordinatorPtr, "persistent store coordinator")
        let source: NSPersistentStoreDescription = try cdArg(descriptionPtr, "persistent store description")
        let description = try cdCopyStoreDescription(source)
        try cdEnsureRegisteredStoreTypes([description])
        let tracker = CDStoreAddTracker()
        try cdOnCoordinatorQueue(coordinator) {
            try cdObjCCall {
                CDXAddPersistentStoreWithDescription(coordinator, description, { addedDescription, error in
                    let store = try? cdOnCoordinatorQueue(coordinator) {
                        addedDescription.url.flatMap { coordinator.persistentStore(for: $0) } ?? coordinator.persistentStores.last
                    }
                    tracker.record(store: store.flatMap { $0 }, error: error as NSError?)
                }, $0)
            }
        }
        let (store, error) = tracker.wait(timeoutSeconds: timeoutSeconds)
        if let error {
            throw error
        }
        outStore?.pointee = store.map(cdRetain)
    }
}

@_cdecl("cd_persistent_store_coordinator_remove_persistent_store")
public func cdPersistentStoreCoordinatorRemovePersistentStore(
    _ coordinatorPtr: UnsafeMutableRawPointer?,
    _ storePtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    cdStatus(outError) {
        let store: NSPersistentStore = try cdArg(storePtr, "persistent store")
        try cdWithCoordinator(coordinatorPtr) { coordinator in
            guard coordinator.persistentStores.contains(where: { $0 === store }) else {
                throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "The persistent store does not belong to this coordinator")
            }
            try coordinator.remove(store)
        }
    }
}

@_cdecl("cd_persistent_store_coordinator_persistent_store_for_url")
public func cdPersistentStoreCoordinatorPersistentStoreForURL(
    _ coordinatorPtr: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let url = cdURL(from: path),
          let store = try? cdWithCoordinator(coordinatorPtr, { $0.persistentStore(for: url) })
    else {
        return nil
    }
    return cdRetain(store)
}

@_cdecl("cd_persistent_store_coordinator_url_for_persistent_store")
public func cdPersistentStoreCoordinatorURLForPersistentStore(
    _ coordinatorPtr: UnsafeMutableRawPointer?,
    _ storePtr: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    guard let store = try? cdArg(storePtr, "persistent store") as NSPersistentStore,
          let path = try? cdWithCoordinator(coordinatorPtr, { coordinator -> String? in
              guard coordinator.persistentStores.contains(where: { $0 === store }) else {
                  return nil
              }
              return coordinator.url(for: store).path
          })
    else {
        return nil
    }
    return cdCString(path)
}

@_cdecl("cd_persistent_store_coordinator_current_persistent_history_token")
public func cdPersistentStoreCoordinatorCurrentPersistentHistoryToken(_ coordinatorPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let token = try? cdWithCoordinator(coordinatorPtr, { $0.currentPersistentHistoryToken(fromStores: nil) }) else {
        return nil
    }
    return cdRetain(token)
}

@_cdecl("cd_persistent_store_coordinator_destroy_persistent_store")
public func cdPersistentStoreCoordinatorDestroyPersistentStore(
    _ coordinatorPtr: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ storeType: UnsafePointer<CChar>?,
    _ optionsJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    cdStatus(outError) {
        guard let path else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing persistent store path")
        }
        let type = try cdRegisteredStoreType(storeType)
        let options = try cdPersistentStoreOptions(from: optionsJSON)
        let url = URL(fileURLWithPath: String(cString: path))
        try cdWithCoordinator(coordinatorPtr) { coordinator in
            try coordinator.destroyPersistentStore(
                at: url,
                type: NSPersistentStore.StoreType(rawValue: type),
                options: options
            )
        }
    }
}

@_cdecl("cd_persistent_store_get_configuration_name")
public func cdPersistentStoreGetConfigurationName(_ storePtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let name = try? cdWithStore(storePtr, { $0.configurationName }) else {
        return nil
    }
    return cdCString(name)
}

@_cdecl("cd_persistent_store_get_url")
public func cdPersistentStoreGetURL(_ storePtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let url = try? cdWithStore(storePtr, { $0.url }) else {
        return nil
    }
    return cdCString(url.path)
}

@_cdecl("cd_persistent_store_set_url")
public func cdPersistentStoreSetURL(
    _ storePtr: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    cdStatus(outError) {
        let url = cdURL(from: path)
        try cdWithStore(storePtr) { $0.url = url }
    }
}

@_cdecl("cd_persistent_store_get_identifier")
public func cdPersistentStoreGetIdentifier(_ storePtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let identifier = try? cdWithStore(storePtr, { $0.identifier }) else {
        return nil
    }
    return cdCString(identifier)
}

@_cdecl("cd_persistent_store_set_identifier")
public func cdPersistentStoreSetIdentifier(
    _ storePtr: UnsafeMutableRawPointer?,
    _ identifier: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    cdStatus(outError) {
        let value = cdOptionalString(identifier)
        try cdWithStore(storePtr) { $0.identifier = value }
    }
}

@_cdecl("cd_persistent_store_get_type")
public func cdPersistentStoreGetType(_ storePtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let type = try? cdWithStore(storePtr, { $0.type }) else {
        return nil
    }
    return cdCString(type)
}

@_cdecl("cd_persistent_store_get_read_only")
public func cdPersistentStoreGetReadOnly(_ storePtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let readOnly = try? cdWithStore(storePtr, { $0.isReadOnly }) else {
        return 0
    }
    return readOnly ? 1 : 0
}

@_cdecl("cd_persistent_store_set_read_only")
public func cdPersistentStoreSetReadOnly(
    _ storePtr: UnsafeMutableRawPointer?,
    _ readOnly: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    cdStatus(outError) {
        try cdWithStore(storePtr) { $0.isReadOnly = readOnly != 0 }
    }
}
