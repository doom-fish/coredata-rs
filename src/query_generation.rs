use crate::context::NSManagedObjectContext;
use crate::error::CoreDataError;
use crate::ffi;
use crate::private::{error_from_status, impl_sync_object_wrapper, status_result};

impl_sync_object_wrapper!(NSQueryGenerationToken);

impl NSQueryGenerationToken {
    /// Wraps `NSQueryGenerationToken.current(...)`.
    pub fn current() -> Result<Self, CoreDataError> {
        let ptr = unsafe { ffi::cd_query_generation_token_current() };
        unsafe { Self::from_retained_ptr(ptr, "current query generation token") }
    }
}

impl NSManagedObjectContext {
    /// Wraps `NSManagedObjectContext.query_generation_token(...)`.
    pub fn query_generation_token(&self) -> Result<Option<NSQueryGenerationToken>, CoreDataError> {
        let mut out_token = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_context_get_query_generation_token(
                self.as_ptr(),
                &raw mut out_token,
                &raw mut out_error,
            )
        };
        status_result(status, out_error)?;
        if out_token.is_null() {
            return Ok(None);
        }
        Ok(Some(unsafe {
            NSQueryGenerationToken::from_retained_ptr(
                out_token,
                "managed object context query generation token",
            )?
        }))
    }

    /// Mirrors `NSManagedObjectContext.query_generation_from_token`.
    pub fn set_query_generation_from_token(
        &self,
        token: Option<&NSQueryGenerationToken>,
    ) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_context_set_query_generation_from_token(
                self.as_ptr(),
                token.map_or(core::ptr::null_mut(), NSQueryGenerationToken::as_ptr),
                &raw mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }
}
