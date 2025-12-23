use super::CoreBadge;
use dioxus::prelude::*;

#[component]
pub fn LineU1() -> Element {
    rsx! {
        CoreBadge { line_name: "U1", color_class: "bg-red-600" }
    }
}

#[component]
pub fn LineU2() -> Element {
    rsx! {
        CoreBadge { line_name: "U2", color_class: "bg-fuchsia-700" }
    }
}

#[component]
pub fn LineU3() -> Element {
    rsx! {
        CoreBadge { line_name: "U3", color_class: "bg-orange-500" }
    }
}

#[component]
pub fn LineU4() -> Element {
    rsx! {
        CoreBadge { line_name: "U4", color_class: "bg-lime-600" }
    }
}

#[component]
pub fn LineU6() -> Element {
    rsx! {
        CoreBadge { line_name: "U6", color_class: "bg-yellow-900" }
    }
}
