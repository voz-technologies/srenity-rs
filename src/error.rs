use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
use wrapi::http::StatusCode;

use wrapi::error::Error as WrapiError;

#[derive(Debug, Clone)]
pub enum Error {
    /// Status: `400`
    BadRequest(Body),
    /// Status `401`
    Unauthorized(Body),
    /// Status `403`
    Forbidden(Body),
    /// Status `404`
    NotFound(Body),
    /// Status `500`
    InternalError(Body),
    /// Unknown/unsupported
    Unknown(String),
    ClientError(String),
    ClientDecodeError(String),
}

/// Error response details
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    pub message: String,
}

impl Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::BadRequest(body)
            | Error::Unauthorized(body)
            | Error::Forbidden(body)
            | Error::NotFound(body)
            | Error::InternalError(body) => write!(f, "{}", body.message),
            Error::Unknown(reason) => write!(f, "Unknown: {}", reason),
            Error::ClientError(reason) => write!(f, "Client: {reason}"),
            Error::ClientDecodeError(reason) => write!(f, "Client decode error: {}", reason),
        }
    }
}

impl From<WrapiError> for Error {
    fn from(value: WrapiError) -> Self {
        match value {
            WrapiError::ResponseError((status, body)) => {
                let Some(body) = body else {
                    return Error::Unknown(format!("No body for status: {}", status.as_u16()));
                };

                let Ok(body) = serde_json::from_value::<Body>(body) else {
                    return Error::ClientDecodeError("Could not deserialize body".into());
                };

                match status {
                    StatusCode::BAD_REQUEST => Error::BadRequest(body),
                    StatusCode::UNAUTHORIZED => Error::Unauthorized(body),
                    StatusCode::FORBIDDEN => Error::Forbidden(body),
                    StatusCode::NOT_FOUND => Error::NotFound(body),
                    StatusCode::INTERNAL_SERVER_ERROR => Error::InternalError(body),
                    _ => Error::Unknown(format!("{}: {body}", status.as_u16())),
                }
            }
            WrapiError::ClientError => Error::ClientError(format!("{value}")),
            WrapiError::ClientDecodeError(x) => Error::ClientDecodeError(x),
        }
    }
}
