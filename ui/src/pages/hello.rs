use dioxus::prelude::*;

#[component]
pub fn HelloPage(api_url: String, token: String) -> Element {
    let hello = api_call!(api_url, token, api, api.hello.user().await);
    rsx! {
        p { "{hello}" }
    }
}
