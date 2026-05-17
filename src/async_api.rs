//! Async API for CoreData
//!
//! Wraps Apple completion-handler and expensive-sync CoreData APIs as executor-agnostic
//! Rust [`Future`]s. Requires the **`async`** Cargo feature.
//!
//! ## Available Types
//!
//! | Type | Apple API | Description |
//! |------|-----------|-------------|
//! | [`AsyncPersistentContainer`] | `NSPersistentContainer.loadPersistentStores` | Async store loading |
//! | [`AsyncPersistentCloudKitContainer`] | `NSPersistentCloudKitContainer.initializeCloudKitSchema` | Async CloudKit schema init |
//! | [`AsyncManagedObjectContext`] | `NSManagedObjectContext.perform { save() }` | Async save on context queue |
//! | [`AsyncHistory`] | `NSPersistentHistoryChangeRequest` execute | Async history fetch |
//! | [`AsyncBatchOperation`] | `NSBatchInsertRequest` / `NSBatchUpdateRequest` | Async batch insert/update |
//!
//! ## Note on Tier-2 APIs
//!
//! `NSFetchedResultsController` delegate callbacks and `NSPersistentCloudKitContainer`
//! event notifications are multi-fire observer patterns. Use a **Tier-2 Stream** wrapper
//! for those — they are not Future candidates.
//!
//! ## Runtime Agnostic Design
//!
//! All futures use only `std` types and work with any async executor (Tokio, async-std,
//! smol, pollster, etc.).
//!
//! ## Examples
//!
//! ```rust,no_run
//! use coredata::async_api::AsyncPersistentContainer;
//! use coredata::{NSPersistentContainer, NSManagedObjectModel};
//!
//! async fn example(container: &NSPersistentContainer) -> Result<(), Box<dyn std::error::Error>> {
//!     AsyncPersistentContainer(container).load_persistent_stores().await?;
//!     Ok(())
//! }
//! ```

use doom_fish_utils::completion::{error_from_cstr, AsyncCompletion, AsyncCompletionFuture};
use std::ffi::c_void;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::error::CoreDataError;

/// Convert a raw string error into a bridge `CoreDataError`.
#[inline]
fn bridge_err(msg: String) -> CoreDataError {
    CoreDataError::bridge(-2, msg)
}

// ============================================================================
// LoadStoresFuture — NSPersistentContainer.loadPersistentStores
// ============================================================================

extern "C" fn load_stores_cb(error: *const i8, ctx: *mut c_void) {
    if error.is_null() {
        unsafe { AsyncCompletion::<()>::complete_ok(ctx, ()) };
    } else {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<()>::complete_err(ctx, msg) };
    }
}

/// Future resolving when `NSPersistentContainer.loadPersistentStores` completes.
pub struct LoadStoresFuture {
    inner: AsyncCompletionFuture<()>,
}

impl std::fmt::Debug for LoadStoresFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoadStoresFuture").finish_non_exhaustive()
    }
}

impl Future for LoadStoresFuture {
    type Output = Result<(), CoreDataError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(bridge_err))
    }
}

/// Async adapter for [`crate::store::NSPersistentContainer`].
///
/// ```rust,no_run
/// # async fn example(container: &coredata::NSPersistentContainer) -> Result<(), Box<dyn std::error::Error>> {
/// use coredata::async_api::AsyncPersistentContainer;
/// AsyncPersistentContainer(container).load_persistent_stores().await?;
/// # Ok(())
/// # }
/// ```
pub struct AsyncPersistentContainer<'a>(pub &'a crate::store::NSPersistentContainer);

impl AsyncPersistentContainer<'_> {
    /// Async variant of `NSPersistentContainer.loadPersistentStores(completionHandler:)`.
    pub fn load_persistent_stores(&self) -> LoadStoresFuture {
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            crate::ffi::async_api::cd_persistent_container_load_stores_async(
                self.0.as_ptr(),
                load_stores_cb,
                ctx,
            );
        }
        LoadStoresFuture { inner: future }
    }
}

