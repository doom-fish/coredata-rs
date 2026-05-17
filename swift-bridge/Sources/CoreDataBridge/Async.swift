// Async.swift — callback-based async thunks for CoreData APIs
//
// Each function takes a C callback of the form:
//   (error_cstr_or_null, ctx)            — void-result operations
//   (result_ptr_or_null, error_cstr, ctx) — operations that return a pointer
//
// The callback is fired exactly once per call, on whatever queue the underlying
// operation naturally completes on.

import CoreData
import Foundation

// ============================================================================
// NSPersistentContainer.loadPersistentStores (completion-handler based)
// ============================================================================

/// Async thunk for `NSPersistentContainer.loadPersistentStores(completionHandler:)`.
/// Fires `cb(nil, ctx)` on success or `cb(errCStr, ctx)` on the first error.
@_cdecl("cd_persistent_container_load_stores_async")
public func cdPersistentContainerLoadStoresAsync(
    _ containerPtr: UnsafeMutableRawPointer?,
    _ cb: @convention(c) (UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer
) {
    guard let containerPtr else {
        "Missing container".withCString { cb($0, ctx) }
        return
    }
    let container: NSPersistentContainer = cdBorrow(containerPtr)
    let expected = max(container.persistentStoreDescriptions.count, 1)
    let lock = NSLock()
    var firstError: String?
    var completed = 0
    container.loadPersistentStores { _, error in
        lock.lock()
        if firstError == nil, let err = error {
            firstError = err.localizedDescription
        }
        completed += 1
        let done = completed >= expected
        lock.unlock()
        if done {
            if let e = firstError {
                e.withCString { cb($0, ctx) }
            } else {
                cb(nil, ctx)
            }
        }
    }
}

// ============================================================================
// NSPersistentCloudKitContainer.initializeCloudKitSchema — offloaded to background
// ============================================================================

/// Async thunk for `NSPersistentCloudKitContainer.initializeCloudKitSchema(options:)`.
/// Runs on a background queue so the Rust caller is not blocked.
@_cdecl("cd_persistent_cloudkit_container_initialize_schema_async")
public func cdPersistentCloudKitContainerInitializeSchemaAsync(
    _ containerPtr: UnsafeMutableRawPointer?,
    _ optionsRawValue: UInt64,
    _ cb: @convention(c) (UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer
) {
    guard let containerPtr else {
        "Missing CloudKit container".withCString { cb($0, ctx) }
        return
    }
    let container: NSPersistentCloudKitContainer = cdBorrow(containerPtr)
    let options = NSPersistentCloudKitContainerSchemaInitializationOptions(
        rawValue: UInt(optionsRawValue)
    )
    DispatchQueue.global(qos: .userInitiated).async {
        do {
            try container.initializeCloudKitSchema(options: options)
            cb(nil, ctx)
        } catch {
            error.localizedDescription.withCString { cb($0, ctx) }
        }
    }
}

// ============================================================================
// NSManagedObjectContext.perform { save() } — async save via context queue
// ============================================================================

/// Async thunk: schedules `context.perform { save() }` and reports success/error
/// via callback when the save completes.
@_cdecl("cd_managed_object_context_perform_save_async")
public func cdManagedObjectContextPerformSaveAsync(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ cb: @convention(c) (UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer
) {
    guard let contextPtr else {
        "Missing managed object context".withCString { cb($0, ctx) }
        return
    }
    let context: NSManagedObjectContext = cdBorrow(contextPtr)
    context.perform {
        do {
            try context.save()
            cb(nil, ctx)
        } catch {
            error.localizedDescription.withCString { cb($0, ctx) }
        }
    }
}

// ============================================================================
// NSPersistentHistoryChangeRequest execute — offloaded via context.perform
// ============================================================================

/// Async thunk for executing a `NSPersistentHistoryChangeRequest`.
/// Runs through `context.perform` to respect the context's concurrency type.
@_cdecl("cd_fetch_history_async")
public func cdFetchHistoryAsync(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ requestPtr: UnsafeMutableRawPointer?,
    _ cb: @convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer
) {
    guard let contextPtr else {
        "Missing managed object context".withCString { cb(nil, $0, ctx) }
        return
    }
    guard let requestPtr else {
        "Missing history change request".withCString { cb(nil, $0, ctx) }
        return
    }
    let context: NSManagedObjectContext = cdBorrow(contextPtr)
    let request: NSPersistentHistoryChangeRequest = cdBorrow(requestPtr)
    context.perform {
        do {
            guard let result = try context.execute(request) as? NSPersistentHistoryResult else {
                throw cdBridgeNSError(code: CDR_FAILURE, message: "Unexpected result type from history request")
            }
            cb(cdRetain(result), nil, ctx)
        } catch {
            error.localizedDescription.withCString { cb(nil, $0, ctx) }
        }
    }
}

// ============================================================================
// NSBatchInsertRequest — offloaded via context.perform
// ============================================================================

/// Async thunk for `NSBatchInsertRequest`. Runs through `context.perform` so the
/// expensive operation does not block the calling thread.
@_cdecl("cd_batch_insert_async")
public func cdBatchInsertAsync(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ requestPtr: UnsafeMutableRawPointer?,
    _ cb: @convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer
) {
    guard let contextPtr else {
        "Missing managed object context".withCString { cb(nil, $0, ctx) }
        return
    }
    guard let requestPtr else {
        "Missing batch insert request".withCString { cb(nil, $0, ctx) }
        return
    }
    let context: NSManagedObjectContext = cdBorrow(contextPtr)
    let request: NSBatchInsertRequest = cdBorrow(requestPtr)
    context.perform {
        do {
            guard let result = try context.execute(request) as? NSBatchInsertResult else {
                throw cdBridgeNSError(code: CDR_FAILURE, message: "Unexpected result type from batch insert")
            }
            cb(cdRetain(result), nil, ctx)
        } catch {
            error.localizedDescription.withCString { cb(nil, $0, ctx) }
        }
    }
}

// ============================================================================
// NSBatchUpdateRequest — offloaded via context.perform
// ============================================================================

/// Async thunk for `NSBatchUpdateRequest`. Runs through `context.perform` so the
/// expensive operation does not block the calling thread.
@_cdecl("cd_batch_update_async")
public func cdBatchUpdateAsync(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ requestPtr: UnsafeMutableRawPointer?,
    _ cb: @convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer
) {
    guard let contextPtr else {
        "Missing managed object context".withCString { cb(nil, $0, ctx) }
        return
    }
    guard let requestPtr else {
        "Missing batch update request".withCString { cb(nil, $0, ctx) }
        return
    }
    let context: NSManagedObjectContext = cdBorrow(contextPtr)
    let request: NSBatchUpdateRequest = cdBorrow(requestPtr)
    context.perform {
        do {
            guard let result = try context.execute(request) as? NSBatchUpdateResult else {
                throw cdBridgeNSError(code: CDR_FAILURE, message: "Unexpected result type from batch update")
            }
            cb(cdRetain(result), nil, ctx)
        } catch {
            error.localizedDescription.withCString { cb(nil, $0, ctx) }
        }
    }
}
