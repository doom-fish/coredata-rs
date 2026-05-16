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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum NSManagedObjectContextConcurrencyType {
    PrivateQueue,
    MainQueue,
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
    let mut state: Box<PerformState> = Box::from_raw(refcon.cast());
    if let Some(callback) = state.callback.take() {
        callback(state.context.clone());
    }
}

unsafe extern "C" fn perform_and_wait_trampoline<F, R>(refcon: *mut c_void)
where
    F: FnOnce(NSManagedObjectContext) -> R,
{
    let state = &mut *refcon.cast::<PerformAndWaitState<F, R>>();
    let callback = state.callback.take().expect("perform_and_wait callback missing");
    state.result = Some(callback(state.context.clone()));
}

impl NSManagedObjectContext {
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

    pub fn new_main_queue() -> Result<Self, CoreDataError> {
        Self::new(NSManagedObjectContextConcurrencyType::MainQueue)
    }

    pub fn new_private_queue() -> Result<Self, CoreDataError> {
        Self::new(NSManagedObjectContextConcurrencyType::PrivateQueue)
    }

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

    pub fn has_changes(&self) -> bool {
        unsafe { ffi::cd_managed_object_context_has_changes(self.as_ptr()) != 0 }
    }

    pub fn save(&self) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe { ffi::cd_managed_object_context_save(self.as_ptr(), &mut out_error) };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

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

    pub fn delete(&self, object: &NSManagedObject) {
        unsafe {
            ffi::cd_managed_object_context_delete_object(self.as_ptr(), object.as_ptr());
        }
    }

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
        state.result.expect("perform_and_wait callback did not return a value")
    }

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

    pub fn entity(&self) -> Result<NSEntityDescription, CoreDataError> {
        let ptr = unsafe { ffi::cd_managed_object_entity(self.as_ptr()) };
        unsafe { NSEntityDescription::from_retained_ptr(ptr, "managed object entity") }
    }

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
