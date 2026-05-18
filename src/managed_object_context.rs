use crate::context::{
    NSManagedObject, NSManagedObjectContext, NSManagedObjectContextConcurrencyType,
};
use crate::error::CoreDataError;
use crate::ffi;
use crate::history::NSPersistentHistoryTransaction;
use crate::managed_object::NSManagedObjectID;
use crate::private::{collect_array, cstring_from_str, error_from_status, take_string};
use crate::query::NSFetchRequest;

impl NSManagedObjectContextConcurrencyType {
    pub(crate) const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::PrivateQueue,
            2 => Self::MainQueue,
            other => Self::Unknown(other),
        }
    }
}

impl NSManagedObjectContext {
    /// Wraps `NSManagedObjectContext.name(...)`.
    pub fn name(&self) -> Option<String> {
        let ptr = unsafe { ffi::cd_managed_object_context_get_name(self.as_ptr()) };
        unsafe { take_string(ptr) }
    }

    /// Mirrors `NSManagedObjectContext.name`.
    pub fn set_name(&self, name: Option<&str>) -> Result<(), CoreDataError> {
        let name = name
            .map(|value| cstring_from_str(value, "managed object context name"))
            .transpose()?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_context_set_name(
                self.as_ptr(),
                name.as_ref()
                    .map_or(core::ptr::null(), |name| name.as_c_str().as_ptr()),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSManagedObjectContext.parent_context(...)`.
    pub fn parent_context(&self) -> Result<Option<Self>, CoreDataError> {
        let ptr = unsafe { ffi::cd_managed_object_context_get_parent_context(self.as_ptr()) };
        if ptr.is_null() {
            return Ok(None);
        }
        Ok(Some(unsafe {
            Self::from_retained_ptr(ptr, "managed object context parent context")?
        }))
    }

    /// Mirrors `NSManagedObjectContext.parent_context`.
    pub fn set_parent_context(&self, parent_context: Option<&Self>) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_context_set_parent_context(
                self.as_ptr(),
                parent_context.map_or(core::ptr::null_mut(), Self::as_ptr),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSManagedObjectContext.concurrency_type(...)`.
    pub fn concurrency_type(&self) -> NSManagedObjectContextConcurrencyType {
        NSManagedObjectContextConcurrencyType::from_raw(unsafe {
            ffi::cd_managed_object_context_get_concurrency_type(self.as_ptr())
        })
    }

    /// Wraps `NSManagedObjectContext.inserted_objects(...)`.
    pub fn inserted_objects(&self) -> Result<Vec<NSManagedObject>, CoreDataError> {
        let array_ptr =
            unsafe { ffi::cd_managed_object_context_get_inserted_objects(self.as_ptr()) };
        collect_array(array_ptr, "managed object context inserted objects")
    }

    /// Wraps `NSManagedObjectContext.updated_objects(...)`.
    pub fn updated_objects(&self) -> Result<Vec<NSManagedObject>, CoreDataError> {
        let array_ptr =
            unsafe { ffi::cd_managed_object_context_get_updated_objects(self.as_ptr()) };
        collect_array(array_ptr, "managed object context updated objects")
    }

    /// Wraps `NSManagedObjectContext.deleted_objects(...)`.
    pub fn deleted_objects(&self) -> Result<Vec<NSManagedObject>, CoreDataError> {
        let array_ptr =
            unsafe { ffi::cd_managed_object_context_get_deleted_objects(self.as_ptr()) };
        collect_array(array_ptr, "managed object context deleted objects")
    }

    /// Wraps `NSManagedObjectContext.registered_objects(...)`.
    pub fn registered_objects(&self) -> Result<Vec<NSManagedObject>, CoreDataError> {
        let array_ptr =
            unsafe { ffi::cd_managed_object_context_get_registered_objects(self.as_ptr()) };
        collect_array(array_ptr, "managed object context registered objects")
    }

    /// Wraps `NSManagedObjectContext.object_registered_for_id(...)`.
    pub fn object_registered_for_id(
        &self,
        object_id: &NSManagedObjectID,
    ) -> Result<Option<NSManagedObject>, CoreDataError> {
        let ptr = unsafe {
            ffi::cd_managed_object_context_object_registered_for_id(
                self.as_ptr(),
                object_id.as_ptr(),
            )
        };
        if ptr.is_null() {
            return Ok(None);
        }
        Ok(Some(unsafe {
            NSManagedObject::from_retained_ptr(ptr, "managed object registered object")?
        }))
    }

    /// Wraps `NSManagedObjectContext.object_with_id(...)`.
    pub fn object_with_id(
        &self,
        object_id: &NSManagedObjectID,
    ) -> Result<NSManagedObject, CoreDataError> {
        let ptr = unsafe {
            ffi::cd_managed_object_context_object_with_id(self.as_ptr(), object_id.as_ptr())
        };
        unsafe { NSManagedObject::from_retained_ptr(ptr, "managed object context object") }
    }

    /// Wraps `NSManagedObjectContext.existing_object(...)`.
    pub fn existing_object(
        &self,
        object_id: &NSManagedObjectID,
    ) -> Result<Option<NSManagedObject>, CoreDataError> {
        let mut out_object = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_context_existing_object_with_id(
                self.as_ptr(),
                object_id.as_ptr(),
                &mut out_object,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        if out_object.is_null() {
            return Ok(None);
        }
        Ok(Some(unsafe {
            NSManagedObject::from_retained_ptr(out_object, "managed object existing object")?
        }))
    }

    /// Wraps `NSManagedObjectContext.count(...)`.
    pub fn count(&self, request: &NSFetchRequest) -> Result<usize, CoreDataError> {
        let mut out_count = 0_u64;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_context_count_for_fetch_request(
                self.as_ptr(),
                request.as_ptr(),
                &mut out_count,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        usize::try_from(out_count)
            .map_err(|_| CoreDataError::bridge(-1, "managed object context count overflow"))
    }

    /// Wraps `NSManagedObjectContext.refresh_object(...)`.
    pub fn refresh_object(&self, object: &NSManagedObject, merge_changes: bool) {
        unsafe {
            ffi::cd_managed_object_context_refresh_object(
                self.as_ptr(),
                object.as_ptr(),
                i32::from(merge_changes),
            );
        }
    }

    /// Wraps `NSManagedObjectContext.process_pending_changes(...)`.
    pub fn process_pending_changes(&self) {
        unsafe { ffi::cd_managed_object_context_process_pending_changes(self.as_ptr()) }
    }

    /// Wraps `NSManagedObjectContext.reset(...)`.
    pub fn reset(&self) {
        unsafe { ffi::cd_managed_object_context_reset(self.as_ptr()) }
    }

    /// Wraps `NSManagedObjectContext.rollback(...)`.
    pub fn rollback(&self) {
        unsafe { ffi::cd_managed_object_context_rollback(self.as_ptr()) }
    }

    /// Wraps `NSManagedObjectContext.refresh_all_objects(...)`.
    pub fn refresh_all_objects(&self) {
        unsafe { ffi::cd_managed_object_context_refresh_all_objects(self.as_ptr()) }
    }

    /// Wraps `NSManagedObjectContext.automatically_merges_changes_from_parent(...)`.
    pub fn automatically_merges_changes_from_parent(&self) -> bool {
        unsafe {
            ffi::cd_managed_object_context_get_automatically_merges_changes_from_parent(
                self.as_ptr(),
            ) != 0
        }
    }

    /// Mirrors `NSManagedObjectContext.automatically_merges_changes_from_parent`.
    pub fn set_automatically_merges_changes_from_parent(
        &self,
        automatically_merges: bool,
    ) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_context_set_automatically_merges_changes_from_parent(
                self.as_ptr(),
                i32::from(automatically_merges),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSManagedObjectContext.transaction_author(...)`.
    pub fn transaction_author(&self) -> Option<String> {
        let ptr = unsafe { ffi::cd_managed_object_context_get_transaction_author(self.as_ptr()) };
        unsafe { take_string(ptr) }
    }

    /// Mirrors `NSManagedObjectContext.transaction_author`.
    pub fn set_transaction_author(&self, author: Option<&str>) -> Result<(), CoreDataError> {
        let author = author
            .map(|value| cstring_from_str(value, "managed object context transaction author"))
            .transpose()?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_context_set_transaction_author(
                self.as_ptr(),
                author
                    .as_ref()
                    .map_or(core::ptr::null(), |author| author.as_c_str().as_ptr()),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSManagedObjectContext.obtain_permanent_ids(...)`.
    pub fn obtain_permanent_ids(&self, objects: &[&NSManagedObject]) -> Result<(), CoreDataError> {
        let raw_objects = objects
            .iter()
            .map(|object| object.as_ptr())
            .collect::<Vec<_>>();
        let count = i32::try_from(raw_objects.len())
            .map_err(|_| CoreDataError::bridge(-1, "managed object permanent ID count overflow"))?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_context_obtain_permanent_ids(
                self.as_ptr(),
                raw_objects.as_ptr(),
                count,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSManagedObjectContext.merge_changes_from_history_transaction(...)`.
    pub fn merge_changes_from_history_transaction(
        &self,
        transaction: &NSPersistentHistoryTransaction,
    ) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_context_merge_changes_from_history_transaction(
                self.as_ptr(),
                transaction.as_ptr(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }
}
