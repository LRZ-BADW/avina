macro_rules! return_unexpected_error {
    ($($arg:tt)*) => {
        tracing::error!($($arg)*);
        return rsx! { p { b { "Error: " }, "Unexpected error, please contact support." } }
    };
}

macro_rules! api_call {
    ($api_url:expr, $token:expr, $api:ident, $call:expr) => {{
        use std::str::FromStr;

        use avina::{Api, Token, error::ApiError};

        let token = $token;
        let api_url = $api_url;
        match use_resource(move || {
            let token_str = token.clone();
            let url = api_url.clone();
            async move {
                let token = Token::from_str(&token_str)?;
                let $api = Api::new(url, token, None, None)?;
                $call
            }
        })
        .read()
        .as_ref()
        {
            Some(Ok(value)) => value.clone(),
            Some(Err(ApiError::ResponseError(message))) => {
                tracing::warn!("API Error Response: {message}");
                return rsx! { p { b { "Error: " }, "{message}" } };
            }
            Some(Err(ApiError::UnexpectedError(error))) => {
                return_unexpected_error!("Unexpected API Error: {}", error);
            }
            None => {
                return rsx! { p { "Loading ..." } };
            }
        }
    }};
}
