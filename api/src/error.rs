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

pub fn unauthorized_error(message: &str) -> actix_web::Error {
    InternalError::from_response(
        anyhow::anyhow!(message.to_string()),
        HttpResponse::Unauthorized().json(ErrorResponse {
            detail: message.to_string(),
        }),
    )
    .into()
}

pub fn internal_server_error(message: &str) -> actix_web::Error {
    InternalError::from_response(
        anyhow::anyhow!(message.to_string()),
        HttpResponse::InternalServerError().json(ErrorResponse {
            detail: message.to_string(),
        }),
    )
    .into()
}

pub fn bad_request_error(message: &str) -> actix_web::Error {
    InternalError::from_response(
        anyhow::anyhow!(message.to_string()),
        HttpResponse::BadRequest().json(ErrorResponse {
            detail: message.to_string(),
        }),
    )
    .into()
}

pub fn not_found_error(message: &str) -> actix_web::Error {
    InternalError::from_response(
        anyhow::anyhow!(message.to_string()),
        HttpResponse::BadRequest().json(ErrorResponse {
            detail: message.to_string(),
        }),
    )
    .into()
}

pub async fn not_found() -> Result<HttpResponse, actix_web::Error> {
    Err(not_found_error("This route does not exist."))
}

#[derive(thiserror::Error)]
pub enum OptionApiError {
    #[error("{0}")]
    ValidationError(String),
    // NOTE: Do not change this string, because different not found
    // messages can lead to information leakage
    #[error("Resource not found")]
    NotFoundError,
    #[error("{0}")]
    AuthorizationError(String),
    #[error("{0}")]
    MethodNotAllowedError(String),
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
            OptionApiError::MethodNotAllowedError(message) => {
                (StatusCode::METHOD_NOT_ALLOWED, message.clone())
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

#[derive(thiserror::Error)]
pub enum NormalApiError {
    #[error("{0}")]
    ValidationError(String),
    #[error("{0}")]
    AuthorizationError(String),
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

#[derive(thiserror::Error)]
pub enum MinimalApiError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for MinimalApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[derive(thiserror::Error)]
pub enum NotFoundOrUnexpectedApiError {
    // NOTE: Do not change this string, because different not found
    // messages can lead to information leakage
    #[error("Resource not found")]
    NotFoundError,
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

#[derive(thiserror::Error)]
pub enum AuthOnlyError {
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

#[derive(thiserror::Error)]
pub enum NotFoundOnlyError {
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

#[derive(thiserror::Error)]
pub enum MethodNotAllowedOnlyError {
    #[error("{0}")]
    MethodNotAllowedError(String),
}

impl std::fmt::Debug for MethodNotAllowedOnlyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl From<MethodNotAllowedOnlyError> for OptionApiError {
    fn from(value: MethodNotAllowedOnlyError) -> Self {
        match value {
            MethodNotAllowedOnlyError::MethodNotAllowedError(message) => {
                Self::MethodNotAllowedError(message)
            }
        }
    }
}

impl ResponseError for MethodNotAllowedOnlyError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let (status_code, message) = match self {
            MethodNotAllowedOnlyError::MethodNotAllowedError(message) => {
                (StatusCode::METHOD_NOT_ALLOWED, message.clone())
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

#[derive(thiserror::Error)]
pub enum UnexpectedOnlyError {
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
