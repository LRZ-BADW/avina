use avina_wire::user::UserClass;
use dioxus::prelude::*;

#[component]
pub fn PricesPage(api_url: String, token: String) -> Element {
    let user =
        api_call!(api_url.clone(), token.clone(), api, api.user.me().await);
    let prices = if user.is_staff {
        api_call!(api_url, token, api, api.flavor_price.list().send().await)
    } else if user.project.user_class != UserClass::NA {
        api_call!(
            api_url,
            token,
            api,
            api.flavor_price
                .list()
                .user_class(user.project.user_class)
                .send()
                .await
        )
    } else {
        return_unexpected_error!(
            "Project {} has no user class.",
            user.project.name
        );
    };
    rsx! {
        h1 { "Flavor Prices" }
        div {
            table {
                thead {
                    tr {
                        th { "ID" },
                        th { "Flavor" },
                        th { "User Class"},
                        th { "Start Time"},
                        th { "Price [EUR / year]"}
                    }
                }
                tbody {
                    for price in prices {
                        tr {
                            td { "{price.id}" },
                            td { "{price.flavor_name}" },
                            td { "{price.user_class}" },
                            td { "{price.start_time}" },
                            td { "{price.unit_price}" }
                        }
                    }
                }
            }
        }
    }
}
