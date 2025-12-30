use super::components::train::TrainIcon;
use dioxus::prelude::*;

#[component]
pub fn WelcomeScreen() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { class: "welcome-screen",
            div { class: "welcome-screen-logo", TrainIcon {} }
            h1 { "Willkommen 👋" }
            p { "Füge diese App zu deinem Home-Bildschirm hinzu:" }
            ul {
                li {
                    "1.) Klicke unten auf "
                    Points {}
                }
                li { "2.) Klicke auf „Teilen“" }
                li { "3.) Klicke auf „Zum Home-Bildschirm“" }
            }
        }
    }
}

#[component]
fn Points() -> Element {
    rsx! {
        div { class: "welcome-screen-points",
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                fill: "none",
                view_box: "0 0 24 24",
                stroke_width: "1.5",
                stroke: "currentColor",
                class: "size-2",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    d: "M8.625 12a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H8.25m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H12m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0h-.375M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
                }
            }
        }
    }
}