impl std::fmt::Debug for AsyncPersistentContainer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncPersistentContainer")
            .finish_non_exhaustive()
    }
}

// ============================================================================
// InitializeCloudKitSchemaFuture — NSPersistentCloudKitContainer.initializeCloudKitSchema
// ============================================================================

extern "C" fn init_schema_cb(error: *const i8, ctx: *mut c_void) {
    if error.is_null() {
        unsafe { AsyncCompletion::<()>::complete_ok(ctx, ()) };
    } else {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<()>::complete_err(ctx, msg) };
    }
}

/// Future resolving when `NSPersistentCloudKitContainer.initializeCloudKitSchema` completes.
pub struct InitializeCloudKitSchemaFuture {
    inner: AsyncCompletionFuture<()>,
}

impl std::fmt::Debug for InitializeCloudKitSchemaFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InitializeCloudKitSchemaFuture")
            .finish_non_exhaustive()
    }
}

impl Future for InitializeCloudKitSchemaFuture {
    type Output = Result<(), CoreDataError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(bridge_err))
    }
}

/// Async adapter for [`crate::cloudkit_mirroring::NSPersistentCloudKitContainer`].
///
/// ```rust,no_run
/// # async fn example(container: &coredata::NSPersistentCloudKitContainer) -> Result<(), Box<dyn std::error::Error>> {
/// use coredata::async_api::{AsyncPersistentCloudKitContainer};
/// use coredata::CloudKitSchemaInitializationOptions;
/// AsyncPersistentCloudKitContainer(container)
///     .initialize_cloud_kit_schema(CloudKitSchemaInitializationOptions::NONE)
///     .await?;
/// # Ok(())
/// # }
/// ```
pub struct AsyncPersistentCloudKitContainer<'a>(
    pub &'a crate::cloudkit_mirroring::NSPersistentCloudKitContainer,
);

impl AsyncPersistentCloudKitContainer<'_> {
    /// Async variant of `NSPersistentCloudKitContainer.initializeCloudKitSchema(options:)`.
    pub fn initialize_cloud_kit_schema(
        &self,
        options: crate::cloudkit_mirroring::CloudKitSchemaInitializationOptions,
    ) -> InitializeCloudKitSchemaFuture {
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            crate::ffi::async_api::cd_persistent_cloudkit_container_initialize_schema_async(
                self.0.as_ptr(),
                options.bits(),
                init_schema_cb,
                ctx,
            );
        }
        InitializeCloudKitSchemaFuture { inner: future }
    }
}

impl std::fmt::Debug for AsyncPersistentCloudKitContainer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncPersistentCloudKitContainer")
            .finish_non_exhaustive()
    }
}

// ============================================================================
// ContextPerformSaveFuture — NSManagedObjectContext.perform { save() }
// ============================================================================

extern "C" fn perform_save_cb(error: *const i8, ctx: *mut c_void) {
    if error.is_null() {
        unsafe { AsyncCompletion::<()>::complete_ok(ctx, ()) };
    } else {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<()>::complete_err(ctx, msg) };
    }
}

/// Future resolving when `context.perform { save() }` completes.
pub struct ContextPerformSaveFuture {
    inner: AsyncCompletionFuture<()>,
}

impl std::fmt::Debug for ContextPerformSaveFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContextPerformSaveFuture")
            .finish_non_exhaustive()
    }
}

impl Future for ContextPerformSaveFuture {
    type Output = Result<(), CoreDataError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(bridge_err))
    }
}

/// Async adapter for [`crate::context::NSManagedObjectContext`].
///
/// `NSManagedObjectContext.performAndWait` is synchronous — not a Future candidate.
/// This wrapper exposes the async `perform { save() }` pattern.
///
/// ```rust,no_run
/// # async fn example(ctx: &coredata::NSManagedObjectContext) -> Result<(), Box<dyn std::error::Error>> {
/// use coredata::async_api::AsyncManagedObjectContext;
/// AsyncManagedObjectContext(ctx).perform_save().await?;
/// # Ok(())
/// # }
/// ```
pub struct AsyncManagedObjectContext<'a>(pub &'a crate::context::NSManagedObjectContext);

