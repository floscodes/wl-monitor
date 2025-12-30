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
        meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1.0, viewport-fit=cover",
        }
        meta { charset: "UTF-8", lang: "de-AT" }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: DX_COMPONENTS }
        document::Link { rel: "stylesheet", href: BASE }
        div { class: "blur-zone-top" }
        Base {}
        div { class: "blur-zone-bottom" }
    }
}

#[component]
fn Base() -> Element {
    let monitor_data = use_signal(|| MonitorData::new());
    let monitor_loading = use_signal(|| false);
    let select_field_visibility = use_signal(|| String::from("hidden"));

    use_future(move || {
        let mut monitor_data = monitor_data.clone();
        async move {
            let mut sleep_time: u64 = 12;
            loop {
                sleep(Duration::from_secs(sleep_time)).await;

                if !monitor_data.read().is_empty() && !*monitor_loading.read() {
                    let monitor_data_value = monitor_data.read().clone();
                    let vao_value = monitor_data_value.vao.clone();
                    let new_monitor_data = MonitorData::from_vao(vao_value).await;
                    if let Ok(new_monitor_data) = new_monitor_data {
                        if sleep_time != 12 {
                            sleep_time = 12;
                        }
                        if new_monitor_data.data.len() != 0 {
                            monitor_data.set(new_monitor_data);
                        }
                    } else {
                        if sleep_time != 0 {
                            sleep_time -= 4;
                        }
                    }
                }
            }
        }
    });

    rsx! {
        div { class: "base",
            div { class: "search-area",
                SearchArea {
                    monitor_data,
                    monitor_loading,
                    select_field_visibility,
                }
            }
            Monitor {
                monitor_data,
                monitor_loading,
                select_field_visibility,
            }
        }
    }
}
