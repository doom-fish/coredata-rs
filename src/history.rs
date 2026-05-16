use std::collections::BTreeMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::context::NSManagedObjectContext;
use crate::error::CoreDataError;
use crate::ffi;
use crate::managed_object::NSManagedObjectID;
use crate::private::{
    collect_array, error_from_status, impl_object_wrapper, parse_json_ptr, take_string,
};
use crate::value::{Value, ValuePayload};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PersistentHistoryResultType {
    StatusOnly,
    ObjectIds,
    Count,
    TransactionsOnly,
    ChangesOnly,
    TransactionsAndChanges,
    Unknown(i64),
}

impl PersistentHistoryResultType {
    const fn from_raw(raw: i64) -> Self {
        match raw {
            0 => Self::StatusOnly,
            1 => Self::ObjectIds,
            2 => Self::Count,
            3 => Self::TransactionsOnly,
            4 => Self::ChangesOnly,
            5 => Self::TransactionsAndChanges,
            other => Self::Unknown(other),
        }
    }

    const fn as_raw(self) -> i64 {
        match self {
            Self::StatusOnly => 0,
            Self::ObjectIds => 1,
            Self::Count => 2,
            Self::TransactionsOnly => 3,
            Self::ChangesOnly => 4,
            Self::TransactionsAndChanges => 5,
            Self::Unknown(raw) => raw,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PersistentHistoryChangeType {
    Insert,
    Update,
    Delete,
    Unknown(i64),
}

impl PersistentHistoryChangeType {
    const fn from_raw(raw: i64) -> Self {
        match raw {
            0 => Self::Insert,
            1 => Self::Update,
            2 => Self::Delete,
            other => Self::Unknown(other),
        }
    }
}

impl_object_wrapper!(NSPersistentHistoryToken);
impl_object_wrapper!(NSPersistentHistoryChangeRequest);
impl_object_wrapper!(NSPersistentHistoryResult);
impl_object_wrapper!(NSPersistentHistoryTransaction);
impl_object_wrapper!(NSPersistentHistoryChange);

fn seconds_since_epoch(time: SystemTime) -> Result<f64, CoreDataError> {
    time.duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs_f64())
        .map_err(|error| CoreDataError::bridge(-1, format!("invalid history timestamp: {error}")))
}

fn system_time_from_seconds(seconds: f64) -> SystemTime {
    UNIX_EPOCH + Duration::from_secs_f64(seconds)
}

fn value_map_from_json_ptr(
    ptr: *mut core::ffi::c_char,
    context: &str,
) -> Result<BTreeMap<String, Value>, CoreDataError> {
    let payloads: BTreeMap<String, ValuePayload> = unsafe { parse_json_ptr(ptr, context)? };
    payloads
        .into_iter()
        .map(|(key, value)| value.try_into().map(|value| (key, value)))
        .collect()
}

impl NSPersistentHistoryChangeRequest {
    pub fn fetch_history_after_token(
        token: Option<&NSPersistentHistoryToken>,
    ) -> Result<Self, CoreDataError> {
        let mut out_request = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_history_change_request_fetch_after_token(
                token.map_or(core::ptr::null_mut(), NSPersistentHistoryToken::as_ptr),
                &mut out_request,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_request, "persistent history change request") }
    }

    pub fn fetch_history_after_date(time: SystemTime) -> Result<Self, CoreDataError> {
        let timestamp = seconds_since_epoch(time)?;
        let mut out_request = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_history_change_request_fetch_after_date(
                timestamp,
                &mut out_request,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_request, "persistent history change request") }
    }

