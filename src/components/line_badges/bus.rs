use super::CoreBadge;
use dioxus::prelude::*;

#[component]
pub fn Bus(line_name: String) -> Element {
    rsx! {
        CoreBadge { line_name, color_class: "bg-sky-700" }
    }
}
