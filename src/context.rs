use core::ffi::c_void;

use crate::error::CoreDataError;
use crate::ffi;
use crate::private::{
    collect_array, cstring_from_str, error_from_status, impl_object_wrapper, json_cstring,
    parse_json_ptr,
};
use crate::query::NSFetchRequest;
use crate::schema::NSEntityDescription;
use crate::store::NSPersistentStoreCoordinator;
use crate::value::{Value, ValuePayload};
use doom_fish_utils::panic_safe::catch_user_panic;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
/// Mirrors the corresponding Core Data `NSManagedObjectContextConcurrencyType` value.
pub enum NSManagedObjectContextConcurrencyType {
    /// Mirrors `NSManagedObjectContextConcurrencyType::PrivateQueue`.
    PrivateQueue,
    /// Mirrors `NSManagedObjectContextConcurrencyType::MainQueue`.
    MainQueue,
    /// Mirrors `NSManagedObjectContextConcurrencyType::Unknown`.
    Unknown(i32),
}

impl NSManagedObjectContextConcurrencyType {
    const fn as_raw(self) -> i32 {
        match self {
            Self::PrivateQueue => 1,
            Self::MainQueue => 2,
            Self::Unknown(raw) => raw,
        }
    }
}

impl_object_wrapper!(NSManagedObjectContext);
impl_object_wrapper!(NSManagedObject);

type AsyncPerformCallback = Box<dyn FnOnce(NSManagedObjectContext) + Send + 'static>;

struct PerformState {
    callback: Option<AsyncPerformCallback>,
    context: NSManagedObjectContext,
}

struct PerformAndWaitState<F, R>
where
    F: FnOnce(NSManagedObjectContext) -> R,
{
    callback: Option<F>,
    context: NSManagedObjectContext,
    result: Option<R>,
}

unsafe extern "C" fn perform_trampoline(refcon: *mut c_void) {
    catch_user_panic("perform_trampoline", || {
        // SAFETY: refcon was produced by Box::into_raw in NSManagedObjectContext::perform()
        // and this trampoline is invoked exactly once, so we can take back ownership.
        let mut state: Box<PerformState> = unsafe { Box::from_raw(refcon.cast()) };
        if let Some(callback) = state.callback.take() {
            callback(state.context.clone());
        }
    });
}

unsafe extern "C" fn perform_and_wait_trampoline<F, R>(refcon: *mut c_void)
where
    F: FnOnce(NSManagedObjectContext) -> R,
{
    catch_user_panic("perform_and_wait_trampoline", || {
        // SAFETY: refcon is addr_of_mut!(state) cast to *mut c_void from perform_and_wait().
        // perform_and_wait() blocks synchronously until this trampoline returns, so the
        // stack frame — and therefore state — is alive for the full duration of this call.
        let state = unsafe { &mut *refcon.cast::<PerformAndWaitState<F, R>>() };
        let callback = state
            .callback
            .take()
            .expect("perform_and_wait callback missing");
        state.result = Some(callback(state.context.clone()));
    });
    // If the callback panicked, catch_user_panic swallowed it; state.result will be None,
    // and perform_and_wait() will panic with "callback did not return a value" on the
    // Rust side rather than allowing UB from unwinding across the FFI boundary.
}

