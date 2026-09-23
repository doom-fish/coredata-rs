// Async.swift — callback-based async thunks for CoreData APIs
//
// Each function takes a C callback of the form:
//   (error_cstr_or_null, ctx)            — void-result operations
//   (result_ptr_or_null, error_cstr, ctx) — operations that return a pointer
//
// The callback is fired exactly once per call, on whatever queue the underlying
// operation naturally completes on.

import CoreData
import CoreDataObjCBridge
import Foundation

public typealias CDAsyncVoidCallback = @convention(c) (UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void
public typealias CDAsyncResultCallback = @convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void

func cdAsyncErrorJSON(_ error: Error) -> String {
    let nsError = error as NSError
    let reported = nsError.domain == "Foundation._GenericObjCError" ? cdSilentFailureError() : nsError
    let payload = CDErrorPayload(domain: reported.domain, code: reported.code, message: reported.localizedDescription)
    return (try? cdEncodeJSON(payload)) ?? reported.localizedDescription
}

func cdFail(_ cb: CDAsyncVoidCallback, _ ctx: UnsafeMutableRawPointer, _ error: Error) {
    cdAsyncErrorJSON(error).withCString { cb($0, ctx) }
}

func cdFail(_ cb: CDAsyncResultCallback, _ ctx: UnsafeMutableRawPointer, _ error: Error) {
    cdAsyncErrorJSON(error).withCString { cb(nil, $0, ctx) }
}

final class CDAsyncLoadState {
    private let lock = NSLock()
    private let expected: Int
    private var completed = 0
    private var firstError: Error?
    private var fired = false

    init(expected: Int) {
        self.expected = expected
    }

    func record(_ error: Error?) -> (fire: Bool, error: Error?) {
        lock.lock()
        defer { lock.unlock() }
        if firstError == nil {
            firstError = error
        }
        completed += 1
        guard completed >= expected, !fired else {
            return (false, nil)
        }
        fired = true
        return (true, firstError)
    }

    func abort() -> Bool {
        lock.lock()
        defer { lock.unlock() }
        guard !fired else {
            return false
        }
        fired = true
        return true
    }
}

func cdEnsureRegisteredStoreTypes(_ descriptions: [NSPersistentStoreDescription]) throws {
    let registered = NSPersistentStoreCoordinator.registeredStoreTypes
    for description in descriptions where registered[description.type] == nil {
        throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Unsupported persistent store type '\(description.type)'")
    }
}

// ============================================================================
// NSPersistentContainer.loadPersistentStores (completion-handler based)
// ============================================================================

/// Async thunk for `NSPersistentContainer.loadPersistentStores(completionHandler:)`.
/// Fires `cb(nil, ctx)` on success or `cb(errCStr, ctx)` on the first error.
@_cdecl("cd_persistent_container_load_stores_async")
public func cdPersistentContainerLoadStoresAsync(
    _ containerPtr: UnsafeMutableRawPointer?,
    _ cb: CDAsyncVoidCallback,
    _ ctx: UnsafeMutableRawPointer
) {
    do {
        let container: NSPersistentContainer = try cdArg(containerPtr, "persistent container")
        let descriptions = container.persistentStoreDescriptions
        guard !descriptions.isEmpty else {
            cb(nil, ctx)
            return
        }
        try cdEnsureRegisteredStoreTypes(descriptions)
        let state = CDAsyncLoadState(expected: descriptions.count)
        var exceptionError: NSError?
        let started = CDXLoadPersistentStores(container, { _, error in
            let outcome = state.record(error)
            guard outcome.fire else {
                return
            }
            if let error = outcome.error {
                cdFail(cb, ctx, error)
            } else {
                cb(nil, ctx)
            }
        }, &exceptionError)
        if !started, state.abort() {
            cdFail(cb, ctx, exceptionError ?? cdSilentFailureError())
        }
    } catch {
        cdFail(cb, ctx, error)
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
    _ cb: CDAsyncVoidCallback,
    _ ctx: UnsafeMutableRawPointer
) {
    let container: NSPersistentCloudKitContainer
    do {
        container = try cdArg(containerPtr, "CloudKit container")
    } catch {
        cdFail(cb, ctx, error)
        return
    }
    let options = NSPersistentCloudKitContainerSchemaInitializationOptions(rawValue: UInt(optionsRawValue))
    DispatchQueue.global(qos: .userInitiated).async {
        do {
            try container.initializeCloudKitSchema(options: options)
            cb(nil, ctx)
        } catch {
            cdFail(cb, ctx, error)
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
    _ cb: CDAsyncVoidCallback,
    _ ctx: UnsafeMutableRawPointer
) {
    let context: NSManagedObjectContext
    do {
        context = try cdArg(contextPtr, "managed object context")
        try cdEnsureQueueContext(context)
    } catch {
        cdFail(cb, ctx, error)
        return
    }
    context.perform {
        do {
            if context.hasChanges {
                try cdEnsureSavable(context)
                try cdObjCCall { CDXSave(context, $0) }
            }
            cb(nil, ctx)
        } catch {
            cdFail(cb, ctx, error)
        }
    }
}

func cdPerformRequestAsync<T: NSPersistentStoreResult>(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ requestPtr: UnsafeMutableRawPointer?,
    requestType: NSPersistentStoreRequest.Type,
    resultType _: T.Type,
    requireSQLite what: String?,
    _ cb: CDAsyncResultCallback,
    _ ctx: UnsafeMutableRawPointer
) {
    let context: NSManagedObjectContext
    let snapshot: NSPersistentStoreRequest
    do {
        context = try cdArg(contextPtr, "managed object context")
        try cdEnsureQueueContext(context)
        let request: NSPersistentStoreRequest = try cdArg(requestPtr, "persistent store request")
        guard request.isKind(of: requestType) else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Expected \(requestType), got \(type(of: request))")
        }
        snapshot = try cdSnapshotRequest(request)
    } catch {
        cdFail(cb, ctx, error)
        return
    }
    context.perform {
        do {
            if let what {
                try cdEnsureSQLiteStores(context, what)
            }
            let result = try cdObjCValue { CDXExecuteRequest(context, snapshot, $0) }
            guard let typed = result as? T else {
                throw cdBridgeNSError(code: CDR_FAILURE, message: "Unexpected result type \(type(of: result))")
            }
            cb(cdRetain(typed), nil, ctx)
        } catch {
            cdFail(cb, ctx, error)
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
    _ cb: CDAsyncResultCallback,
    _ ctx: UnsafeMutableRawPointer
) {
    cdPerformRequestAsync(
        contextPtr,
        requestPtr,
        requestType: NSPersistentHistoryChangeRequest.self,
        resultType: NSPersistentHistoryResult.self,
        requireSQLite: nil,
        cb,
        ctx
    )
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
    _ cb: CDAsyncResultCallback,
    _ ctx: UnsafeMutableRawPointer
) {
    cdPerformRequestAsync(
        contextPtr,
        requestPtr,
        requestType: NSBatchInsertRequest.self,
        resultType: NSBatchInsertResult.self,
        requireSQLite: "Batch insert",
        cb,
        ctx
    )
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
    _ cb: CDAsyncResultCallback,
    _ ctx: UnsafeMutableRawPointer
) {
    cdPerformRequestAsync(
        contextPtr,
        requestPtr,
        requestType: NSBatchUpdateRequest.self,
        resultType: NSBatchUpdateResult.self,
        requireSQLite: "Batch update",
        cb,
        ctx
    )
}
