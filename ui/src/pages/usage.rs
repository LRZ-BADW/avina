use dioxus::prelude::*;

#[component]
pub fn UsagePage(api_url: String, token: String) -> Element {
    let usage = api_call!(api_url, token, api, api.usage.get().await);

    rsx! {
        h2 { "Cloud Usage" }
        hr {}

        p {
            class: "text-end",
            "Last updated: {usage.datetime}"
        }
    }
}