impl NSManagedObjectContext {
    /// Wraps `NSManagedObjectContext.init(...)`.
    pub fn new(
        concurrency_type: NSManagedObjectContextConcurrencyType,
    ) -> Result<Self, CoreDataError> {
        let mut out_context = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_context_new(
                concurrency_type.as_raw(),
                &mut out_context,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_context, "managed object context") }
    }

    /// Wraps `NSManagedObjectContext.init(...)`.
    pub fn new_main_queue() -> Result<Self, CoreDataError> {
        Self::new(NSManagedObjectContextConcurrencyType::MainQueue)
    }

    /// Wraps `NSManagedObjectContext.init(...)`.
    pub fn new_private_queue() -> Result<Self, CoreDataError> {
        Self::new(NSManagedObjectContextConcurrencyType::PrivateQueue)
    }

    /// Mirrors `NSManagedObjectContext.persistent_store_coordinator`.
    pub fn set_persistent_store_coordinator(
        &self,
        coordinator: &NSPersistentStoreCoordinator,
    ) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_context_set_persistent_store_coordinator(
                self.as_ptr(),
                coordinator.as_ptr(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSManagedObjectContext.has_changes(...)`.
    pub fn has_changes(&self) -> bool {
        unsafe { ffi::cd_managed_object_context_has_changes(self.as_ptr()) != 0 }
    }

    /// Wraps `NSManagedObjectContext.save(...)`.
    pub fn save(&self) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe { ffi::cd_managed_object_context_save(self.as_ptr(), &mut out_error) };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSManagedObjectContext.insert(...)`.
    pub fn insert(&self, object: &NSManagedObject) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_context_insert_object(
                self.as_ptr(),
                object.as_ptr(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSManagedObjectContext.delete(...)`.
    pub fn delete(&self, object: &NSManagedObject) {
        unsafe {
            ffi::cd_managed_object_context_delete_object(self.as_ptr(), object.as_ptr());
        }
    }

    /// Wraps `NSManagedObjectContext.perform(...)`.
    pub fn perform<F>(&self, callback: F)
    where
        F: FnOnce(NSManagedObjectContext) + Send + 'static,
    {
        let state = Box::new(PerformState {
            callback: Some(Box::new(callback) as AsyncPerformCallback),
            context: self.clone(),
        });
        unsafe {
            ffi::cd_managed_object_context_perform(
                self.as_ptr(),
                perform_trampoline,
                Box::into_raw(state).cast(),
            );
        }
    }

    /// Wraps `NSManagedObjectContext.perform_and_wait(...)`.
    pub fn perform_and_wait<F, R>(&self, callback: F) -> R
    where
        F: FnOnce(NSManagedObjectContext) -> R,
    {
        let mut state = PerformAndWaitState {
            callback: Some(callback),
            context: self.clone(),
            result: None,
        };
        unsafe {
            ffi::cd_managed_object_context_perform_and_wait(
                self.as_ptr(),
                perform_and_wait_trampoline::<F, R>,
                core::ptr::addr_of_mut!(state).cast(),
            );
        }
        state
            .result
            .expect("perform_and_wait callback did not return a value")
    }

    /// Wraps `NSManagedObjectContext.fetch(...)`.
    pub fn fetch(&self, request: &NSFetchRequest) -> Result<Vec<NSManagedObject>, CoreDataError> {
        let mut out_array = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_context_execute_fetch_request(
                self.as_ptr(),
                request.as_ptr(),
                &mut out_array,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        collect_array(out_array, "fetch results")
    }
}

impl NSManagedObject {
    /// Wraps `NSManagedObject.init(...)`.
    pub fn new(
        entity: &NSEntityDescription,
        context: Option<&NSManagedObjectContext>,
    ) -> Result<Self, CoreDataError> {
        let mut out_object = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_new(
                entity.as_ptr(),
                context.map_or(core::ptr::null_mut(), NSManagedObjectContext::as_ptr),
                &mut out_object,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_object, "managed object") }
    }

    /// Wraps `NSManagedObject.entity(...)`.
    pub fn entity(&self) -> Result<NSEntityDescription, CoreDataError> {
        let ptr = unsafe { ffi::cd_managed_object_entity(self.as_ptr()) };
        unsafe { NSEntityDescription::from_retained_ptr(ptr, "managed object entity") }
    }

    /// Mirrors `NSManagedObject.value`.
    pub fn set_value(&self, key: &str, value: impl Into<Value>) -> Result<(), CoreDataError> {
        let key = cstring_from_str(key, "managed object value key")?;
        let value = ValuePayload::from(value.into());
        let value_json = json_cstring(&value, "managed object value")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_set_value_json(
                self.as_ptr(),
                key.as_ptr(),
                value_json.as_ptr(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSManagedObject.value(...)`.
    pub fn value(&self, key: &str) -> Result<Value, CoreDataError> {
        let key = cstring_from_str(key, "managed object value key")?;
        let mut out_json = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_get_value_json(
                self.as_ptr(),
                key.as_ptr(),
                &mut out_json,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        let payload: ValuePayload = unsafe { parse_json_ptr(out_json, "managed object value")? };
        payload.try_into()
    }
}
