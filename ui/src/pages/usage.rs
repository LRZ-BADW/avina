use std::cmp::max;

use avina_wire::resources::{CloudUsageAggregate, CloudUsageFlavorSlot};
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

        div {
            class: "row",
            h3 { "Public Resources Overview" }
            PieChart {
                name: "vCPU",
                used: usage.overview.vcpus.used,
                total: usage.overview.vcpus.total,
                unit: "",
            }
            PieChart {
                name: "RAM",
                used: usage.overview.ram.used / 1_000_000,
                total: usage.overview.ram.total / 1_000_000,
                unit: "TB",
            }
            PieChart {
                name: "GPU",
                used: usage.overview.gpus.used,
                total: usage.overview.gpus.total,
                unit: "",
            }
            PieChart {
                name: "Storage",
                used: (usage.overview.storage.used / 1024.) as u64,
                total: (usage.overview.storage.total / 1024.) as u64,
                unit: "TiB",
            }
            PieChart {
                name: "MWN Floating IP",
                used: usage.overview.mwn_ips.used,
                total: usage.overview.mwn_ips.total,
                unit: "",
            }
            PieChart {
                name: "Internet Floating IP",
                used: usage.overview.www_ips.used,
                total: usage.overview.www_ips.total,
                unit: "",
            }
        }

        br {}
        br {}

        FlavorSlotRow { title: "LRZ Flavor Slots", aggregates: usage.lrz_flavor_slots }

        br {}
        br {}

        FlavorSlotRow { title: "ACH Flavor Slots", aggregates: usage.ach_flavor_slots }

        br {}
        br {}

        FlavorSlotRow { title: "Other Flavor Slots", aggregates: usage.other_flavor_slots }
    }
}

#[component]
fn PieChart(name: String, used: u64, total: u64, unit: String) -> Element {
    let ratio = used as f64 / total as f64;
    let percentage = (ratio * 100.).round() as u32;
    let color = match ratio {
        _ if ratio < 0.75 => "rgba(109, 173, 223, 1.0)",
        _ if ratio < 0.9 => "rgba(240, 173, 78, 1.0)",
        _ => "rgba(240, 78, 78, 1.0)",
    }
    .to_string();
    rsx! {
        div {
            class: "col-lg-2",
            class: "col-md-4",
            class: "col-sm-4",
            class: "col-xs-6",
            div {
                table {
                    width: "100px",
                    class: "charts-css",
                    class: "pie",
                    tbody {
                        tr {
                            td {
                                style: "--start: 0.0; --end: {ratio}; --color: {color};",
                            }
                            td {
                                style: "--start: {ratio}; --end: 1.0; --color: rgba(0, 0, 0, 0.0);",
                            }
                        }
                    }
                }
            }
            br {}
            h5 {
                class: "text-center",
                "{name} Usage: {percentage}%"
            }
            h6 {
                class: "text-center",
                "Used {used}{unit} of {total}{unit}"
            }
        }
    }
}

#[component]
fn FlavorSlotRow(
    title: String,
    aggregates: Vec<CloudUsageAggregate>,
) -> Element {
    rsx! {
        div {
            class: "row",
            h3 { "{title}" }
            br {}
            for aggregate in aggregates {
                div {
                    class: "col-lg-4",
                    class: "col-md-4",
                    class: "col-sm-6",
                    class: "col-xs-12",
                    h4 { "{aggregate.title}" },
                    FlavorSlotTable { slots: aggregate.flavors }
                }
            }
        }
    }
}

#[component]
fn FlavorSlotTable(slots: Vec<CloudUsageFlavorSlot>) -> Element {
    rsx! {
        div {
            class: "table_wrapper",
            table {
                class: "table",
                style: "--bs-table-bg: #eeeeee;",
                thead {
                    tr {
                        th { "Flavor" },
                        th { "Slots (available)" },
                        th { "Slots (total)"},
                    }
                }
                tbody {
                    for slot in slots {
                        FlavorSlotTableRow { slot }
                    }
                }
            }
        }
    }
}

#[component]
fn FlavorSlotTableRow(slot: CloudUsageFlavorSlot) -> Element {
    if slot.free < 0 {
        tracing::warn!(
            "Negative flavor slot free value for flavor {}.",
            slot.name
        );
    }
    let free = max(0, slot.free);
    let color = if free > 0 { "#ccffcc" } else { "#ffcccc" };
    rsx! {
        tr {
            style: "--bs-table-bg: {color}",
            td { "{slot.name}" },
            td { "{free}" },
            td { "{slot.total}" },
        }
    }
}