impl AsyncManagedObjectContext<'_> {
    /// Schedules `context.save()` on the context's serial queue and resolves when
    /// the save completes.
    pub fn perform_save(&self) -> ContextPerformSaveFuture {
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            crate::ffi::async_api::cd_managed_object_context_perform_save_async(
                self.0.as_ptr(),
                perform_save_cb,
                ctx,
            );
        }
        ContextPerformSaveFuture { inner: future }
    }
}

impl std::fmt::Debug for AsyncManagedObjectContext<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncManagedObjectContext")
            .finish_non_exhaustive()
    }
}

// ============================================================================
// FetchHistoryFuture — NSPersistentHistoryChangeRequest execute
// ============================================================================

extern "C" fn fetch_history_cb(
    result: *mut c_void,
    error: *const i8,
    ctx: *mut c_void,
) {
    if !error.is_null() {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<crate::history::NSPersistentHistoryResult>::complete_err(ctx, msg) };
    } else if !result.is_null() {
        let history_result = unsafe {
            crate::history::NSPersistentHistoryResult::from_retained_ptr(result, "history result")
                .expect("history result ptr valid")
        };
        unsafe { AsyncCompletion::complete_ok(ctx, history_result) };
    } else {
        unsafe {
            AsyncCompletion::<crate::history::NSPersistentHistoryResult>::complete_err(
                ctx,
                "Nil result from history request".to_string(),
            );
        };
    }
}

/// Future resolving with a [`crate::history::NSPersistentHistoryResult`] when the
/// history change request completes.
pub struct FetchHistoryFuture {
    inner: AsyncCompletionFuture<crate::history::NSPersistentHistoryResult>,
}

impl std::fmt::Debug for FetchHistoryFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FetchHistoryFuture").finish_non_exhaustive()
    }
}

impl Future for FetchHistoryFuture {
    type Output = Result<crate::history::NSPersistentHistoryResult, CoreDataError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(bridge_err))
    }
}

/// Async adapter for history fetch operations.
///
/// ```rust,no_run
/// # async fn example(
/// #     ctx: &coredata::NSManagedObjectContext,
/// #     req: &coredata::NSPersistentHistoryChangeRequest,
/// # ) -> Result<(), Box<dyn std::error::Error>> {
/// use coredata::async_api::AsyncHistory;
/// let result = AsyncHistory::fetch(ctx, req).await?;
/// # Ok(())
/// # }
/// ```
pub struct AsyncHistory;

impl AsyncHistory {
    /// Executes `request` asynchronously via `context.perform`, returning the
    /// [`crate::history::NSPersistentHistoryResult`].
    pub fn fetch(
        context: &crate::context::NSManagedObjectContext,
        request: &crate::history::NSPersistentHistoryChangeRequest,
    ) -> FetchHistoryFuture {
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            crate::ffi::async_api::cd_fetch_history_async(
                context.as_ptr(),
                request.as_ptr(),
                fetch_history_cb,
                ctx,
            );
        }
        FetchHistoryFuture { inner: future }
    }
}

// ============================================================================
// BatchInsertFuture — NSBatchInsertRequest async
// ============================================================================

extern "C" fn batch_insert_cb(
    result: *mut c_void,
    error: *const i8,
    ctx: *mut c_void,
) {
    if !error.is_null() {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<crate::batch_operation::NSBatchInsertResult>::complete_err(ctx, msg) };
    } else if !result.is_null() {
        let insert_result = unsafe {
            crate::batch_operation::NSBatchInsertResult::from_retained_ptr(result, "batch insert result")
                .expect("batch insert result ptr valid")
        };
        unsafe { AsyncCompletion::complete_ok(ctx, insert_result) };
    } else {
        unsafe {
            AsyncCompletion::<crate::batch_operation::NSBatchInsertResult>::complete_err(
                ctx,
                "Nil result from batch insert".to_string(),
            );
        };
    }
}

