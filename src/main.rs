use components::*;
use data::dataset::MonitorData;
use dioxus::prelude::*;
use std::time::Duration;

#[cfg(not(feature = "web"))]
use tokio::time::sleep;

#[cfg(feature = "web")]
use wasmtimer::tokio::sleep;

mod components;
pub mod data;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const DX_COMPONENTS: Asset = asset!("/assets/dx-components-theme.css");
const BASE: Asset = asset!("/assets/base.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: DX_COMPONENTS }
        document::Link { rel: "stylesheet", href: BASE }
        Base {}
    }
}

#[component]
fn Base() -> Element {
    let monitor_data = use_signal(|| MonitorData::new());
    let monitor_loading = use_signal(|| false);

    if !monitor_data.read().is_empty() {
        use_future(move || {
            let mut monitor_data = monitor_data.clone();
            async move {
                loop {
                    sleep(Duration::from_secs(11)).await;
                    let new_monitor_data = monitor_data.read().update().await;
                    if let Ok(new_monitor_data) = new_monitor_data {
                        if new_monitor_data.data.len() != 0 {
                            monitor_data.set(new_monitor_data);
                        }
                    }
                }
            }
        });
    }

    rsx! {
        div { class: "base",
            if cfg!(feature = "desktop") || cfg!(feature = "web") {
                TrainIcon {}
            }
            if cfg!(feature = "mobile") {
                br {}
                br {}
                br {}
            }
            div { class: "search-area",
                SearchArea { monitor_data, monitor_loading }
            }
            br {}
            br {}
            Monitor { monitor_data, monitor_loading }
        }
    }
}
