pub mod manifest_json;

use dioxus::prelude::*;

pub async fn is_android_pwa() -> bool {
    let js_check = document::eval("return /Android/i.test(navigator.userAgent);").await;
    js_check.map(|check| check.as_bool().unwrap_or(false)).unwrap_or(false)
}