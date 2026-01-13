use dioxus::prelude::*;

pub mod splash_screen;

pub async fn is_ios() -> bool {
    let js_check = document::eval("return /iPad|iPhone|iPod/.test(navigator.userAgent) || (navigator.platform === 'MacIntel' && navigator.maxTouchPoints > 1);").await;
    js_check
        .map(|check| check.as_bool().unwrap_or(false))
        .unwrap_or(false)
}

pub async fn is_safari() -> bool {
    let js_check = document::eval("return /safari/i.test(navigator.userAgent) && !/crios|fxios|opios|edgios/i.test(navigator.userAgent);").await;
    js_check
        .map(|check| check.as_bool().unwrap_or(false))
        .unwrap_or(false)
}
