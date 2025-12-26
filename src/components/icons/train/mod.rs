use dioxus::prelude::*;

#[component]
pub fn TrainIcon() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        span {
            class: "train",
            img {
                src: asset!("/assets/icons/train.svg")
            }
         }
    }
}
