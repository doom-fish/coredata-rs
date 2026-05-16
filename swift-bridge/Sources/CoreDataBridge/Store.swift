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
        let url = urlPath.map { URL(fileURLWithPath: String(cString: $0)) }
        _ = try coordinator.addPersistentStore(
            ofType: String(cString: storeType),
            configurationName: configuration.map(String.init(cString:)),
            at: url,
            options: try cdDictionary(from: optionsJSON))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
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
