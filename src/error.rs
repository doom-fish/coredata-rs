use core::fmt;

use serde::{Deserialize, Serialize};

/// Mirrors `CoreDataBridge`.
pub const COREDATA_BRIDGE_ERROR_DOMAIN: &str = "CoreDataBridge";

#[derive(Debug, Clone, PartialEq, Eq)]
/// Wraps `CoreDataError`.
pub struct CoreDataError {
    /// Mirrors `CoreDataError.domain`.
    pub domain: String,
    /// Mirrors `CoreDataError.code`.
    pub code: i64,
    /// Mirrors `CoreDataError.message`.
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ErrorPayload {
    pub domain: String,
    pub code: i64,
    pub message: String,
}

impl CoreDataError {
    pub(crate) fn from_payload(payload: ErrorPayload) -> Self {
        Self {
            domain: payload.domain,
            code: payload.code,
            message: payload.message,
        }
    }

    pub(crate) fn bridge(code: i64, message: impl Into<String>) -> Self {
        Self {
            domain: COREDATA_BRIDGE_ERROR_DOMAIN.into(),
            code,
            message: message.into(),
        }
    }
}

impl fmt::Display for CoreDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}) [{}]", self.message, self.code, self.domain)
    }
}

impl std::error::Error for CoreDataError {}
