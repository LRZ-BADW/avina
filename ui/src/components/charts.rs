use std::{cmp::Ordering, collections::HashMap};

use dioxus::prelude::*;

#[component]
pub fn UsagePieChart(
    name: String,
    used: u64,
    total: u64,
    unit: String,
    size: u32,
) -> Element {
    let ratio = if used <= total {
        used as f64 / total as f64
    } else {
        1.
    };
    let percentage = (ratio * 100.).round() as u32;
    let color = match ratio {
        _ if ratio < 0.75 => "rgba(109, 173, 223, 1.0)",
        _ if ratio < 0.9 => "rgba(240, 173, 78, 1.0)",
        _ => "rgba(240, 78, 78, 1.0)",
    }
    .to_string();
    rsx! {
        div {
            table {
                width: "{size}px",
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

#[component]
pub fn BarChart(
    data: HashMap<String, f64>,
    show_zero: Option<bool>,
    caption: Option<String>,
    label_size: Option<usize>,
) -> Element {
    let max = data
        .values()
        .cloned()
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .unwrap_or(1.);
    rsx! {
        div {
            table {
                class: "charts-css",
                class: "bar",
                class: "show-labels",
                class: "show-heading",
                style: if let Some(label_size) = label_size {
                    "--labels-size: {label_size}px"
                } else { "" },
                tbody {
                    for (key, value) in data {
                        if value >= 0.05 || (show_zero.is_some() && show_zero.unwrap()) {
                            tr {
                                th {
                                    scope: "row",
                                    "{key}"
                                }
                                td {
                                    style: "--size: {value / max}",
                                    if value >= 0.05 { "{value:.1}" } else { "" }
                                }
                            }
                        }
                    }
                }
                if let Some(caption) = caption {
                    caption {
                        class: "text-center",
                        "{caption}",
                    }
                }
            }
        }
    }
}
