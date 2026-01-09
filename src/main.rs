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
mod pwa;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const DX_COMPONENTS: Asset = asset!("/assets/dx-components-theme.css");
const BASE: Asset = asset!("/assets/base.css");

#[allow(non_upper_case_globals)]
const APP_ICON_180_iOS: Asset = asset!("/assets/icons/iOS-icons/Icon-180.png");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut is_installed = use_signal(|| false);
    let mut is_android = use_signal(|| false);
    let mut is_ios = use_signal(|| false);
    let mut is_safari = use_signal(|| false);

    use_future(move || async move {
        is_installed.set(
            pwa::is_installed().await
        );
    });
    use_future(move || async move {
        is_ios.set(
            pwa::ios::is_ios_pwa().await
        );
    });
    use_future(move || async move {
        is_safari.set(
            pwa::ios::is_safari().await
        );
    });
    use_future(move || async move {
        is_android.set(
            pwa::android::is_android_pwa().await
        )
    });
    use_future(move || async move {
        pwa::service_worker::run().await;
    });

    rsx! {
        document::Link { rel: "manifest", href: pwa::manifest::generate_manifest_href() }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: DX_COMPONENTS }
        document::Link { rel: "stylesheet", href: BASE }
        document::Link { rel: "apple-touch-icon", href: APP_ICON_180_iOS }

        meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1.0, viewport-fit=cover",
        }
        meta { charset: "UTF-8", lang: "de-AT" }

        if *is_ios.read() {
            meta { name: "apple-mobile-web-app-capable", content: "yes" }
            meta {
                name: "apple-mobile-web-app-status-bar-style",
                content: "#49170eff",
            }
            meta { name: "theme-color", content: "#8f2e1d" }
            meta { name: "apple-mobile-web-app-title", content: "WL-Monitor" }

            {pwa::ios::splash_screen::SplashScreenTags()}
        }

        if *is_ios.read() && !*is_installed.read() {
            pwa::welcome_screen::WelcomeScreen { is_safari }
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
