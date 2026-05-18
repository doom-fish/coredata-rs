use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::context::NSManagedObjectContext;
use crate::error::CoreDataError;
use crate::ffi;
use crate::model::NSManagedObjectModel;
use crate::persistent_container::NSPersistentStoreDescription;
use crate::persistent_store_coordinator::NSPersistentStore;
use crate::private::{
    collect_array, cstring_from_str, error_from_status, impl_object_wrapper, parse_error_ptr,
    parse_json_ptr, take_string,
};
use crate::query::NSFetchRequest;
use crate::store::NSPersistentStoreCoordinator;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
/// Mirrors the corresponding Core Data `CloudKitDatabaseScope` value.
pub enum CloudKitDatabaseScope {
    /// Mirrors `CloudKitDatabaseScope::Public`.
    Public,
    /// Mirrors `CloudKitDatabaseScope::Private`.
    Private,
    /// Mirrors `CloudKitDatabaseScope::Shared`.
    Shared,
    /// Mirrors `CloudKitDatabaseScope::Unknown`.
    Unknown(i64),
}

impl CloudKitDatabaseScope {
    const fn from_raw(raw: i64) -> Self {
        match raw {
            1 => Self::Public,
            2 => Self::Private,
            3 => Self::Shared,
            other => Self::Unknown(other),
        }
    }

