use std::collections::BTreeMap;

use crate::error::CoreDataError;
use crate::ffi;
use crate::private::{error_from_status, json_cstring, take_string};
use crate::query::NSPredicate;
use crate::value::{Value, ValuePayload};

fn encode_value_map(values: &BTreeMap<String, Value>) -> Result<std::ffi::CString, CoreDataError> {
    let payload = values
        .iter()
        .map(|(key, value)| (key.clone(), ValuePayload::from(value)))
        .collect::<BTreeMap<_, _>>();
    json_cstring(&payload, "predicate value map")
}

impl NSPredicate {
    /// Wraps `NSPredicate.init(...)`.
    pub fn from_value(value: bool) -> Result<Self, CoreDataError> {
        let mut out_predicate = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_predicate_new_with_value(i32::from(value), &mut out_predicate, &mut out_error)
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_predicate, "predicate") }
    }

    /// Wraps `NSPredicate.format(...)`.
    pub fn format(&self) -> Result<String, CoreDataError> {
        let ptr = unsafe { ffi::cd_predicate_get_format(self.as_ptr()) };
        unsafe { take_string(ptr) }
            .ok_or_else(|| CoreDataError::bridge(-1, "predicate format was nil"))
    }

    /// Wraps `NSPredicate.init(...)`.
    pub fn with_substitution_variables(
        &self,
        variables: &BTreeMap<String, Value>,
    ) -> Result<Self, CoreDataError> {
        let variables_json = encode_value_map(variables)?;
        let mut out_predicate = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_predicate_with_substitution_variables(
                self.as_ptr(),
                variables_json.as_ptr(),
                &mut out_predicate,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_predicate, "predicate with substitution variables") }
    }

    /// Wraps `NSPredicate.evaluate_with_object(...)`.
    pub fn evaluate_with_object(
        &self,
        object: Option<&BTreeMap<String, Value>>,
        substitution_variables: Option<&BTreeMap<String, Value>>,
    ) -> Result<bool, CoreDataError> {
        let object_json = object.map(encode_value_map).transpose()?;
        let substitution_variables_json =
            substitution_variables.map(encode_value_map).transpose()?;
        let mut out_result = 0_i32;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_predicate_evaluate_with_object_json(
                self.as_ptr(),
                object_json
                    .as_ref()
                    .map_or(core::ptr::null(), |json| json.as_ptr()),
                substitution_variables_json
                    .as_ref()
                    .map_or(core::ptr::null(), |json| json.as_ptr()),
                &mut out_result,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(out_result != 0)
    }
}