    pub fn fetch_history_after_transaction(
        transaction: Option<&NSPersistentHistoryTransaction>,
    ) -> Result<Self, CoreDataError> {
        let mut out_request = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_history_change_request_fetch_after_transaction(
                transaction.map_or(
                    core::ptr::null_mut(),
                    NSPersistentHistoryTransaction::as_ptr,
                ),
                &mut out_request,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_request, "persistent history change request") }
    }

    pub fn delete_history_before_token(
        token: Option<&NSPersistentHistoryToken>,
    ) -> Result<Self, CoreDataError> {
        let mut out_request = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_history_change_request_delete_before_token(
                token.map_or(core::ptr::null_mut(), NSPersistentHistoryToken::as_ptr),
                &mut out_request,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_request, "persistent history delete request") }
    }

    pub fn delete_history_before_date(time: SystemTime) -> Result<Self, CoreDataError> {
        let timestamp = seconds_since_epoch(time)?;
        let mut out_request = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_history_change_request_delete_before_date(
                timestamp,
                &mut out_request,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_request, "persistent history delete request") }
    }

    pub fn delete_history_before_transaction(
        transaction: Option<&NSPersistentHistoryTransaction>,
    ) -> Result<Self, CoreDataError> {
        let mut out_request = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_history_change_request_delete_before_transaction(
                transaction.map_or(
                    core::ptr::null_mut(),
                    NSPersistentHistoryTransaction::as_ptr,
                ),
                &mut out_request,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_request, "persistent history delete request") }
    }

    pub fn result_type(&self) -> PersistentHistoryResultType {
        PersistentHistoryResultType::from_raw(unsafe {
            ffi::cd_persistent_history_change_request_get_result_type(self.as_ptr())
        })
    }

    pub fn set_result_type(&self, result_type: PersistentHistoryResultType) {
        unsafe {
            ffi::cd_persistent_history_change_request_set_result_type(
                self.as_ptr(),
                result_type.as_raw(),
            );
        }
    }

    pub fn execute(
        &self,
        context: &NSManagedObjectContext,
    ) -> Result<NSPersistentHistoryResult, CoreDataError> {
        let mut out_result = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_context_execute_persistent_history_change_request(
                context.as_ptr(),
                self.as_ptr(),
                &mut out_result,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe {
            NSPersistentHistoryResult::from_retained_ptr(out_result, "persistent history result")
        }
    }
}

impl NSPersistentHistoryResult {
    pub fn result_type(&self) -> PersistentHistoryResultType {
        PersistentHistoryResultType::from_raw(unsafe {
            ffi::cd_persistent_history_result_get_result_type(self.as_ptr())
        })
    }

    pub fn status(&self) -> bool {
        unsafe { ffi::cd_persistent_history_result_get_status(self.as_ptr()) != 0 }
    }

    pub fn count(&self) -> usize {
        unsafe { ffi::cd_persistent_history_result_get_count(self.as_ptr()) as usize }
    }

    pub fn object_ids(&self) -> Result<Vec<NSManagedObjectID>, CoreDataError> {
        let array_ptr = unsafe { ffi::cd_persistent_history_result_get_object_ids(self.as_ptr()) };
        collect_array(array_ptr, "persistent history result object IDs")
    }

    pub fn transactions(&self) -> Result<Vec<NSPersistentHistoryTransaction>, CoreDataError> {
        let array_ptr =
            unsafe { ffi::cd_persistent_history_result_get_transactions(self.as_ptr()) };
        collect_array(array_ptr, "persistent history transactions")
    }

    pub fn changes(&self) -> Result<Vec<NSPersistentHistoryChange>, CoreDataError> {
        let array_ptr = unsafe { ffi::cd_persistent_history_result_get_changes(self.as_ptr()) };
        collect_array(array_ptr, "persistent history changes")
    }
}

impl NSPersistentHistoryTransaction {
    pub fn timestamp(&self) -> SystemTime {
        system_time_from_seconds(unsafe {
            ffi::cd_persistent_history_transaction_get_timestamp(self.as_ptr())
        })
    }

