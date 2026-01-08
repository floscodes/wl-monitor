pub mod android;
pub mod ios;
pub mod service_worker;
pub mod manifest_json;

use dioxus::prelude::*;

pub async fn is_installed() -> bool {
    let js_check = document::eval("return (window.matchMedia('(display-mode: standalone)').matches || window.navigator.standalone === true);").await;
    js_check.map(|check| check.as_bool().unwrap_or(false)).unwrap_or(false)
}