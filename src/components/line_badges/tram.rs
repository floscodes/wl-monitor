use super::CoreBadge;
use dioxus::prelude::*;

#[component]
pub fn Tram(line_name: String) -> Element {
    rsx! {
        CoreBadge { line_name, color_class: "bg-red-700" }
    }
}
