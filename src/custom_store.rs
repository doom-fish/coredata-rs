#![allow(dead_code)]

use std::collections::BTreeMap;

use crate::error::CoreDataError;
use crate::ffi;
use crate::managed_object::NSManagedObjectID;
use crate::private::{error_from_status, impl_object_wrapper, json_cstring};
use crate::value::{Value, ValuePayload};

impl_object_wrapper!(NSAtomicStore);
impl_object_wrapper!(NSAtomicStoreCacheNode);
impl_object_wrapper!(NSIncrementalStore);
impl_object_wrapper!(NSIncrementalStoreNode);

fn encode_value_dictionary(
    values: &BTreeMap<String, Value>,
    context: &str,
) -> Result<std::ffi::CString, CoreDataError> {
    let payload = values
        .iter()
        .map(|(key, value)| (key.clone(), ValuePayload::from(value)))
        .collect::<BTreeMap<_, _>>();
    json_cstring(&payload, context)
}

impl NSAtomicStoreCacheNode {
    pub fn new(object_id: &NSManagedObjectID) -> Result<Self, CoreDataError> {
        let mut out_node = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_atomic_store_cache_node_new(object_id.as_ptr(), &mut out_node, &mut out_error)
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_node, "atomic store cache node") }
    }

    pub fn object_id(&self) -> Result<NSManagedObjectID, CoreDataError> {
        let ptr = unsafe { ffi::cd_atomic_store_cache_node_get_object_id(self.as_ptr()) };
        unsafe { NSManagedObjectID::from_retained_ptr(ptr, "atomic store cache node object ID") }
    }
}

impl NSIncrementalStoreNode {
    pub fn new(
        object_id: &NSManagedObjectID,
        values: &BTreeMap<String, Value>,
        version: u64,
    ) -> Result<Self, CoreDataError> {
        let values_json = encode_value_dictionary(values, "incremental store node values")?;
        let mut out_node = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_incremental_store_node_new(
                object_id.as_ptr(),
                values_json.as_ptr(),
                version,
                &mut out_node,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_node, "incremental store node") }
    }

    pub fn object_id(&self) -> Result<NSManagedObjectID, CoreDataError> {
        let ptr = unsafe { ffi::cd_incremental_store_node_get_object_id(self.as_ptr()) };
        unsafe { NSManagedObjectID::from_retained_ptr(ptr, "incremental store node object ID") }
    }

    pub fn version(&self) -> u64 {
        unsafe { ffi::cd_incremental_store_node_get_version(self.as_ptr()) }
    }

    pub fn update(
        &self,
        values: &BTreeMap<String, Value>,
        version: u64,
    ) -> Result<(), CoreDataError> {
        let values_json = encode_value_dictionary(values, "incremental store node values")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_incremental_store_node_update(
                self.as_ptr(),
                values_json.as_ptr(),
                version,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }
}