    const fn as_raw(self) -> i64 {
        match self {
            Self::Public => 1,
            Self::Private => 2,
            Self::Shared => 3,
            Self::Unknown(raw) => raw,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Wraps `CloudKitSchemaInitializationOptions`.
pub struct CloudKitSchemaInitializationOptions(u64);

impl CloudKitSchemaInitializationOptions {
    /// Mirrors the corresponding `CloudKitSchemaInitializationOptions` constant.
    pub const NONE: Self = Self(0);
    /// Mirrors the corresponding `CloudKitSchemaInitializationOptions` constant.
    pub const DRY_RUN: Self = Self(1 << 1);
    /// Mirrors the corresponding `CloudKitSchemaInitializationOptions` constant.
    pub const PRINT_SCHEMA: Self = Self(1 << 2);

    /// Mirrors the corresponding `CloudKitSchemaInitializationOptions` constant.
    pub const fn bits(self) -> u64 {
        self.0
    }

    /// Mirrors the corresponding `CloudKitSchemaInitializationOptions` constant.
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

impl_object_wrapper!(NSPersistentCloudKitContainerOptions);
impl_object_wrapper!(NSPersistentCloudKitContainer);

impl NSPersistentCloudKitContainerOptions {
    /// Wraps `NSPersistentCloudKitContainerOptions.init(...)`.
    pub fn new(container_identifier: &str) -> Result<Self, CoreDataError> {
        let container_identifier =
            cstring_from_str(container_identifier, "CloudKit container identifier")?;
        let mut out_options = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_cloudkit_container_options_new(
                container_identifier.as_ptr(),
                &mut out_options,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_options, "CloudKit container options") }
    }

    /// Wraps `NSPersistentCloudKitContainerOptions.container_identifier(...)`.
    pub fn container_identifier(&self) -> Result<String, CoreDataError> {
        let ptr = unsafe {
            ffi::cd_persistent_cloudkit_container_options_get_container_identifier(self.as_ptr())
        };
        unsafe { take_string(ptr) }
            .ok_or_else(|| CoreDataError::bridge(-1, "CloudKit container identifier was nil"))
    }

    /// Wraps `NSPersistentCloudKitContainerOptions.database_scope(...)`.
    pub fn database_scope(&self) -> CloudKitDatabaseScope {
        CloudKitDatabaseScope::from_raw(unsafe {
            ffi::cd_persistent_cloudkit_container_options_get_database_scope(self.as_ptr())
        })
    }

    /// Mirrors `NSPersistentCloudKitContainerOptions.database_scope`.
    pub fn set_database_scope(
        &self,
        database_scope: CloudKitDatabaseScope,
    ) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_cloudkit_container_options_set_database_scope(
                self.as_ptr(),
                database_scope.as_raw(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }
}

impl NSPersistentStoreDescription {
    /// Wraps `NSPersistentStoreDescription.cloudkit_container_options(...)`.
    pub fn cloudkit_container_options(
        &self,
    ) -> Result<Option<NSPersistentCloudKitContainerOptions>, CoreDataError> {
        let ptr = unsafe {
            ffi::cd_persistent_store_description_get_cloudkit_container_options(self.as_ptr())
        };
        if ptr.is_null() {
            return Ok(None);
        }
        Ok(Some(unsafe {
            NSPersistentCloudKitContainerOptions::from_retained_ptr(
                ptr,
                "CloudKit container options",
            )?
        }))
    }

    /// Mirrors `NSPersistentStoreDescription.cloudkit_container_options`.
    pub fn set_cloudkit_container_options(
        &self,
        options: Option<&NSPersistentCloudKitContainerOptions>,
    ) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_store_description_set_cloudkit_container_options(
                self.as_ptr(),
                options.map_or(
                    core::ptr::null_mut(),
                    NSPersistentCloudKitContainerOptions::as_ptr,
                ),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }
}

impl NSPersistentCloudKitContainer {
    /// Wraps `NSPersistentCloudKitContainer.init(...)`.
    pub fn new(name: &str, model: &NSManagedObjectModel) -> Result<Self, CoreDataError> {
        let name = cstring_from_str(name, "CloudKit container name")?;
        let mut out_container = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_cloudkit_container_new(
                name.as_ptr(),
                model.as_ptr(),
                &mut out_container,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_container, "CloudKit container") }
    }

    /// Wraps `NSPersistentCloudKitContainer.name(...)`.
    pub fn name(&self) -> Result<String, CoreDataError> {
        let ptr = unsafe { ffi::cd_persistent_cloudkit_container_get_name(self.as_ptr()) };
        unsafe { take_string(ptr) }
            .ok_or_else(|| CoreDataError::bridge(-1, "CloudKit container name was nil"))
    }

    /// Wraps `NSPersistentCloudKitContainer.managed_object_model(...)`.
    pub fn managed_object_model(&self) -> Result<NSManagedObjectModel, CoreDataError> {
        let ptr =
            unsafe { ffi::cd_persistent_cloudkit_container_managed_object_model(self.as_ptr()) };
        unsafe { NSManagedObjectModel::from_retained_ptr(ptr, "CloudKit container model") }
    }

    /// Wraps `NSPersistentCloudKitContainer.persistent_store_coordinator(...)`.
    pub fn persistent_store_coordinator(
        &self,
    ) -> Result<NSPersistentStoreCoordinator, CoreDataError> {
        let ptr = unsafe {
            ffi::cd_persistent_cloudkit_container_persistent_store_coordinator(self.as_ptr())
        };
        unsafe {
            NSPersistentStoreCoordinator::from_retained_ptr(ptr, "CloudKit container coordinator")
        }
    }

    /// Wraps `NSPersistentCloudKitContainer.persistent_store_descriptions(...)`.
    pub fn persistent_store_descriptions(
        &self,
    ) -> Result<Vec<NSPersistentStoreDescription>, CoreDataError> {
        let array_ptr = unsafe {
            ffi::cd_persistent_cloudkit_container_persistent_store_descriptions(self.as_ptr())
        };
        collect_array(
            array_ptr,
            "CloudKit container persistent store descriptions",
        )
    }

    /// Mirrors `NSPersistentCloudKitContainer.persistent_store_descriptions`.
    pub fn set_persistent_store_descriptions(
        &self,
        descriptions: &[&NSPersistentStoreDescription],
    ) -> Result<(), CoreDataError> {
        let raw_descriptions = descriptions
            .iter()
            .map(|description| description.as_ptr())
            .collect::<Vec<_>>();
        let count = i32::try_from(raw_descriptions.len()).map_err(|_| {
            CoreDataError::bridge(-1, "CloudKit container description count overflow")
        })?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_cloudkit_container_set_persistent_store_descriptions(
                self.as_ptr(),
                raw_descriptions.as_ptr(),
                count,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSPersistentCloudKitContainer.load_persistent_stores_with_timeout(...)`.
    pub fn load_persistent_stores_with_timeout(
        &self,
        timeout_seconds: i32,
    ) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_cloudkit_container_load_persistent_stores(
                self.as_ptr(),
                timeout_seconds,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSPersistentCloudKitContainer.load_persistent_stores(...)`.
    pub fn load_persistent_stores(&self) -> Result<(), CoreDataError> {
        self.load_persistent_stores_with_timeout(30)
    }

    /// Wraps `NSPersistentCloudKitContainer.view_context(...)`.
    pub fn view_context(&self) -> Result<NSManagedObjectContext, CoreDataError> {
        let ptr = unsafe { ffi::cd_persistent_cloudkit_container_view_context(self.as_ptr()) };
        unsafe { NSManagedObjectContext::from_retained_ptr(ptr, "CloudKit container view context") }
    }

    /// Wraps `NSPersistentCloudKitContainer.new_background_context(...)`.
    pub fn new_background_context(&self) -> Result<NSManagedObjectContext, CoreDataError> {
        let ptr =
            unsafe { ffi::cd_persistent_cloudkit_container_new_background_context(self.as_ptr()) };
        unsafe {
            NSManagedObjectContext::from_retained_ptr(ptr, "CloudKit container background context")
        }
    }

    /// Wraps `NSPersistentCloudKitContainer.initialize_schema(...)`.
    pub fn initialize_schema(
        &self,
        options: CloudKitSchemaInitializationOptions,
    ) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_cloudkit_container_initialize_schema(
                self.as_ptr(),
                options.bits(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps `NSPersistentCloudKitContainer.can_update_record_for_managed_object_with_id(...)`.
    pub fn can_update_record_for_managed_object_with_id(
        &self,
        object_id: &crate::managed_object::NSManagedObjectID,
    ) -> bool {
        unsafe {
            ffi::cd_persistent_cloudkit_container_can_update_record_for_managed_object_id(
                self.as_ptr(),
                object_id.as_ptr(),
            ) != 0
        }
    }

    /// Wraps `NSPersistentCloudKitContainer.can_delete_record_for_managed_object_with_id(...)`.
    pub fn can_delete_record_for_managed_object_with_id(
        &self,
        object_id: &crate::managed_object::NSManagedObjectID,
    ) -> bool {
        unsafe {
            ffi::cd_persistent_cloudkit_container_can_delete_record_for_managed_object_id(
                self.as_ptr(),
                object_id.as_ptr(),
            ) != 0
        }
    }

    /// Wraps `NSPersistentCloudKitContainer.can_modify_managed_objects_in_store(...)`.
    pub fn can_modify_managed_objects_in_store(&self, store: &NSPersistentStore) -> bool {
        unsafe {
            ffi::cd_persistent_cloudkit_container_can_modify_managed_objects_in_store(
                self.as_ptr(),
                store.as_ptr(),
            ) != 0
        }
    }
}

/// Core Data items for event notification names.
pub mod event_notification_names {
    /// Mirrors `NSPersistentCloudKitContainerEventChangedNotification`.
    pub const CHANGED: &str = "NSPersistentCloudKitContainerEventChangedNotification";
}

/// Core Data items for event user info keys.
pub mod event_user_info_keys {
    /// Mirrors `NSPersistentCloudKitContainerEventUserInfoKey`.
    pub const EVENT: &str = "NSPersistentCloudKitContainerEventUserInfoKey";
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
/// Mirrors the corresponding Core Data `NSPersistentCloudKitContainerEventType` value.
pub enum NSPersistentCloudKitContainerEventType {
    /// Mirrors `NSPersistentCloudKitContainerEventType::Setup`.
    Setup,
    /// Mirrors `NSPersistentCloudKitContainerEventType::Import`.
    Import,
    /// Mirrors `NSPersistentCloudKitContainerEventType::Export`.
    Export,
    /// Mirrors `NSPersistentCloudKitContainerEventType::Unknown`.
    Unknown(i64),
}

impl NSPersistentCloudKitContainerEventType {
    const fn from_raw(raw: i64) -> Self {
        match raw {
            0 => Self::Setup,
            1 => Self::Import,
            2 => Self::Export,
            other => Self::Unknown(other),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
/// Mirrors the corresponding Core Data `NSPersistentCloudKitContainerEventResultType` value.
pub enum NSPersistentCloudKitContainerEventResultType {
    /// Mirrors `NSPersistentCloudKitContainerEventResultType::Events`.
    Events,
    /// Mirrors `NSPersistentCloudKitContainerEventResultType::CountEvents`.
    CountEvents,
    /// Mirrors `NSPersistentCloudKitContainerEventResultType::Unknown`.
    Unknown(i64),
}

impl NSPersistentCloudKitContainerEventResultType {
    const fn from_raw(raw: i64) -> Self {
        match raw {
            0 => Self::Events,
            1 => Self::CountEvents,
            other => Self::Unknown(other),
        }
    }

    const fn as_raw(self) -> i64 {
        match self {
            Self::Events => 0,
            Self::CountEvents => 1,
            Self::Unknown(raw) => raw,
        }
    }
}

impl_object_wrapper!(NSPersistentCloudKitContainerEvent);
impl_object_wrapper!(NSPersistentCloudKitContainerEventRequest);
impl_object_wrapper!(NSPersistentCloudKitContainerEventResult);

fn seconds_since_epoch(time: SystemTime) -> Result<f64, CoreDataError> {
    time.duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs_f64())
        .map_err(|error| {
            CoreDataError::bridge(-1, format!("invalid CloudKit event timestamp: {error}"))
        })
}

fn system_time_from_seconds(seconds: f64) -> SystemTime {
    UNIX_EPOCH + Duration::from_secs_f64(seconds)
}

impl NSPersistentCloudKitContainerEvent {
    /// Wraps `NSPersistentCloudKitContainerEvent.identifier(...)`.
    pub fn identifier(&self) -> Result<String, CoreDataError> {
        let ptr = unsafe { ffi::cd_persistent_cloudkit_event_get_identifier(self.as_ptr()) };
        unsafe { take_string(ptr) }
            .ok_or_else(|| CoreDataError::bridge(-1, "CloudKit event identifier was nil"))
    }

    /// Wraps `NSPersistentCloudKitContainerEvent.store_identifier(...)`.
    pub fn store_identifier(&self) -> Result<String, CoreDataError> {
        let ptr = unsafe { ffi::cd_persistent_cloudkit_event_get_store_identifier(self.as_ptr()) };
        unsafe { take_string(ptr) }
            .ok_or_else(|| CoreDataError::bridge(-1, "CloudKit event store identifier was nil"))
    }

    /// Wraps `NSPersistentCloudKitContainerEvent.event_type(...)`.
    pub fn event_type(&self) -> NSPersistentCloudKitContainerEventType {
        NSPersistentCloudKitContainerEventType::from_raw(unsafe {
            ffi::cd_persistent_cloudkit_event_get_type(self.as_ptr())
        })
    }

    /// Wraps `NSPersistentCloudKitContainerEvent.start_date(...)`.
    pub fn start_date(&self) -> SystemTime {
        system_time_from_seconds(unsafe {
            ffi::cd_persistent_cloudkit_event_get_start_timestamp(self.as_ptr())
        })
    }

    /// Wraps `NSPersistentCloudKitContainerEvent.end_date(...)`.
    pub fn end_date(&self) -> Option<SystemTime> {
        let has_end_date =
            unsafe { ffi::cd_persistent_cloudkit_event_has_end_date(self.as_ptr()) != 0 };
        if !has_end_date {
            return None;
        }
        Some(system_time_from_seconds(unsafe {
            ffi::cd_persistent_cloudkit_event_get_end_timestamp(self.as_ptr())
        }))
    }

    /// Wraps `NSPersistentCloudKitContainerEvent.succeeded(...)`.
    pub fn succeeded(&self) -> bool {
        unsafe { ffi::cd_persistent_cloudkit_event_get_succeeded(self.as_ptr()) != 0 }
    }

    /// Wraps `NSPersistentCloudKitContainerEvent.error(...)`.
    pub fn error(&self) -> Option<CoreDataError> {
        let ptr = unsafe { ffi::cd_persistent_cloudkit_event_get_error_json(self.as_ptr()) };
        if ptr.is_null() {
            return None;
        }
        Some(unsafe { parse_error_ptr(ptr) })
    }
}

impl NSPersistentCloudKitContainerEventRequest {
    /// Wraps `NSPersistentCloudKitContainerEventRequest.fetch_events_after_date(...)`.
    pub fn fetch_events_after_date(time: SystemTime) -> Result<Self, CoreDataError> {
        let timestamp = seconds_since_epoch(time)?;
        let mut out_request = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_cloudkit_event_request_fetch_after_date(
                timestamp,
                &mut out_request,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_request, "CloudKit event request") }
    }

    /// Wraps `NSPersistentCloudKitContainerEventRequest.fetch_events_after_event(...)`.
    pub fn fetch_events_after_event(
        event: Option<&NSPersistentCloudKitContainerEvent>,
    ) -> Result<Self, CoreDataError> {
        let mut out_request = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_cloudkit_event_request_fetch_after_event(
                event.map_or(
                    core::ptr::null_mut(),
                    NSPersistentCloudKitContainerEvent::as_ptr,
                ),
                &mut out_request,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_request, "CloudKit event request") }
    }

    /// Wraps `NSPersistentCloudKitContainerEventRequest.fetch_request_for_events(...)`.
    pub fn fetch_request_for_events() -> Result<NSFetchRequest, CoreDataError> {
        let mut out_fetch_request = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_cloudkit_event_request_fetch_request_for_events(
                &mut out_fetch_request,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe {
            NSFetchRequest::from_retained_ptr(out_fetch_request, "CloudKit event fetch request")
        }
    }

    /// Wraps `NSPersistentCloudKitContainerEventRequest.result_type(...)`.
    pub fn result_type(&self) -> NSPersistentCloudKitContainerEventResultType {
        NSPersistentCloudKitContainerEventResultType::from_raw(unsafe {
            ffi::cd_persistent_cloudkit_event_request_get_result_type(self.as_ptr())
        })
    }

    /// Mirrors `NSPersistentCloudKitContainerEventRequest.result_type`.
    pub fn set_result_type(&self, result_type: NSPersistentCloudKitContainerEventResultType) {
        unsafe {
            ffi::cd_persistent_cloudkit_event_request_set_result_type(
                self.as_ptr(),
                result_type.as_raw(),
            );
        }
    }

    /// Wraps `NSPersistentCloudKitContainerEventRequest.execute(...)`.
    pub fn execute(
        &self,
        context: &NSManagedObjectContext,
    ) -> Result<NSPersistentCloudKitContainerEventResult, CoreDataError> {
        let mut out_result = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_context_execute_persistent_cloudkit_event_request(
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
            NSPersistentCloudKitContainerEventResult::from_retained_ptr(
                out_result,
                "CloudKit event result",
            )
        }
    }
}

impl NSPersistentCloudKitContainerEventResult {
    /// Wraps `NSPersistentCloudKitContainerEventResult.result_type(...)`.
    pub fn result_type(&self) -> NSPersistentCloudKitContainerEventResultType {
        NSPersistentCloudKitContainerEventResultType::from_raw(unsafe {
            ffi::cd_persistent_cloudkit_event_result_get_result_type(self.as_ptr())
        })
    }

    /// Wraps `NSPersistentCloudKitContainerEventResult.events(...)`.
    pub fn events(&self) -> Result<Vec<NSPersistentCloudKitContainerEvent>, CoreDataError> {
        let array_ptr =
            unsafe { ffi::cd_persistent_cloudkit_event_result_get_events(self.as_ptr()) };
        collect_array(array_ptr, "CloudKit event result events")
    }

    /// Wraps `NSPersistentCloudKitContainerEventResult.counts(...)`.
    pub fn counts(&self) -> Result<Vec<usize>, CoreDataError> {
        let mut out_json = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_persistent_cloudkit_event_result_get_counts_json(
                self.as_ptr(),
                &mut out_json,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        let raw_counts: Vec<u64> =
            unsafe { parse_json_ptr(out_json, "CloudKit event result counts") }?;
        raw_counts
            .into_iter()
            .map(|count| {
                usize::try_from(count)
                    .map_err(|_| CoreDataError::bridge(-1, "CloudKit event count overflow"))
            })
            .collect()
    }
}
