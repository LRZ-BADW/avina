//! Rudimentary health-check endpoint.

use actix_web::HttpResponse;

/// Rudimentary health-check endpoint.
///
/// This simply returns an HTTP 200 OK. This is also the only endpoint, that
/// doesn't require authentication, and is not located under `/api/`.
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that the [health_check] endpoint returns a HTTP 200 OK.
    #[tokio::test]
    async fn health_check_succeeds() {
        let response = health_check().await;
        assert!(response.status().is_success())
    }
}
