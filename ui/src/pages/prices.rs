use std::collections::HashMap;

use avina_wire::{pricing::FlavorPrice, user::UserClass};
use chrono::Utc;
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

    let mut map: HashMap<u32, HashMap<UserClass, Vec<FlavorPrice>>> =
        HashMap::new();
    for price in prices {
        map.entry(price.flavor)
            .or_default()
            .entry(price.user_class)
            .or_default()
            .push(price);
    }

    let now = Utc::now();

    let mut current = Vec::new();
    let mut future = Vec::new();
    let mut past = Vec::new();
    for (_flavor_id, uc_map) in map {
        for (_user_class, prices) in uc_map {
            let mut latest = None;
            for price in prices {
                if price.start_time > now {
                    future.push(price);
                    continue;
                }
                if latest.is_none() {
                    latest = Some(price);
                    continue;
                }
                if price.start_time > latest.clone().unwrap().start_time {
                    past.push(latest.unwrap());
                    latest = Some(price);
                    continue;
                }
                past.push(price);
            }
            if let Some(latest) = latest {
                current.push(latest);
            }
        }
    }

    rsx! {
        h2 { "Flavor Prices" }
        hr {}

        h3 { "Current Prices" }
        PriceTable { prices: current }
        br {}

        h3 { "Future Prices" }
        PriceTable { prices: future }
        br {}

        h3 { "Past Prices" }
        PriceTable { prices: past }
        br {}
    }
}

#[component]
fn PriceTable(prices: Vec<FlavorPrice>) -> Element {
    rsx! {
        div {
            class: "table_wrapper",
            table {
                class: "table",
                class: "table-striped",
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
