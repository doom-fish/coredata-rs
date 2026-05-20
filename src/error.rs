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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bridge_sets_expected_domain_and_fields() {
        let error = CoreDataError::bridge(-7, "bridge failed");

        assert_eq!(error.domain, COREDATA_BRIDGE_ERROR_DOMAIN);
        assert_eq!(error.code, -7);
        assert_eq!(error.message, "bridge failed");
    }

    #[test]
    fn from_payload_preserves_fields() {
        let error = CoreDataError::from_payload(ErrorPayload {
            domain: "NSCocoaErrorDomain".into(),
            code: 133_000,
            message: "missing model".into(),
        });

        assert_eq!(error.domain, "NSCocoaErrorDomain");
        assert_eq!(error.code, 133_000);
        assert_eq!(error.message, "missing model");
    }

    #[test]
    fn display_formats_message_code_and_domain() {
        let error = CoreDataError {
            domain: "NSCocoaErrorDomain".into(),
            code: 42,
            message: "boom".into(),
        };

        assert_eq!(error.to_string(), "boom (42) [NSCocoaErrorDomain]");
    }

    #[test]
    fn error_payload_round_trips_through_serde() {
        let payload = ErrorPayload {
            domain: COREDATA_BRIDGE_ERROR_DOMAIN.into(),
            code: -1,
            message: "decode failed".into(),
        };
        let json = serde_json::to_string(&payload).expect("serialize error payload");
        let decoded: ErrorPayload = serde_json::from_str(&json).expect("deserialize error payload");

        assert_eq!(decoded.domain, payload.domain);
        assert_eq!(decoded.code, payload.code);
        assert_eq!(decoded.message, payload.message);
    }
}
