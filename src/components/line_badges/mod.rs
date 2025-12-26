pub use bus::*;
use dioxus::prelude::*;
pub use nightline::*;
pub use railway::*;
pub use tram::*;
pub use underground::*;

pub mod bus;
pub mod nightline;
pub mod railway;
pub mod tram;
pub mod underground;

#[component]
pub fn CoreBadge(line_name: String, color_class: &'static str) -> Element {
    rsx! {
        div { class: "p-8 h-6 min-w-11 text-center rounded-xs text-white {color_class}",
            "{line_name}"
        }
    }
}
