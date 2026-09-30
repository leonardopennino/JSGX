use ciborium::Value;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum RATLSError {
    #[error("Certificate extension not found")]
    QuoteOidNotFound,
    #[error("Unexpected CBOR value, expected : {expected} , got : {found}")]
    InvalidCborEntry { expected: String, found: String },
    #[error("IO Error : {0}")]
    IOError(#[from] ciborium::de::Error<std::io::Error>),
    #[error("{0}")]
    GenericError(String),
    #[error("{0}")]
    InvalidPayloadEntryError(Box<dyn InvalidPayloadEntryTrait>),
    #[error("Token expired")]
    TokenExpired,
    #[error("Enclave attribute {0} mismatch")]
    EnclaveAttributeMismatch(String),
    #[error("Unable to decode payload {0}")]
    Base64DecodeError(String),
    #[error("Unable to decode json payload {0}")]
    JSONDecodeError(#[from] serde_json::Error),
    #[error("Invalid X509 certificate provided, make sure it is PEM encoded")]
    InvalidCertificateProvided
}
pub trait InvalidPayloadEntryTrait: std::fmt::Display + std::fmt::Debug {}
#[derive(Debug)]
pub(crate) struct InvalidPayloadEntry<T> {
    expected: Box<T>,
    found: Box<T>,
    field_name: String,
}
impl<T> InvalidPayloadEntry<T> {
    pub(crate) fn new(expected: Box<T>, found: Box<T>, field_name: &str) -> InvalidPayloadEntry<T> {
        Self {
            expected,
            found,
            field_name: field_name.to_string(),
        }
    }
}
impl<T> InvalidPayloadEntryTrait for InvalidPayloadEntry<T> where
    T: std::fmt::Display + ToString + std::fmt::Debug
{
}
impl<T> std::fmt::Display for InvalidPayloadEntry<T>
where
    T: std::fmt::Debug + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Invalid entry {}, expected : {}, found : {}",
            self.field_name, self.expected, self.found
        ))
    }
}

impl<'a> RATLSError {
    pub fn from_cbor_value_err(value: &ciborium::Value, expected: &ciborium::Value) -> Self {
        RATLSError::InvalidCborEntry {
            expected: Self::cbor_to_string(expected),
            found: Self::cbor_to_string(value),
        }
    }
    fn cbor_to_string(value: &Value) -> String {
        match value {
            Value::Tag(_, _) => "TAG".to_string(),
            Value::Map(_) => "MAP".to_string(),
            Value::Text(_) => "TEXT".to_string(),
            Value::Bool(_) => "BOOL".to_string(),
            Value::Null => "NULL".to_string(),
            Value::Float(_) => "FLOAT".to_string(),
            Value::Integer(_) => "INTEGER".to_string(),
            Value::Array(_) => "ARRAY".to_string(),
            Value::Bytes(_) => "BYTES".to_string(),
            _ => "UNKNOWN".to_string(),
        }
    }
}

pub type AttestationResult<T> = Result<T, RATLSError>;
