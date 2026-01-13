use dioxus::prelude::*;

pub async fn is_android() -> bool {
    let js_check = document::eval("return /Android/i.test(navigator.userAgent);").await;
    js_check
        .map(|check| check.as_bool().unwrap_or(false))
        .unwrap_or(false)
}

pub async fn is_chrome() -> bool {
    let js_check = document::eval("return /Chrome/i.test(navigator.userAgent);").await;
    js_check
        .map(|check| check.as_bool().unwrap_or(false))
        .unwrap_or(false)
}
