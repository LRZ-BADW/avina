//! Common helper types and functions.

use std::fmt::Debug;

use anyhow::Context;
use avina_wire::error::ErrorResponse;
use reqwest::{Client, Method, Response, StatusCode};
use serde::{de::DeserializeOwned, ser::Serialize};

use crate::error::ApiError;

/// Zero-size marker type that implements [serde::Serialize].
///
/// This is use by the [SerializableNone] macro to pass a [None] as parameter that only requires
/// a generic implementing [serde::Serialize] without needing type annotations.
#[derive(serde::Serialize, Debug)]
pub(crate) struct SerializableFoo;

/// Returns a [None] with an annotation of a serializable marker type.
///
/// This can be passed as argument for generic function parameters, those requirement is
/// that they implement [serde::Serialize], without having to annotate a type. This is
/// helpful for the `data` parameter of the [request_bare] and [request] functions.
macro_rules! SerializableNone {
    () => {
        None::<crate::common::SerializableFoo>
    };
}
pub(crate) use SerializableNone;

/// Perform an HTTP request and return the bare response.
///
/// This function is generic over the request data, those only requirement is,
/// that is implements [Serialize] and [Debug]. It sends the data with the
/// given HTTP method to the given URL, using the given client. It matches
/// the returned response status code against the given one and otherwise
/// returns an [ApiError], but otherwise just returns the received [Response]
///
/// When the API returned an error message, this is wrapped in an
/// [ApiError::ResponseError], otherwise or if something on the client side
/// went wrong, like serializing request data or sending the request, the
/// [ApiError::UnexpectedError] is returned.
///
/// # Arguments
///
///   - `client` - HTTP client.
///   - `method` - Request method to use.
///   - `url` - URL to call.
///   - `data` - Generic serializable request data to send.
///   - `expected_status` - Response status code to expect.
pub(crate) async fn request_bare<T>(
    client: &Client,
    method: Method,
    url: &str,
    data: Option<T>,
    expected_status: StatusCode,
) -> Result<Response, ApiError>
where
    T: Serialize + Debug,
{
    let mut request = client.request(method, url);
    if let Some(data) = data {
        request = request.body(serde_json::to_string(&data).context(
            format!("Could not serialize json request body from {data:?}"),
        )?);
    }
    let response = match request.send().await.context("") {
        Ok(response) => response,
        Err(err) => {
            let detail =
                format!("Could not complete request: {}", err.root_cause());
            return Err(ApiError::ResponseError(detail));
        }
    };
    let status = response.status();
    if status != expected_status {
        let text = response.text().await.context(format!(
            "Could not retrieve response text on unexpected status code {status}.",
        ))?;
        let err_resp: ErrorResponse = serde_json::from_str(text.as_str())
            .context(format!(
                "Unexpected status code {status} without error message.",
            ))?;
        return Err(ApiError::ResponseError(err_resp.detail));
    }
    Ok(response)
}

/// Perform an HTTP request and deserialize the response data.
///
/// This calls [request_bare] and attempts to deserialize the response data
/// into the second generic type, that needs to implement [DeserializeOwned].
/// If this fails it returns an [ApiError::UnexpectedError].
///
/// # Arguments
///
///   - `client` - HTTP client.
///   - `method` - Request method to use.
///   - `url` - URL to call.
///   - `data` - Generic serializable request data to send.
///   - `expected_status` - Response status code to expect.
pub(crate) async fn request<T, U>(
    client: &Client,
    method: Method,
    url: &str,
    data: Option<T>,
    expected_status: StatusCode,
) -> Result<U, ApiError>
where
    T: Serialize + Debug,
    U: DeserializeOwned,
{
    let response =
        request_bare(client, method, url, data, expected_status).await?;
    let text = response
        .text()
        .await
        .context("Could not retrieve response text.")?;
    let u: U = serde_json::from_str(text.as_str())
        .context(format!("Could not parse response text: {text}"))?;
    Ok(u)
}
