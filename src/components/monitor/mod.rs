use super::icons::Spinner;
use super::line_badges::*;
use crate::data::dataset::MonitorData;
use dioxus::prelude::*;

#[component]
pub fn Monitor(monitor_data: Signal<MonitorData>, monitor_loading: Signal<bool>) -> Element {
    if *monitor_loading.read() {
        return rsx! {
            div { class: "monitor-loading-spinner", Spinner {} }
        };
    }

    let mut data_elements = vec![];
    let mut dark = true;
    let md = monitor_data.read();
    for monitor_data_set in md.data.iter() {
        let line_name = monitor_data_set.line_name.clone();
        let destination = monitor_data_set.destination.clone();
        let countdowns = monitor_data_set.countdown.clone();

        if countdowns.len() != 0 {
            if dark {
                dark = false;
                data_elements.push(rsx! {
                    DataElement {
                        line_name,
                        destination,
                        countdowns,
                        bgcolor_class: "bg-red-950",
                    }
                });
            } else {
                dark = true;
                data_elements.push(rsx! {
                    DataElement {
                        line_name,
                        destination,
                        countdowns,
                        bgcolor_class: "bg-red-900",
                    }
                });
            }
        }
    }
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { class: "monitor", {data_elements.iter().map(|element| { element })} }
    }
}

#[component]
fn DataElement(
    line_name: String,
    destination: String,
    countdowns: Vec<String>,
    bgcolor_class: &'static str,
) -> Element {
    let line_element = rsx! {
        if line_name.contains("U") {
            div { class: "line-element",
                UndergroundElement { line_name }
            }
        } else if line_name.contains("N") {
            div { class: "line-element",
                NightLine { line_name }
            }
        } else if line_name.contains("Badner") {
            div { class: "line-element",
                RailwayPrivateCompany { line_name: "WLB" }
            }
        } else if line_name.contains("WB") {
            div { class: "line-element",
                RailwayPrivateCompany { line_name }
            }
        } else if line_name.contains("RJ") || line_name.contains("IC") || line_name.contains("CJ")
            || line_name.contains("EC")
        {
            div { class: "line-element",
                RailwayTrain { line_name }
            }
        } else if line_name.contains("S") || line_name.contains("R") || line_name.contains("REX") {
            div { class: "line-element",
                RailwayRegional { line_name }
            }
        } else if line_name.contains("A") || line_name.contains("B") {
            div { class: "line-element",
                Bus { line_name }
            }
        } else {
            div { class: "line-element",
                Tram { line_name }
            }
        }
    };
    let destination_element = rsx! {
        div { class: "text-amber-200 destination-element", "{destination}" }
    };
    let countdown_element = create_countdown_times_from_vec(countdowns);

    rsx! {
        div { class: "data-element {bgcolor_class}",
            div { class: "line-and-destination-container", lang: "de",
                {line_element}
                {destination_element}
            }
            div { class: "countdown-container", {countdown_element} }
        }
    }
}

#[component]
fn UndergroundElement(line_name: String) -> Element {
    let underground_element = match line_name.as_str() {
        "U1" => rsx! {
            div { LineU1 {} }
        },
        "U2" => rsx! {
            div { LineU2 {} }
        },
        "U3" => rsx! {
            div { LineU3 {} }
        },
        "U4" => rsx! {
            div { LineU4 {} }
        },
        "U6" => rsx! {
            div { LineU6 {} }
        },
        _ => unreachable!(),
    };

    rsx! {
        {underground_element}
    }
}

fn create_countdown_times_from_vec(countdowns: Vec<String>) -> Element {
    rsx! {
        div { class: "text-amber-200 countdown-element",
            for countdown in countdowns {
                if countdown == "0" {
                    div { class: "signal-container",
                        span { class: "signal", "■   " }
                    }
                } else {
                    div { class: "count-container",
                        span { "{countdown}" }
                    }
                }
            }
        }
    }
}
