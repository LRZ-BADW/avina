//! Various utility functions.

use chrono::{DateTime, TimeZone, Utc};

/// Wrap the given displayable type inside a bad request error.
pub fn e400<T>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorBadRequest(e)
}

/// Wrap the given displayable type inside an internal server error.
pub fn e500<T>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorInternalServerError(e)
}

/// Get the start of the given year as datetime.
pub fn start_of_the_year(year: u32) -> DateTime<Utc> {
    // TODO: handle this unwrap
    Utc.with_ymd_and_hms(year as i32, 1, 1, 1, 0, 0).unwrap()
}