/// Future resolving with a [`crate::batch_operation::NSBatchInsertResult`] when the
/// batch insert completes on the context queue.
pub struct BatchInsertFuture {
    inner: AsyncCompletionFuture<crate::batch_operation::NSBatchInsertResult>,
}

impl std::fmt::Debug for BatchInsertFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BatchInsertFuture").finish_non_exhaustive()
    }
}

impl Future for BatchInsertFuture {
    type Output = Result<crate::batch_operation::NSBatchInsertResult, CoreDataError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(bridge_err))
    }
}

// ============================================================================
// BatchUpdateFuture — NSBatchUpdateRequest async
// ============================================================================

extern "C" fn batch_update_cb(
    result: *mut c_void,
    error: *const i8,
    ctx: *mut c_void,
) {
    if !error.is_null() {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<crate::batch_operation::NSBatchUpdateResult>::complete_err(ctx, msg) };
    } else if !result.is_null() {
        let update_result = unsafe {
            crate::batch_operation::NSBatchUpdateResult::from_retained_ptr(result, "batch update result")
                .expect("batch update result ptr valid")
        };
        unsafe { AsyncCompletion::complete_ok(ctx, update_result) };
    } else {
        unsafe {
            AsyncCompletion::<crate::batch_operation::NSBatchUpdateResult>::complete_err(
                ctx,
                "Nil result from batch update".to_string(),
            );
        };
    }
}

/// Future resolving with a [`crate::batch_operation::NSBatchUpdateResult`] when the
/// batch update completes on the context queue.
pub struct BatchUpdateFuture {
    inner: AsyncCompletionFuture<crate::batch_operation::NSBatchUpdateResult>,
}

impl std::fmt::Debug for BatchUpdateFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BatchUpdateFuture").finish_non_exhaustive()
    }
}

impl Future for BatchUpdateFuture {
    type Output = Result<crate::batch_operation::NSBatchUpdateResult, CoreDataError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(bridge_err))
    }
}

/// Async adapter for batch insert and batch update operations.
///
/// ```rust,no_run
/// # async fn example(
/// #     ctx: &coredata::NSManagedObjectContext,
/// #     req: &coredata::NSBatchInsertRequest,
/// # ) -> Result<(), Box<dyn std::error::Error>> {
/// use coredata::async_api::AsyncBatchOperation;
/// let result = AsyncBatchOperation::insert(ctx, req).await?;
/// # Ok(())
/// # }
/// ```
pub struct AsyncBatchOperation;

impl AsyncBatchOperation {
    /// Executes a [`crate::batch_operation::NSBatchInsertRequest`] asynchronously via
    /// `context.perform`, returning the [`crate::batch_operation::NSBatchInsertResult`].
    pub fn insert(
        context: &crate::context::NSManagedObjectContext,
        request: &crate::batch_operation::NSBatchInsertRequest,
    ) -> BatchInsertFuture {
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            crate::ffi::async_api::cd_batch_insert_async(
                context.as_ptr(),
                request.as_ptr(),
                batch_insert_cb,
                ctx,
            );
        }
        BatchInsertFuture { inner: future }
    }

    /// Executes a [`crate::batch_operation::NSBatchUpdateRequest`] asynchronously via
    /// `context.perform`, returning the [`crate::batch_operation::NSBatchUpdateResult`].
    pub fn update(
        context: &crate::context::NSManagedObjectContext,
        request: &crate::batch_operation::NSBatchUpdateRequest,
    ) -> BatchUpdateFuture {
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            crate::ffi::async_api::cd_batch_update_async(
                context.as_ptr(),
                request.as_ptr(),
                batch_update_cb,
                ctx,
            );
        }
        BatchUpdateFuture { inner: future }
    }
}
