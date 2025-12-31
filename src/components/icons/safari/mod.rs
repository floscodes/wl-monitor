use dioxus::prelude::*;

#[component]
pub fn Safari() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        span { class: "safari-icon",
            img { src: asset!("/assets/icons/safari.svg") }
        }
    }
}
