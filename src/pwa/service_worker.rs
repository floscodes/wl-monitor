use dioxus::prelude::*;

const SW_JS: Asset = asset!("/assets/js/android_service_worker.js");

#[inline]
pub async fn run() {
    let sw_path = SW_JS.to_string();
    let js_code = format!(r#"
        if ('serviceWorker' in navigator) {{
                window.addEventListener('load', () => {{
                    navigator.serviceWorker.register("{sw_path}");
            }});
        }}
    "#);

    let _ = document::eval(&js_code).await;
}