    pub fn changes(&self) -> Result<Vec<NSPersistentHistoryChange>, CoreDataError> {
        let array_ptr =
            unsafe { ffi::cd_persistent_history_transaction_get_changes(self.as_ptr()) };
        collect_array(array_ptr, "persistent history transaction changes")
    }

    pub fn transaction_number(&self) -> i64 {
        unsafe { ffi::cd_persistent_history_transaction_get_transaction_number(self.as_ptr()) }
    }

    pub fn store_id(&self) -> Result<String, CoreDataError> {
        let ptr = unsafe { ffi::cd_persistent_history_transaction_get_store_id(self.as_ptr()) };
        unsafe { take_string(ptr) }.ok_or_else(|| {
            CoreDataError::bridge(-1, "persistent history transaction store ID was nil")
        })
    }

    pub fn bundle_id(&self) -> Result<String, CoreDataError> {
        let ptr = unsafe { ffi::cd_persistent_history_transaction_get_bundle_id(self.as_ptr()) };
        unsafe { take_string(ptr) }.ok_or_else(|| {
            CoreDataError::bridge(-1, "persistent history transaction bundle ID was nil")
        })
    }

    pub fn process_id(&self) -> Result<String, CoreDataError> {
        let ptr = unsafe { ffi::cd_persistent_history_transaction_get_process_id(self.as_ptr()) };
        unsafe { take_string(ptr) }.ok_or_else(|| {
            CoreDataError::bridge(-1, "persistent history transaction process ID was nil")
        })
    }

    pub fn context_name(&self) -> Option<String> {
        let ptr = unsafe { ffi::cd_persistent_history_transaction_get_context_name(self.as_ptr()) };
        unsafe { take_string(ptr) }
    }

    pub fn author(&self) -> Option<String> {
        let ptr = unsafe { ffi::cd_persistent_history_transaction_get_author(self.as_ptr()) };
        unsafe { take_string(ptr) }
    }

    pub fn token(&self) -> Result<NSPersistentHistoryToken, CoreDataError> {
        let ptr = unsafe { ffi::cd_persistent_history_transaction_get_token(self.as_ptr()) };
        unsafe { NSPersistentHistoryToken::from_retained_ptr(ptr, "persistent history token") }
    }
}

impl NSPersistentHistoryChange {
    pub fn change_id(&self) -> i64 {
        unsafe { ffi::cd_persistent_history_change_get_change_id(self.as_ptr()) }
    }

    pub fn change_type(&self) -> PersistentHistoryChangeType {
        PersistentHistoryChangeType::from_raw(unsafe {
            ffi::cd_persistent_history_change_get_change_type(self.as_ptr())
        })
    }

    pub fn changed_object_id(&self) -> Result<NSManagedObjectID, CoreDataError> {
        let ptr = unsafe { ffi::cd_persistent_history_change_get_changed_object_id(self.as_ptr()) };
        unsafe { NSManagedObjectID::from_retained_ptr(ptr, "persistent history changed object ID") }
    }

    pub fn tombstone(&self) -> Result<BTreeMap<String, Value>, CoreDataError> {
        let mut out_json = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_history_change_get_tombstone_json(
                self.as_ptr(),
                &mut out_json,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        value_map_from_json_ptr(out_json, "persistent history tombstone")
    }

    pub fn updated_properties(&self) -> Result<Vec<String>, CoreDataError> {
        let mut out_json = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_history_change_get_updated_properties_json(
                self.as_ptr(),
                &mut out_json,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { parse_json_ptr(out_json, "persistent history updated properties") }
    }

    pub fn transaction(&self) -> Result<Option<NSPersistentHistoryTransaction>, CoreDataError> {
        let ptr = unsafe { ffi::cd_persistent_history_change_get_transaction(self.as_ptr()) };
        if ptr.is_null() {
            return Ok(None);
        }
        Ok(Some(unsafe {
            NSPersistentHistoryTransaction::from_retained_ptr(
                ptr,
                "persistent history change transaction",
            )?
        }))
    }
}
