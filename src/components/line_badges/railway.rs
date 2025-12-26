use super::CoreBadge;
use dioxus::prelude::*;

#[component]
pub fn RailwayRegional(line_name: String) -> Element {
    rsx! {
        CoreBadge { line_name, color_class: "bg-sky-700" }
    }
}

#[component]
pub fn RailwayTrain(line_name: String) -> Element {
    rsx! {
        CoreBadge { line_name, color_class: "bg-red-700" }
    }
}

#[component]
pub fn RailwayPrivateCompany(line_name: String) -> Element {
    rsx! {
        CoreBadge { line_name, color_class: "bg-sky-700" }
    }
}
