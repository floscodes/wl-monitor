use super::CoreBadge;
use dioxus::prelude::*;

#[component]
pub fn NightLine(line_name: String) -> Element {
    rsx! {
        CoreBadge { line_name, color_class: "bg-stone-400" }
    }
}
