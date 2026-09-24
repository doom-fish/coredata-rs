use crate::context::{NSManagedObject, NSManagedObjectContext};
use crate::error::CoreDataError;
use crate::ffi;
use crate::private::{
    cstring_from_str, error_from_status, impl_copied_object_wrapper, impl_sync_object_wrapper,
    json_cstring, status_result,
};
use crate::value::{Value, ValuePayload};

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
/// Wraps `SortDescriptor`.
pub struct SortDescriptor {
    /// Mirrors `SortDescriptor.key`.
    pub key: String,
    /// Mirrors `SortDescriptor.ascending`.
    pub ascending: bool,
}

impl SortDescriptor {
    /// Wraps `SortDescriptor.ascending(...)`.
    pub fn ascending(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            ascending: true,
        }
    }

    /// Wraps `SortDescriptor.descending(...)`.
    pub fn descending(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            ascending: false,
        }
    }
}

impl_sync_object_wrapper!(NSPredicate);
impl_copied_object_wrapper!(NSFetchRequest);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PredicateOperator {
    LessThan,
    LessThanOrEqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
    EqualTo,
    NotEqualTo,
    Matches,
    Like,
    BeginsWith,
    EndsWith,
    Contains,
}

impl PredicateOperator {
    const fn as_raw(self) -> u64 {
        match self {
            Self::LessThan => 0,
            Self::LessThanOrEqualTo => 1,
            Self::GreaterThan => 2,
            Self::GreaterThanOrEqualTo => 3,
            Self::EqualTo => 4,
            Self::NotEqualTo => 5,
            Self::Matches => 6,
            Self::Like => 7,
            Self::BeginsWith => 8,
            Self::EndsWith => 9,
            Self::Contains => 99,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PredicateOptions(u64);

impl PredicateOptions {
    pub const NONE: Self = Self(0);
    pub const CASE_INSENSITIVE: Self = Self(1);
    pub const DIACRITIC_INSENSITIVE: Self = Self(2);
    pub const NORMALIZED: Self = Self(4);

    pub const fn bits(self) -> u64 {
        self.0
    }

    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

const COMPOUND_NOT: u64 = 0;
const COMPOUND_AND: u64 = 1;
const COMPOUND_OR: u64 = 2;

impl NSPredicate {
    /// Wraps `NSPredicate.init(...)`.
    pub fn from_format(
        format: impl AsRef<str>,
        arguments: &[Value],
    ) -> Result<Self, CoreDataError> {
        let format = cstring_from_str(format.as_ref(), "predicate format")?;
        let payload = arguments.iter().map(ValuePayload::from).collect::<Vec<_>>();
        let arguments_json = json_cstring(&payload, "predicate arguments")?;
        let mut out_predicate = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_predicate_new_with_format(
                format.as_ptr(),
                arguments_json.as_ptr(),
                &raw mut out_predicate,
                &raw mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_predicate, "predicate") }
    }

    pub fn comparison(
        key_path: &str,
        operator: PredicateOperator,
        value: impl Into<Value>,
        options: PredicateOptions,
    ) -> Result<Self, CoreDataError> {
        let key_path = cstring_from_str(key_path, "predicate key path")?;
        let value = ValuePayload::from(value.into());
        let value_json = json_cstring(&value, "predicate comparison value")?;
        let mut out_predicate = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_predicate_new_comparison(
                key_path.as_ptr(),
                operator.as_raw(),
                value_json.as_ptr(),
                options.bits(),
                &raw mut out_predicate,
                &raw mut out_error,
            )
        };
        status_result(status, out_error)?;
        unsafe { Self::from_retained_ptr(out_predicate, "comparison predicate") }
    }

    fn compound(compound_type: u64, predicates: &[&Self]) -> Result<Self, CoreDataError> {
        let raw = predicates
            .iter()
            .map(|predicate| predicate.as_ptr())
            .collect::<Vec<_>>();
        let count = i32::try_from(raw.len())
            .map_err(|_| CoreDataError::bridge(-1, "compound predicate count overflow"))?;
        let mut out_predicate = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_predicate_new_compound(
                compound_type,
                raw.as_ptr(),
                count,
                &raw mut out_predicate,
                &raw mut out_error,
            )
        };
        status_result(status, out_error)?;
        unsafe { Self::from_retained_ptr(out_predicate, "compound predicate") }
    }

    pub fn and(predicates: &[&Self]) -> Result<Self, CoreDataError> {
        Self::compound(COMPOUND_AND, predicates)
    }

    pub fn or(predicates: &[&Self]) -> Result<Self, CoreDataError> {
        Self::compound(COMPOUND_OR, predicates)
    }

    pub fn not(predicate: &Self) -> Result<Self, CoreDataError> {
        Self::compound(COMPOUND_NOT, &[predicate])
    }
}

impl NSFetchRequest {
    /// Wraps `NSFetchRequest.init(...)`.
    pub fn new(entity_name: impl AsRef<str>) -> Result<Self, CoreDataError> {
        let entity_name = cstring_from_str(entity_name.as_ref(), "fetch request entity name")?;
        let mut out_request = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_fetch_request_new(
                entity_name.as_ptr(),
                &raw mut out_request,
                &raw mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_request, "fetch request") }
    }

    /// Mirrors `NSFetchRequest.predicate`.
    pub fn set_predicate(&mut self, predicate: Option<&NSPredicate>) {
        unsafe {
            ffi::cd_fetch_request_set_predicate(
                self.as_ptr(),
                predicate.map_or(core::ptr::null_mut(), NSPredicate::as_ptr),
            );
        }
    }

    /// Mirrors `NSFetchRequest.sort_descriptors`.
    pub fn set_sort_descriptors(
        &mut self,
        descriptors: &[SortDescriptor],
    ) -> Result<(), CoreDataError> {
        let descriptors_json = json_cstring(descriptors, "sort descriptors")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cd_fetch_request_set_sort_descriptors_json(
                self.as_ptr(),
                descriptors_json.as_ptr(),
                &raw mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn fetch_limit(&self) -> usize {
        usize::try_from(unsafe { ffi::cd_fetch_request_get_fetch_limit(self.as_ptr()) })
            .unwrap_or(usize::MAX)
    }

    pub fn fetch_offset(&self) -> usize {
        usize::try_from(unsafe { ffi::cd_fetch_request_get_fetch_offset(self.as_ptr()) })
            .unwrap_or(usize::MAX)
    }

    /// Mirrors `NSFetchRequest.fetch_limit`.
    pub fn set_fetch_limit(&mut self, fetch_limit: usize) -> Result<(), CoreDataError> {
        let fetch_limit = u64::try_from(fetch_limit)
            .map_err(|_| CoreDataError::bridge(-1, "fetch limit overflow"))?;
        unsafe {
            ffi::cd_fetch_request_set_fetch_limit(self.as_ptr(), fetch_limit);
        }
        Ok(())
    }

    /// Mirrors `NSFetchRequest.fetch_offset`.
    pub fn set_fetch_offset(&mut self, fetch_offset: usize) -> Result<(), CoreDataError> {
        let fetch_offset = u64::try_from(fetch_offset)
            .map_err(|_| CoreDataError::bridge(-1, "fetch offset overflow"))?;
        unsafe {
            ffi::cd_fetch_request_set_fetch_offset(self.as_ptr(), fetch_offset);
        }
        Ok(())
    }

    /// Wraps `NSFetchRequest.execute(...)`.
    pub fn execute(
        &self,
        context: &NSManagedObjectContext,
    ) -> Result<Vec<NSManagedObject>, CoreDataError> {
        context.fetch(self)
    }
}
