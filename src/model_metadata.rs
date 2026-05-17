use core::ffi::c_void;

use crate::error::CoreDataError;
use crate::ffi;
use crate::private::{
    collect_array, cstring_from_str, error_from_status, impl_object_wrapper, take_string,
};
use crate::query::NSFetchRequest;
use crate::schema::{AttributeType, NSAttributeDescription};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum NSFetchIndexElementType {
    Binary,
    RTree,
    Unknown(u64),
}

impl NSFetchIndexElementType {
    const fn from_raw(raw: u64) -> Self {
        match raw {
            0 => Self::Binary,
            1 => Self::RTree,
            other => Self::Unknown(other),
        }
    }

    const fn as_raw(self) -> u64 {
        match self {
            Self::Binary => 0,
            Self::RTree => 1,
            Self::Unknown(raw) => raw,
        }
    }
}

impl_object_wrapper!(NSPropertyDescription);
impl_object_wrapper!(NSFetchedPropertyDescription);
impl_object_wrapper!(NSExpressionDescription);
impl_object_wrapper!(NSDerivedAttributeDescription);
impl_object_wrapper!(NSCompositeAttributeDescription);
impl_object_wrapper!(NSFetchIndexDescription);
impl_object_wrapper!(NSFetchIndexElementDescription);

fn clone_retained_wrapper<T>(ptr: *mut c_void, context: &str) -> Result<T, CoreDataError>
where
    T: crate::private::FromRetainedPtr,
{
    let retained = unsafe { ffi::cd_retain_object(ptr) };
    unsafe { T::from_retained_ptr(retained, context) }
}

impl NSPropertyDescription {
    pub fn new() -> Result<Self, CoreDataError> {
        let mut out_property = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe { ffi::cd_property_description_new(&mut out_property, &mut out_error) };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_property, "property description") }
    }

    pub fn name(&self) -> Result<String, CoreDataError> {
        let ptr = unsafe { ffi::cd_property_description_get_name(self.as_ptr()) };
        unsafe { take_string(ptr) }
            .ok_or_else(|| CoreDataError::bridge(-1, "property description name was nil"))
    }

    pub fn set_name(&self, name: &str) -> Result<(), CoreDataError> {
        let name = cstring_from_str(name, "property description name")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_property_description_set_name(self.as_ptr(), name.as_ptr(), &mut out_error)
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn is_optional(&self) -> bool {
        unsafe { ffi::cd_property_description_get_optional(self.as_ptr()) != 0 }
    }

    pub fn set_optional(&self, optional: bool) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_property_description_set_optional(
                self.as_ptr(),
                i32::from(optional),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn is_transient(&self) -> bool {
        unsafe { ffi::cd_property_description_get_transient(self.as_ptr()) != 0 }
    }

    pub fn set_transient(&self, transient: bool) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_property_description_set_transient(
                self.as_ptr(),
                i32::from(transient),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }
}

impl NSFetchedPropertyDescription {
    pub fn new() -> Result<Self, CoreDataError> {
        let mut out_property = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status =
            unsafe { ffi::cd_fetched_property_description_new(&mut out_property, &mut out_error) };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_property, "fetched property description") }
    }

    pub fn fetch_request(&self) -> Result<Option<NSFetchRequest>, CoreDataError> {
        let ptr = unsafe { ffi::cd_fetched_property_description_get_fetch_request(self.as_ptr()) };
        if ptr.is_null() {
            return Ok(None);
        }
        Ok(Some(unsafe {
            NSFetchRequest::from_retained_ptr(ptr, "fetched property fetch request")?
        }))
    }

    pub fn set_fetch_request(
        &self,
        fetch_request: Option<&NSFetchRequest>,
    ) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_fetched_property_description_set_fetch_request(
                self.as_ptr(),
                fetch_request.map_or(core::ptr::null_mut(), NSFetchRequest::as_ptr),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn as_property_description(&self) -> Result<NSPropertyDescription, CoreDataError> {
        clone_retained_wrapper(self.as_ptr(), "property description")
    }
}

impl NSExpressionDescription {
    pub fn new() -> Result<Self, CoreDataError> {
        let mut out_property = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status =
            unsafe { ffi::cd_expression_description_new(&mut out_property, &mut out_error) };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_property, "expression description") }
    }

    pub fn expression_result_type(&self) -> AttributeType {
        AttributeType::from_raw(unsafe {
            ffi::cd_expression_description_get_result_type(self.as_ptr())
        })
    }

    pub fn set_expression_result_type(
        &self,
        result_type: AttributeType,
    ) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_expression_description_set_result_type(
                self.as_ptr(),
                result_type.as_raw(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn as_property_description(&self) -> Result<NSPropertyDescription, CoreDataError> {
        clone_retained_wrapper(self.as_ptr(), "property description")
    }
}

impl NSDerivedAttributeDescription {
    pub fn new() -> Result<Self, CoreDataError> {
        let mut out_attribute = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_derived_attribute_description_new(&mut out_attribute, &mut out_error)
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_attribute, "derived attribute description") }
    }

    pub fn as_attribute_description(&self) -> Result<NSAttributeDescription, CoreDataError> {
        clone_retained_wrapper(self.as_ptr(), "attribute description")
    }
}

