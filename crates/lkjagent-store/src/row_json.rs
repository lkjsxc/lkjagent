use crate::error::{StoreError, StoreResult};

pub(crate) fn json_string<T>(value: &T) -> StoreResult<String>
where
    T: serde::Serialize,
{
    serde_json::to_string(value).map_err(json_error)
}

pub(crate) fn json_value<T>(text: &str) -> StoreResult<T>
where
    T: serde::de::DeserializeOwned,
{
    serde_json::from_str(text).map_err(json_error)
}

pub(crate) fn json_error(error: serde_json::Error) -> StoreError {
    StoreError::InvalidState(error.to_string())
}

pub(crate) fn fingerprint_error(
    error: lkjagent_core::runtime_fingerprint::FingerprintError,
) -> StoreError {
    StoreError::InvalidState(error.message)
}
