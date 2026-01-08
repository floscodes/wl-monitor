use components::*;
use data::dataset::MonitorData;
use dioxus::prelude::*;
use std::time::Duration;
use welcome_screen::WelcomeScreen;

#[cfg(not(feature = "web"))]
use tokio::time::sleep;

#[cfg(feature = "web")]
use wasmtimer::tokio::sleep;

mod components;
pub mod data;
mod welcome_screen;
mod manifest_json;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const DX_COMPONENTS: Asset = asset!("/assets/dx-components-theme.css");
const BASE: Asset = asset!("/assets/base.css");
const APP_ICON_192: Asset = asset!("/assets/icons/Android-icons/Icon-192.png");
const APP_ICON_512: Asset = asset!("/assets/icons/Android-icons/Icon-512.png");

#[allow(non_upper_case_globals)]
const APP_ICON_180_iOS: Asset = asset!("/assets/icons/iOS-icons/Icon-180.png");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut is_installed = use_signal(|| false);
    let mut is_ios = use_signal(|| false);
    let mut is_safari = use_signal(|| false);

    use_future(move || async move {
        let js_check = document::eval("return (window.matchMedia('(display-mode: standalone)').matches || window.navigator.standalone === true);").await;
        if let Ok(check) = js_check {
            is_installed.set(check.as_bool().unwrap_or(false));
        }
    });

    use_future(move || async move {
        let js_check = document::eval(r#"return (/iPad|iPhone|iPod/.test(navigator.userAgent) || (navigator.platform === "MacIntel" && navigator.maxTouchPoints > 1));"#).await;
        if let Ok(check) = js_check {
            is_ios.set(check.as_bool().unwrap_or(false));
        }
    });

    use_future(move || async move {
        let js_check = document::eval(r#"return (/safari/i.test(navigator.userAgent) && !/crios|fxios|opios|edgios/i.test(navigator.userAgent));"#).await;
        if let Ok(check) = js_check {
            is_safari.set(check.as_bool().unwrap_or(false));
        }
    });

    rsx! {
        meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1.0, viewport-fit=cover",
        }
        meta { charset: "UTF-8", lang: "de-AT" }
        meta { name: "apple-mobile-web-app-capable", content: "yes" }
        meta {
            name: "apple-mobile-web-app-status-bar-style",
            content: "#49170eff",
        }
        meta { name: "theme-color", content: "#8f2e1d" }
        meta { name: "apple-mobile-web-app-title", content: "WL-Monitor" }

        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: DX_COMPONENTS }
        document::Link { rel: "stylesheet", href: BASE }
        document::Link { rel: "apple-touch-icon", href: APP_ICON_180_iOS }
        document::Link {
            rel: "manifest",
            href: manifest_json::generate_manifest_href(APP_ICON_192, APP_ICON_512),
        }

        if *is_ios.read() && !*is_installed.read() {
            WelcomeScreen { is_safari }
        } else {
            div { class: "blur-zone-top" }
            Base {}
            div { class: "blur-zone-bottom" }
        }
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
                    let new_monitor_data =
                        MonitorData::from_vao(monitor_data.read().vao.clone()).await;
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