impl NSCompositeAttributeDescription {
    pub fn new() -> Result<Self, CoreDataError> {
        let mut out_attribute = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_composite_attribute_description_new(&mut out_attribute, &mut out_error)
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_attribute, "composite attribute description") }
    }

    pub fn elements(&self) -> Result<Vec<NSAttributeDescription>, CoreDataError> {
        let array_ptr =
            unsafe { ffi::cd_composite_attribute_description_get_elements(self.as_ptr()) };
        collect_array(array_ptr, "composite attribute elements")
    }

    pub fn set_elements(&self, elements: &[&NSAttributeDescription]) -> Result<(), CoreDataError> {
        let raw_elements = elements
            .iter()
            .map(|element| element.as_ptr())
            .collect::<Vec<_>>();
        let count = i32::try_from(raw_elements.len())
            .map_err(|_| CoreDataError::bridge(-1, "composite attribute element count overflow"))?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_composite_attribute_description_set_elements(
                self.as_ptr(),
                raw_elements.as_ptr(),
                count,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn as_attribute_description(&self) -> Result<NSAttributeDescription, CoreDataError> {
        clone_retained_wrapper(self.as_ptr(), "attribute description")
    }
}

impl NSFetchIndexElementDescription {
    pub fn new(
        property: &NSPropertyDescription,
        collation_type: NSFetchIndexElementType,
    ) -> Result<Self, CoreDataError> {
        let mut out_element = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_fetch_index_element_description_new(
                property.as_ptr(),
                collation_type.as_raw(),
                &mut out_element,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_element, "fetch index element description") }
    }

    pub fn property_name(&self) -> Option<String> {
        let ptr =
            unsafe { ffi::cd_fetch_index_element_description_get_property_name(self.as_ptr()) };
        unsafe { take_string(ptr) }
    }

    pub fn collation_type(&self) -> NSFetchIndexElementType {
        NSFetchIndexElementType::from_raw(unsafe {
            ffi::cd_fetch_index_element_description_get_collation_type(self.as_ptr())
        })
    }

    pub fn set_collation_type(
        &self,
        collation_type: NSFetchIndexElementType,
    ) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_fetch_index_element_description_set_collation_type(
                self.as_ptr(),
                collation_type.as_raw(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn is_ascending(&self) -> bool {
        unsafe { ffi::cd_fetch_index_element_description_get_ascending(self.as_ptr()) != 0 }
    }

    pub fn set_ascending(&self, ascending: bool) -> Result<(), CoreDataError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_fetch_index_element_description_set_ascending(
                self.as_ptr(),
                i32::from(ascending),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }
}

impl NSFetchIndexDescription {
    pub fn new(
        name: &str,
        elements: &[&NSFetchIndexElementDescription],
    ) -> Result<Self, CoreDataError> {
        let name = cstring_from_str(name, "fetch index description name")?;
        let raw_elements = elements
            .iter()
            .map(|element| element.as_ptr())
            .collect::<Vec<_>>();
        let count = i32::try_from(raw_elements.len())
            .map_err(|_| CoreDataError::bridge(-1, "fetch index element count overflow"))?;
        let mut out_index = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_fetch_index_description_new(
                name.as_ptr(),
                raw_elements.as_ptr(),
                count,
                &mut out_index,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_index, "fetch index description") }
    }

    pub fn name(&self) -> Result<String, CoreDataError> {
        let ptr = unsafe { ffi::cd_fetch_index_description_get_name(self.as_ptr()) };
        unsafe { take_string(ptr) }
            .ok_or_else(|| CoreDataError::bridge(-1, "fetch index description name was nil"))
    }

    pub fn set_name(&self, name: &str) -> Result<(), CoreDataError> {
        let name = cstring_from_str(name, "fetch index description name")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_fetch_index_description_set_name(self.as_ptr(), name.as_ptr(), &mut out_error)
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn elements(&self) -> Result<Vec<NSFetchIndexElementDescription>, CoreDataError> {
        let array_ptr = unsafe { ffi::cd_fetch_index_description_get_elements(self.as_ptr()) };
        collect_array(array_ptr, "fetch index description elements")
    }

    pub fn set_elements(
        &self,
        elements: &[&NSFetchIndexElementDescription],
    ) -> Result<(), CoreDataError> {
        let raw_elements = elements
            .iter()
            .map(|element| element.as_ptr())
            .collect::<Vec<_>>();
        let count = i32::try_from(raw_elements.len())
            .map_err(|_| CoreDataError::bridge(-1, "fetch index element count overflow"))?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_fetch_index_description_set_elements(
                self.as_ptr(),
                raw_elements.as_ptr(),
                count,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }
}
