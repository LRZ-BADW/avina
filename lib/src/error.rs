//! Helper types and functions for error handling.

use std::fmt::Debug;

use avina_wire::error::error_chain_fmt;

/// Error returned by the API bindings.
///
/// This can either be an error the API backend responded with, or an unexpected error on the client
/// side.
#[derive(thiserror::Error)]
pub enum ApiError {
    /// An error occurred on the API backend side, the message is contained within.
    #[error("{0}")]
    ResponseError(String),
    /// An unexpected error occurred on the client side, it is wrapped in an [anyhow::Error].
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl Debug for ApiError {
    /// Show the error chain with the given formatter.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
