use core::ffi::{c_char, c_void};
use core::ptr::NonNull;
use std::ffi::{CStr, CString};
use std::path::Path;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::{CoreDataError, ErrorPayload};
use crate::ffi;

#[derive(Debug)]
pub(crate) struct RetainedObject(NonNull<c_void>);

unsafe impl Send for RetainedObject {}

impl Clone for RetainedObject {
    fn clone(&self) -> Self {
        let ptr = unsafe { ffi::cd_retain_object(self.as_ptr()) };
        Self(NonNull::new(ptr).expect("Core Data retain returned a null pointer"))
    }
}

impl Drop for RetainedObject {
    fn drop(&mut self) {
        unsafe {
            ffi::cd_release_object(self.as_ptr());
        }
    }
}

impl RetainedObject {
    pub(crate) unsafe fn from_retained_ptr(
        ptr: *mut c_void,
        context: &str,
    ) -> Result<Self, CoreDataError> {
        NonNull::new(ptr)
            .map(Self)
            .ok_or_else(|| CoreDataError::bridge(-2, format!("missing object pointer for {context}")))
    }

    pub(crate) fn as_ptr(&self) -> *mut c_void {
        self.0.as_ptr()
    }
}

pub(crate) trait FromRetainedPtr: Sized {
    unsafe fn from_retained_ptr(ptr: *mut c_void, context: &str) -> Result<Self, CoreDataError>;
}

macro_rules! impl_object_wrapper {
    ($name:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub(crate) inner: $crate::private::RetainedObject,
        }

        unsafe impl Send for $name {}

        impl $name {
            pub(crate) unsafe fn from_retained_ptr(
                ptr: *mut core::ffi::c_void,
                context: &str,
            ) -> Result<Self, $crate::error::CoreDataError> {
                Ok(Self {
                    inner: $crate::private::RetainedObject::from_retained_ptr(ptr, context)?,
                })
            }

            pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
                self.inner.as_ptr()
            }
        }

        impl $crate::private::FromRetainedPtr for $name {
            unsafe fn from_retained_ptr(
                ptr: *mut core::ffi::c_void,
                context: &str,
            ) -> Result<Self, $crate::error::CoreDataError> {
                Self::from_retained_ptr(ptr, context)
            }
        }
    };
}

pub(crate) use impl_object_wrapper;

pub(crate) fn cstring_from_str(value: &str, context: &str) -> Result<CString, CoreDataError> {
    CString::new(value).map_err(|error| {
        CoreDataError::bridge(
            -1,
            format!("{context} contains an interior NUL byte: {error}"),
        )
    })
}

pub(crate) fn path_cstring(path: &Path, context: &str) -> Result<CString, CoreDataError> {
    cstring_from_str(&path.as_os_str().to_string_lossy(), context)
}

pub(crate) fn optional_cstring_from_str(
    value: Option<&str>,
    context: &str,
) -> Result<Option<CString>, CoreDataError> {
    value.map(|value| cstring_from_str(value, context)).transpose()
}

pub(crate) fn opt_cstring_ptr(value: &Option<CString>) -> *const c_char {
    value
        .as_ref()
        .map_or(core::ptr::null(), |value| value.as_c_str().as_ptr())
}

pub(crate) fn json_cstring<T: Serialize + ?Sized>(
    value: &T,
    context: &str,
) -> Result<CString, CoreDataError> {
    let json = serde_json::to_string(value).map_err(|error| {
        CoreDataError::bridge(-1, format!("failed to encode {context} as JSON: {error}"))
    })?;
    cstring_from_str(&json, context)
}

pub(crate) unsafe fn take_string(ptr: *mut c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    let string = CStr::from_ptr(ptr).to_string_lossy().into_owned();
    ffi::cd_string_free(ptr);
    Some(string)
}

pub(crate) fn parse_json_str<T: DeserializeOwned>(
    json: &str,
    context: &str,
) -> Result<T, CoreDataError> {
    serde_json::from_str(json).map_err(|error| {
        CoreDataError::bridge(
            -1,
            format!("failed to parse {context} JSON: {error}; payload={json}"),
        )
    })
}

pub(crate) unsafe fn parse_json_ptr<T: DeserializeOwned>(
    ptr: *mut c_char,
    context: &str,
) -> Result<T, CoreDataError> {
    let json = take_string(ptr)
        .ok_or_else(|| CoreDataError::bridge(-1, format!("missing JSON payload for {context}")))?;
    parse_json_str(&json, context)
}

pub(crate) fn parse_error_json_str(json: &str) -> CoreDataError {
    match serde_json::from_str::<ErrorPayload>(json) {
        Ok(payload) => CoreDataError::from_payload(payload),
        Err(error) => CoreDataError::bridge(
            -1,
            format!("failed to parse Core Data error payload: {error}; payload={json}"),
        ),
    }
}

pub(crate) unsafe fn parse_error_ptr(ptr: *mut c_char) -> CoreDataError {
    if ptr.is_null() {
        return CoreDataError::bridge(-2, "Core Data bridge returned an error without payload");
    }
    let json = CStr::from_ptr(ptr).to_string_lossy().into_owned();
    ffi::cd_string_free(ptr);
    parse_error_json_str(&json)
}

pub(crate) unsafe fn error_from_status(status: i32, err_msg: *mut c_char) -> CoreDataError {
    if !err_msg.is_null() {
        return parse_error_ptr(err_msg);
    }
    let message = match status {
        ffi::status::INVALID_ARGUMENT => "invalid argument",
        ffi::status::TIMED_OUT => "timed out waiting for Core Data",
        _ => "Core Data bridge failure",
    };
    CoreDataError::bridge(i64::from(status), message)
}

pub(crate) fn collect_array<T: FromRetainedPtr>(
    array_ptr: *mut c_void,
    context: &str,
) -> Result<Vec<T>, CoreDataError> {
    if array_ptr.is_null() {
        return Ok(Vec::new());
    }

    let array = unsafe { RetainedObject::from_retained_ptr(array_ptr, context)? };
    let count = unsafe { ffi::cd_array_count(array.as_ptr()) };
    let count = usize::try_from(count)
        .map_err(|_| CoreDataError::bridge(-1, format!("negative array count for {context}")))?;

    let mut items = Vec::with_capacity(count);
    for index in 0..count {
        let index_i32 = i32::try_from(index)
            .map_err(|_| CoreDataError::bridge(-1, format!("array index overflow for {context}")))?;
        let item_ptr = unsafe { ffi::cd_array_get_object(array.as_ptr(), index_i32) };
        let item = unsafe { T::from_retained_ptr(item_ptr, context)? };
        items.push(item);
    }
    Ok(items)
}
