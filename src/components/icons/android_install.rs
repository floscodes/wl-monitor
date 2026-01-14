use dioxus::prelude::*;

#[component]
pub fn AndroidInstallIcon() -> Element {
    rsx! {
        svg {
            xmlns: "www.w3.org",
            view_box: "0 0 24 24",
            fill: "currentColor",
            width: "24px",
            height: "24px",

            path { d: "M20 18c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2H4c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2H0v2h24v-2h-4zM4 6h16v10H4V6z" }

            path { d: "M11 10V7h2v3h3l-4 4-4-4h3z" }
        }
    }
}