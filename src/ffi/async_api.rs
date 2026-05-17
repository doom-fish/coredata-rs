use core::ffi::{c_char, c_void};

extern "C" {
    /// Async `NSPersistentContainer.loadPersistentStores`.
    /// `cb(null, ctx)` on success; `cb(err_cstr, ctx)` on failure.
    pub fn cd_persistent_container_load_stores_async(
        container: *mut c_void,
        cb: extern "C" fn(*const c_char, *mut c_void),
        ctx: *mut c_void,
    );

    /// Async `NSPersistentCloudKitContainer.initializeCloudKitSchema(options:)`.
    /// `cb(null, ctx)` on success; `cb(err_cstr, ctx)` on failure.
    pub fn cd_persistent_cloudkit_container_initialize_schema_async(
        container: *mut c_void,
        options_raw: u64,
        cb: extern "C" fn(*const c_char, *mut c_void),
        ctx: *mut c_void,
    );

    /// Async `NSManagedObjectContext.perform { save() }`.
    /// `cb(null, ctx)` on success; `cb(err_cstr, ctx)` on failure.
    pub fn cd_managed_object_context_perform_save_async(
        context: *mut c_void,
        cb: extern "C" fn(*const c_char, *mut c_void),
        ctx: *mut c_void,
    );

    /// Async `NSPersistentHistoryChangeRequest` execute via `context.perform`.
    /// `cb(result_ptr, null, ctx)` on success; `cb(null, err_cstr, ctx)` on failure.
    pub fn cd_fetch_history_async(
        context: *mut c_void,
        request: *mut c_void,
        cb: extern "C" fn(*mut c_void, *const c_char, *mut c_void),
        ctx: *mut c_void,
    );

    /// Async `NSBatchInsertRequest` execute via `context.perform`.
    /// `cb(result_ptr, null, ctx)` on success; `cb(null, err_cstr, ctx)` on failure.
    pub fn cd_batch_insert_async(
        context: *mut c_void,
        request: *mut c_void,
        cb: extern "C" fn(*mut c_void, *const c_char, *mut c_void),
        ctx: *mut c_void,
    );

    /// Async `NSBatchUpdateRequest` execute via `context.perform`.
    /// `cb(result_ptr, null, ctx)` on success; `cb(null, err_cstr, ctx)` on failure.
    pub fn cd_batch_update_async(
        context: *mut c_void,
        request: *mut c_void,
        cb: extern "C" fn(*mut c_void, *const c_char, *mut c_void),
        ctx: *mut c_void,
    );
}
