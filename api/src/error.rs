//! Errors the endpoints may return and useful functions and trait implementations.
//!
//! At the basis are the seven error enums, for example [NormalApiError]. While some have multiple
//! variants, others only convey a single type of error. [From] implementations turn each more
//! specific enum into less specific ones, for example, an [NotFoundOnlyError] can be turned into
//! an [NotFoundOrUnexpectedApiError].
//!
//! For special cases, where an [actix_web::Error] needs to be returned, there are also
//! a few helper functions for various kinds of HTTP errors.

use actix_web::{
    HttpResponse, ResponseError,
    body::BoxBody,
    error::InternalError,
    http::{
        StatusCode,
        header::{CONTENT_TYPE, HeaderValue},
    },
};
use avina_wire::error::{ErrorResponse, error_chain_fmt};

/// Wrap given message in an HTTP Unauthorized Error.
pub fn unauthorized_error(message: &str) -> actix_web::Error {
    InternalError::from_response(
        anyhow::anyhow!(message.to_string()),
        HttpResponse::Unauthorized().json(ErrorResponse {
            detail: message.to_string(),
        }),
    )
    .into()
}

/// Wrap the given message in an HTTP Internal Server Error.
pub fn internal_server_error(message: &str) -> actix_web::Error {
    InternalError::from_response(
        anyhow::anyhow!(message.to_string()),
        HttpResponse::InternalServerError().json(ErrorResponse {
            detail: message.to_string(),
        }),
    )
    .into()
}

/// Wrap the given message in an HTTP Bad Request Error.
pub fn bad_request_error(message: &str) -> actix_web::Error {
    InternalError::from_response(
        anyhow::anyhow!(message.to_string()),
        HttpResponse::BadRequest().json(ErrorResponse {
            detail: message.to_string(),
        }),
    )
    .into()
}

/// Wrap the given message in an HTTP Not Found Error.
pub fn not_found_error(message: &str) -> actix_web::Error {
    InternalError::from_response(
        anyhow::anyhow!(message.to_string()),
        HttpResponse::BadRequest().json(ErrorResponse {
            detail: message.to_string(),
        }),
    )
    .into()
}

/// Return an HTTP Not Found Error signalling a non-existent route.
pub async fn not_found() -> Result<HttpResponse, actix_web::Error> {
    Err(not_found_error("This route does not exist."))
}

/// Like [NormalApiError] but with an additional not-found variant.
#[derive(thiserror::Error)]
pub enum OptionApiError {
    /// Validation of the user input failed. The error message is contained within.
    #[error("{0}")]
    ValidationError(String),
    /// The requested resource was not found.
    // NOTE: Do not change this string, because different not found
    // messages can lead to information leakage
    #[error("Resource not found")]
    NotFoundError,
    /// The user is unauthorized to perform the request. The error message is contained within.
    #[error("{0}")]
    AuthorizationError(String),
    /// An unexpected error occurred, which is contained inside.
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for OptionApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for OptionApiError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let (status_code, message) = match self {
            OptionApiError::ValidationError(message) => {
                (StatusCode::BAD_REQUEST, message.clone())
            }
            OptionApiError::NotFoundError => {
                (StatusCode::NOT_FOUND, self.to_string())
            }
            OptionApiError::AuthorizationError(message) => {
                (StatusCode::FORBIDDEN, message.clone())
            }
            OptionApiError::UnexpectedError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error, contact admin or check logs"
                    .to_string(),
            ),
        };
        HttpResponse::build(status_code)
            .insert_header((
                CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            ))
            // TODO: handle unwrap
            .body(
                serde_json::to_string(&ErrorResponse { detail: message })
                    .unwrap(),
            )
    }
}

impl From<NormalApiError> for OptionApiError {
    fn from(value: NormalApiError) -> Self {
        match value {
            NormalApiError::ValidationError(message) => {
                Self::ValidationError(message)
            }
            NormalApiError::AuthorizationError(message) => {
                Self::AuthorizationError(message)
            }
            NormalApiError::UnexpectedError(error) => {
                Self::UnexpectedError(error)
            }
        }
    }
}

impl From<MinimalApiError> for OptionApiError {
    fn from(value: MinimalApiError) -> Self {
        match value {
            MinimalApiError::ValidationError(message) => {
                Self::ValidationError(message)
            }
            MinimalApiError::UnexpectedError(error) => {
                Self::UnexpectedError(error)
            }
        }
    }
}

impl From<NotFoundOrUnexpectedApiError> for OptionApiError {
    fn from(value: NotFoundOrUnexpectedApiError) -> Self {
        match value {
            NotFoundOrUnexpectedApiError::NotFoundError => Self::NotFoundError,
            NotFoundOrUnexpectedApiError::UnexpectedError(error) => {
                Self::UnexpectedError(error)
            }
        }
    }
}

impl From<UnexpectedOnlyError> for OptionApiError {
    fn from(value: UnexpectedOnlyError) -> Self {
        match value {
            UnexpectedOnlyError::UnexpectedError(message) => {
                Self::UnexpectedError(message)
            }
        }
    }
}

impl From<UnexpectedOnlyError> for NotFoundOrUnexpectedApiError {
    fn from(value: UnexpectedOnlyError) -> Self {
        match value {
            UnexpectedOnlyError::UnexpectedError(message) => {
                Self::UnexpectedError(message)
            }
        }
    }
}

impl From<AuthOnlyError> for OptionApiError {
    fn from(value: AuthOnlyError) -> Self {
        match value {
            AuthOnlyError::AuthorizationError(message) => {
                Self::AuthorizationError(message)
            }
        }
    }
}

