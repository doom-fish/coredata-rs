use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Value {
    Null,
    String(String),
    Int32(i32),
    Int64(i64),
    Double(f64),
    Bool(bool),
}

impl Value {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value.as_str()),
            _ => None,
        }
    }

    pub fn as_i32(&self) -> Option<i32> {
        match self {
            Self::Int32(value) => Some(*value),
            Self::Int64(value) => i32::try_from(*value).ok(),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Int32(value) => Some(i64::from(*value)),
            Self::Int64(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Double(value) => Some(*value),
            Self::Int32(value) => Some(f64::from(*value)),
            Self::Int64(value) => Some(*value as f64),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(*value),
            _ => None,
        }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Self::Int32(i32::from(value))
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::Int32(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::Int64(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self::Double(f64::from(value))
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Double(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum ValueKind {
    Null,
    String,
    Int32,
    Int64,
    Double,
    Bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ValuePayload {
    pub kind: ValueKind,
    pub string_value: Option<String>,
    pub int32_value: Option<i32>,
    pub int64_value: Option<i64>,
    pub double_value: Option<f64>,
    pub bool_value: Option<bool>,
}

impl From<Value> for ValuePayload {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => Self {
                kind: ValueKind::Null,
                string_value: None,
                int32_value: None,
                int64_value: None,
                double_value: None,
                bool_value: None,
            },
            Value::String(value) => Self {
                kind: ValueKind::String,
                string_value: Some(value),
                int32_value: None,
                int64_value: None,
                double_value: None,
                bool_value: None,
            },
            Value::Int32(value) => Self {
                kind: ValueKind::Int32,
                string_value: None,
                int32_value: Some(value),
                int64_value: None,
                double_value: None,
                bool_value: None,
            },
            Value::Int64(value) => Self {
                kind: ValueKind::Int64,
                string_value: None,
                int32_value: None,
                int64_value: Some(value),
                double_value: None,
                bool_value: None,
            },
            Value::Double(value) => Self {
                kind: ValueKind::Double,
                string_value: None,
                int32_value: None,
                int64_value: None,
                double_value: Some(value),
                bool_value: None,
            },
            Value::Bool(value) => Self {
                kind: ValueKind::Bool,
                string_value: None,
                int32_value: None,
                int64_value: None,
                double_value: None,
                bool_value: Some(value),
            },
        }
    }
}

impl From<&Value> for ValuePayload {
    fn from(value: &Value) -> Self {
        value.clone().into()
    }
}

impl TryFrom<ValuePayload> for Value {
    type Error = crate::error::CoreDataError;

    fn try_from(value: ValuePayload) -> Result<Self, Self::Error> {
        match value.kind {
            ValueKind::Null => Ok(Self::Null),
            ValueKind::String => value
                .string_value
                .map(Self::String)
                .ok_or_else(|| crate::error::CoreDataError::bridge(-1, "missing string value")),
            ValueKind::Int32 => value
                .int32_value
                .map(Self::Int32)
                .ok_or_else(|| crate::error::CoreDataError::bridge(-1, "missing int32 value")),
            ValueKind::Int64 => value
                .int64_value
                .map(Self::Int64)
                .ok_or_else(|| crate::error::CoreDataError::bridge(-1, "missing int64 value")),
            ValueKind::Double => value
                .double_value
                .map(Self::Double)
                .ok_or_else(|| crate::error::CoreDataError::bridge(-1, "missing double value")),
            ValueKind::Bool => value
                .bool_value
                .map(Self::Bool)
                .ok_or_else(|| crate::error::CoreDataError::bridge(-1, "missing bool value")),
        }
    }
}
