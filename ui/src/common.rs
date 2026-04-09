#[macro_export]
macro_rules! api_call {
    ($api_url:expr, $token:expr, $api:ident, $call:expr) => {
        {
            use avina::{Api, Token, error::ApiError};
            use std::str::FromStr;
            match use_resource(move || {
                let token_str = $token.clone();
                let url = $api_url.clone();
                async move {
                    let token = Token::from_str(&token_str)?;
                    let $api = Api::new(url, token, None, None)?;
                    $call
                }
            }).read().as_ref() {
                Some(Ok(value)) => value.clone(),
                Some(Err(ApiError::ResponseError(message))) => {
                    tracing::warn!("API Error Response: {message}");
                    return rsx! { p { b { "Error: " }, "{message}" } };
                }
                Some(Err(ApiError::UnexpectedError(error))) => {
                    tracing::error!("Unexpected API Error: {error}");
                    return rsx! { p { b { "Error: " }, "Unexpected error, please contact support." } };
                }
                None => {
                    return rsx! { p { "Loading ..." } };
                }
            }
        }
    }
}
