import CoreData
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

@_cdecl("cd_persistent_store_coordinator_new")
public func cdPersistentStoreCoordinatorNew(
    _ modelPtr: UnsafeMutableRawPointer?,
    _ outCoordinator: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let modelPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object model")
        }
        let model: NSManagedObjectModel = cdBorrow(modelPtr)
        outCoordinator?.pointee = cdRetain(NSPersistentStoreCoordinator(managedObjectModel: model))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_persistent_store_coordinator_get_name")
public func cdPersistentStoreCoordinatorGetName(_ coordinatorPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let coordinatorPtr else {
        return nil
    }
    let coordinator: NSPersistentStoreCoordinator = cdBorrow(coordinatorPtr)
    return coordinator.name.flatMap { cdCString($0) }
}

@_cdecl("cd_persistent_store_coordinator_set_name")
public func cdPersistentStoreCoordinatorSetName(
    _ coordinatorPtr: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let coordinatorPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing persistent store coordinator")
        }
        let coordinator: NSPersistentStoreCoordinator = cdBorrow(coordinatorPtr)
        coordinator.name = cdOptionalString(name)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_persistent_store_coordinator_managed_object_model")
public func cdPersistentStoreCoordinatorManagedObjectModel(_ coordinatorPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let coordinatorPtr else {
        return nil
    }
    let coordinator: NSPersistentStoreCoordinator = cdBorrow(coordinatorPtr)
    return cdRetain(coordinator.managedObjectModel)
}

@_cdecl("cd_persistent_store_coordinator_persistent_stores")
public func cdPersistentStoreCoordinatorPersistentStores(_ coordinatorPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let coordinatorPtr else {
        return nil
    }
    let coordinator: NSPersistentStoreCoordinator = cdBorrow(coordinatorPtr)
    return cdRetain(coordinator.persistentStores as NSArray)
}

@_cdecl("cd_persistent_store_coordinator_add_persistent_store")
public func cdPersistentStoreCoordinatorAddPersistentStore(
    _ coordinatorPtr: UnsafeMutableRawPointer?,
    _ storeType: UnsafePointer<CChar>?,
    _ configuration: UnsafePointer<CChar>?,
    _ urlPath: UnsafePointer<CChar>?,
    _ optionsJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let coordinatorPtr, let storeType else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing persistent store coordinator or store type")
        }
        let coordinator: NSPersistentStoreCoordinator = cdBorrow(coordinatorPtr)
        _ = try coordinator.addPersistentStore(
            ofType: String(cString: storeType),
            configurationName: configuration.map(String.init(cString:)),
            at: cdURL(from: urlPath),
            options: try cdPersistentStoreOptions(from: optionsJSON)
        )
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_persistent_store_coordinator_add_persistent_store_with_description")
public func cdPersistentStoreCoordinatorAddPersistentStoreWithDescription(
    _ coordinatorPtr: UnsafeMutableRawPointer?,
    _ descriptionPtr: UnsafeMutableRawPointer?,
    _ timeoutSeconds: Int32,
    _ outStore: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let coordinatorPtr, let descriptionPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing persistent store coordinator or store description")
        }
        let coordinator: NSPersistentStoreCoordinator = cdBorrow(coordinatorPtr)
        let description: NSPersistentStoreDescription = cdBorrow(descriptionPtr)
        let tracker = CDStoreAddTracker()
        coordinator.addPersistentStore(with: description) { addedDescription, error in
            let store = addedDescription.url.flatMap { coordinator.persistentStore(for: $0) } ?? coordinator.persistentStores.last
            tracker.record(store: store, error: error as NSError?)
        }
        let (store, error) = tracker.wait(timeoutSeconds: timeoutSeconds)
        if let error {
            throw error
        }
        outStore?.pointee = store.map(cdRetain)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_persistent_store_coordinator_remove_persistent_store")
public func cdPersistentStoreCoordinatorRemovePersistentStore(
    _ coordinatorPtr: UnsafeMutableRawPointer?,
    _ storePtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let coordinatorPtr, let storePtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing persistent store coordinator or store")
        }
        let coordinator: NSPersistentStoreCoordinator = cdBorrow(coordinatorPtr)
        let store: NSPersistentStore = cdBorrow(storePtr)
        try coordinator.remove(store)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_persistent_store_coordinator_persistent_store_for_url")
