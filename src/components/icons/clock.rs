use dioxus::prelude::*;

#[component]
pub fn Clock() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke_width: "1.5",
            stroke: "currentColor",
            class: "size-6",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M12 6v6h4.5m4.5 0a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
            }
        }
    }
}
