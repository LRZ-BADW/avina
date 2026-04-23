use avina_wire::user::UserDetailed;
use dioxus::prelude::*;

#[component]
pub fn BudgetProjectSubPage(
    api_url: String,
    token: String,
    user: UserDetailed,
) -> Element {
    rsx! {
        p { "TODO" }
    }
}
