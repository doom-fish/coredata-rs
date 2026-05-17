#![allow(dead_code)]

use crate::context::NSManagedObjectContext;
use crate::error::CoreDataError;
use crate::ffi;
use crate::private::{error_from_status, impl_object_wrapper};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum MergePolicyType {
    Error,
    MergeByPropertyStoreTrump,
    MergeByPropertyObjectTrump,
    Overwrite,
    Rollback,
    Unknown(u64),
}

impl MergePolicyType {
    const fn from_raw(raw: u64) -> Self {
        match raw {
            0 => Self::Error,
            1 => Self::MergeByPropertyStoreTrump,
            2 => Self::MergeByPropertyObjectTrump,
            3 => Self::Overwrite,
            4 => Self::Rollback,
            other => Self::Unknown(other),
        }
    }

    const fn as_raw(self) -> u64 {
        match self {
            Self::Error => 0,
            Self::MergeByPropertyStoreTrump => 1,
            Self::MergeByPropertyObjectTrump => 2,
            Self::Overwrite => 3,
            Self::Rollback => 4,
            Self::Unknown(raw) => raw,
        }
    }
}

impl_object_wrapper!(NSMergePolicy);
impl_object_wrapper!(NSMergeConflict);
impl_object_wrapper!(NSConstraintConflict);

impl NSMergePolicy {
    pub fn new(merge_type: MergePolicyType) -> Result<Self, CoreDataError> {
        let mut out_policy = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_merge_policy_new(merge_type.as_raw(), &mut out_policy, &mut out_error)
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_policy, "merge policy") }
    }

    pub fn error_policy() -> Result<Self, CoreDataError> {
        let ptr = unsafe { ffi::cd_merge_policy_error_policy() };
        unsafe { Self::from_retained_ptr(ptr, "error merge policy") }
    }

    pub fn rollback_policy() -> Result<Self, CoreDataError> {
        let ptr = unsafe { ffi::cd_merge_policy_rollback_policy() };
        unsafe { Self::from_retained_ptr(ptr, "rollback merge policy") }
    }

    pub fn overwrite_policy() -> Result<Self, CoreDataError> {
        let ptr = unsafe { ffi::cd_merge_policy_overwrite_policy() };
        unsafe { Self::from_retained_ptr(ptr, "overwrite merge policy") }
    }

    pub fn merge_by_property_object_trump_policy() -> Result<Self, CoreDataError> {
        let ptr = unsafe { ffi::cd_merge_policy_merge_by_property_object_trump_policy() };
        unsafe { Self::from_retained_ptr(ptr, "object trump merge policy") }
    }

    pub fn merge_by_property_store_trump_policy() -> Result<Self, CoreDataError> {
        let ptr = unsafe { ffi::cd_merge_policy_merge_by_property_store_trump_policy() };
        unsafe { Self::from_retained_ptr(ptr, "store trump merge policy") }
    }

    pub fn merge_type(&self) -> MergePolicyType {
        MergePolicyType::from_raw(unsafe { ffi::cd_merge_policy_get_merge_type(self.as_ptr()) })
    }
}

impl NSManagedObjectContext {
    pub fn merge_policy(&self) -> Result<NSMergePolicy, CoreDataError> {
        let ptr = unsafe { ffi::cd_managed_object_context_get_merge_policy(self.as_ptr()) };
        unsafe { NSMergePolicy::from_retained_ptr(ptr, "managed object context merge policy") }
    }

    pub fn set_merge_policy(&self, merge_policy: &NSMergePolicy) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_managed_object_context_set_merge_policy(
                self.as_ptr(),
                merge_policy.as_ptr(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }
}
