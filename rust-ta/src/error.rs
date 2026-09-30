use axum::response::IntoResponse;
use base64::DecodeError;
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum StartupError {
    #[error("The config file could not be found")]
    ConfigNotFound,
    #[error("Unable to locate TA jar")]
    JarNotFoundError,
    #[error("Unable to start JVM, reason : {0}")]
    JavaVmInitError(String),
    #[error("ConfigParseException")]
    ConfigParseException(#[from] toml::de::Error),
    #[error("Base64 decoding exception")]
    Base64DecondingException(#[from] DecodeError),
    #[error("Unknown error, reason : {0}")]
    UnknownError(String)
}

#[derive(Debug)]
pub enum TeeError {
    NoMethodSpecified,
    UDSFailure(String),
}
impl IntoResponse for TeeError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::NoMethodSpecified => {
                (StatusCode::NOT_FOUND, "No method supplied").into_response()
            }
            Self::UDSFailure(s) => (
                StatusCode::BAD_REQUEST,
                format!("Failure communicating with Java vm : {s:?}"),
            )
                .into_response(),
        }
    }
}

impl From<std::io::Error> for TeeError {
    fn from(value: std::io::Error) -> Self {
        TeeError::UDSFailure(value.to_string())
    }
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("The hash could not be verified")]
    HASHVERIFICATIONFAILED,
    #[error("Error while decoding value {0}")]
    BASE64DECODINGERROR(String),
    #[error("Error while deserializing value {}", _0.to_string())]
    JSONDECODEERROR(#[from] serde_json::Error),
    #[error("No user found with the provided public key")]
    USERNOTFOUNDERROR,
    #[error("The provided MR_ENCLAVE is not valid")]
    INVALIDMRENCLAVE,
    #[error("The provided message is expired")]
    INVALIDTIMESTAMP,
    #[error("Invalid jwt token")]
    INVALIDJWT,
    #[error("Jwt not provided")]
    MISSINGJWT,
    #[error("Jwt expired")]
    JWTEXPIRED
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::JWTEXPIRED | Self::MISSINGJWT => (StatusCode::UNAUTHORIZED, self.to_string()).into_response(),
            _ => (StatusCode::BAD_REQUEST, self.to_string()).into_response()

        }

    }
}
impl From<ring::error::Unspecified> for AuthError {
    fn from(_: ring::error::Unspecified) -> Self {
        Self::HASHVERIFICATIONFAILED
    }
}

impl From<DecodeError> for AuthError {
    fn from(value: DecodeError) -> Self {
        Self::BASE64DECODINGERROR(value.to_string())
    }
}

pub type AuthResult<T> = Result<T, AuthError>;
pub type TeeResult<T,E> = Result<T,E>;
