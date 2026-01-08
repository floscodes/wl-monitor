use dioxus::prelude::*;

const SW_JS: Asset = asset!("/assets/js/android_service_worker.js");

#[inline]
pub async fn run() {
    let js_code = format!(r#"
        if ('serviceWorker' in navigator) {{
                window.addEventListener('load', () => {{
                    navigator.serviceWorker.register("{SW_JS}");
            }});
        }}
    "#);

    let _ = document::eval(&js_code).await;
}