impl From<UnexpectedOnlyError> for MinimalApiError {
    fn from(value: UnexpectedOnlyError) -> Self {
        match value {
            UnexpectedOnlyError::UnexpectedError(message) => {
                Self::UnexpectedError(message)
            }
        }
    }
}

/// Typical error response many API endpoints return.
///
/// Either the validation of the user input failed, the user is not authorized
/// for the specified action, or something unexpected happened.
#[derive(thiserror::Error)]
pub enum NormalApiError {
    /// Validation of the user input failed. The error message is contained within.
    #[error("{0}")]
    ValidationError(String),
    /// The user is unauthorized to perform the request. The error message is contained within.
    #[error("{0}")]
    AuthorizationError(String),
    /// An unexpected error occurred, which is contained inside.
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for NormalApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for NormalApiError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let (status_code, message) = match self {
            NormalApiError::ValidationError(message) => {
                (StatusCode::BAD_REQUEST, message.clone())
            }
            NormalApiError::AuthorizationError(message) => {
                (StatusCode::FORBIDDEN, message.clone())
            }
            NormalApiError::UnexpectedError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error, contact admin or check logs"
                    .to_string(),
            ),
        };
        HttpResponse::build(status_code)
            .insert_header((
                CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            ))
            // TODO: handle unwrap
            .body(
                serde_json::to_string(&ErrorResponse { detail: message })
                    .unwrap(),
            )
    }
}

impl From<MinimalApiError> for NormalApiError {
    fn from(value: MinimalApiError) -> Self {
        match value {
            MinimalApiError::ValidationError(message) => {
                Self::ValidationError(message)
            }
            MinimalApiError::UnexpectedError(error) => {
                Self::UnexpectedError(error)
            }
        }
    }
}

/// Like [NormalApiError] but without unauthorized variant.
#[derive(thiserror::Error)]
pub enum MinimalApiError {
    /// Validation of the user input failed. The error message is contained within.
    #[error("{0}")]
    ValidationError(String),
    /// An unexpected error occurred, which is contained inside.
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for MinimalApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

/// The request can fail either when the resource does not exist, or unexpectedly.
#[derive(thiserror::Error)]
pub enum NotFoundOrUnexpectedApiError {
    // NOTE: Do not change this string, because different not found
    // messages can lead to information leakage
    #[error("Resource not found")]
    NotFoundError,
    /// An unexpected error occurred, which is contained inside.
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for NotFoundOrUnexpectedApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for NotFoundOrUnexpectedApiError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let (status_code, message) = match self {
            NotFoundOrUnexpectedApiError::NotFoundError => {
                (StatusCode::NOT_FOUND, self.to_string())
            }
            NotFoundOrUnexpectedApiError::UnexpectedError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error, contact admin or check logs"
                    .to_string(),
            ),
        };
        HttpResponse::build(status_code)
            .insert_header((
                CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            ))
            // TODO: handle unwrap
            .body(
                serde_json::to_string(&ErrorResponse { detail: message })
                    .unwrap(),
            )
    }
}

/// The request can only fail due to an authorization error.
#[derive(thiserror::Error)]
pub enum AuthOnlyError {
    /// The user is unauthorized to perform the request. The error message is contained within.
    #[error("{0}")]
    AuthorizationError(String),
}

impl std::fmt::Debug for AuthOnlyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for AuthOnlyError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let (status_code, message) = match self {
            AuthOnlyError::AuthorizationError(message) => {
                (StatusCode::FORBIDDEN, message.clone())
            }
        };
        HttpResponse::build(status_code)
            .insert_header((
                CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            ))
            // TODO: handle unwrap
            .body(
                serde_json::to_string(&ErrorResponse { detail: message })
                    .unwrap(),
            )
    }
}

impl From<AuthOnlyError> for NormalApiError {
    fn from(value: AuthOnlyError) -> Self {
        match value {
            AuthOnlyError::AuthorizationError(message) => {
                Self::AuthorizationError(message)
            }
        }
    }
}

/// The request can only fail when the resource does not exist.
#[derive(thiserror::Error)]
pub enum NotFoundOnlyError {
    /// The requested resource was not found.
    // NOTE: Do not change this string, because different not found messages
    // messages can lead to information leakage
    #[error("Resource not found")]
    NotFoundError,
}

impl std::fmt::Debug for NotFoundOnlyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for NotFoundOnlyError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let (status_code, message) = match self {
            NotFoundOnlyError::NotFoundError => {
                (StatusCode::NOT_FOUND, self.to_string())
            }
        };
        HttpResponse::build(status_code)
            .insert_header((
                CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            ))
            // TODO: handle unwrap
            .body(
                serde_json::to_string(&ErrorResponse { detail: message })
                    .unwrap(),
            )
    }
}

impl From<NotFoundOnlyError> for OptionApiError {
    fn from(value: NotFoundOnlyError) -> Self {
        match value {
            NotFoundOnlyError::NotFoundError => Self::NotFoundError,
        }
    }
}

/// The request can only fail unexpectedly.
#[derive(thiserror::Error)]
pub enum UnexpectedOnlyError {
    /// An unexpected error occurred, which is contained inside.
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for UnexpectedOnlyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for UnexpectedOnlyError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let (status_code, message) = match self {
            UnexpectedOnlyError::UnexpectedError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error, contact admin or check logs"
                    .to_string(),
            ),
        };
        HttpResponse::build(status_code)
            .insert_header((
                CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            ))
            // TODO: handle unwrap
            .body(
                serde_json::to_string(&ErrorResponse { detail: message })
                    .unwrap(),
            )
    }
}

impl From<UnexpectedOnlyError> for NormalApiError {
    fn from(value: UnexpectedOnlyError) -> Self {
        match value {
            UnexpectedOnlyError::UnexpectedError(message) => {
                Self::UnexpectedError(message)
            }
        }
    }
}