public func cdPersistentStoreCoordinatorPersistentStoreForURL(
    _ coordinatorPtr: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let coordinatorPtr, let url = cdURL(from: path) else {
        return nil
    }
    let coordinator: NSPersistentStoreCoordinator = cdBorrow(coordinatorPtr)
    return coordinator.persistentStore(for: url).map(cdRetain)
}

@_cdecl("cd_persistent_store_coordinator_url_for_persistent_store")
public func cdPersistentStoreCoordinatorURLForPersistentStore(
    _ coordinatorPtr: UnsafeMutableRawPointer?,
    _ storePtr: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    guard let coordinatorPtr, let storePtr else {
        return nil
    }
    let coordinator: NSPersistentStoreCoordinator = cdBorrow(coordinatorPtr)
    let store: NSPersistentStore = cdBorrow(storePtr)
    return cdCString(coordinator.url(for: store).path)
}

@_cdecl("cd_persistent_store_coordinator_current_persistent_history_token")
public func cdPersistentStoreCoordinatorCurrentPersistentHistoryToken(_ coordinatorPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let coordinatorPtr else {
        return nil
    }
    let coordinator: NSPersistentStoreCoordinator = cdBorrow(coordinatorPtr)
    return coordinator.currentPersistentHistoryToken(fromStores: nil).map(cdRetain)
}

@_cdecl("cd_persistent_store_coordinator_destroy_persistent_store")
public func cdPersistentStoreCoordinatorDestroyPersistentStore(
    _ coordinatorPtr: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ storeType: UnsafePointer<CChar>?,
    _ optionsJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let coordinatorPtr, let path, let storeType else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing persistent store coordinator, path, or store type")
        }
        let coordinator: NSPersistentStoreCoordinator = cdBorrow(coordinatorPtr)
        try coordinator.destroyPersistentStore(
            at: URL(fileURLWithPath: String(cString: path)),
            type: NSPersistentStore.StoreType(rawValue: String(cString: storeType)),
            options: try cdPersistentStoreOptions(from: optionsJSON)
        )
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_persistent_store_get_configuration_name")
public func cdPersistentStoreGetConfigurationName(_ storePtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let storePtr else {
        return nil
    }
    let store: NSPersistentStore = cdBorrow(storePtr)
    return cdCString(store.configurationName)
}

@_cdecl("cd_persistent_store_get_url")
public func cdPersistentStoreGetURL(_ storePtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let storePtr else {
        return nil
    }
    let store: NSPersistentStore = cdBorrow(storePtr)
    return store.url.flatMap { cdCString($0.path) }
}

@_cdecl("cd_persistent_store_set_url")
public func cdPersistentStoreSetURL(
    _ storePtr: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let storePtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing persistent store")
        }
        let store: NSPersistentStore = cdBorrow(storePtr)
        store.url = cdURL(from: path)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_persistent_store_get_identifier")
public func cdPersistentStoreGetIdentifier(_ storePtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let storePtr else {
        return nil
    }
    let store: NSPersistentStore = cdBorrow(storePtr)
    return cdCString(store.identifier)
}

@_cdecl("cd_persistent_store_set_identifier")
public func cdPersistentStoreSetIdentifier(
    _ storePtr: UnsafeMutableRawPointer?,
    _ identifier: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let storePtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing persistent store")
        }
        let store: NSPersistentStore = cdBorrow(storePtr)
        store.identifier = cdOptionalString(identifier)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_persistent_store_get_type")
public func cdPersistentStoreGetType(_ storePtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let storePtr else {
        return nil
    }
    let store: NSPersistentStore = cdBorrow(storePtr)
    return cdCString(store.type)
}

@_cdecl("cd_persistent_store_get_read_only")
public func cdPersistentStoreGetReadOnly(_ storePtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let storePtr else {
        return 0
    }
    let store: NSPersistentStore = cdBorrow(storePtr)
    return store.isReadOnly ? 1 : 0
}

@_cdecl("cd_persistent_store_set_read_only")
public func cdPersistentStoreSetReadOnly(
    _ storePtr: UnsafeMutableRawPointer?,
    _ readOnly: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let storePtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing persistent store")
        }
        let store: NSPersistentStore = cdBorrow(storePtr)
        store.isReadOnly = readOnly != 0
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}
