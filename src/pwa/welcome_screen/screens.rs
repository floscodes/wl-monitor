use dioxus::prelude::*;
use super::*;

#[component]
pub fn WelcomeIOS(client: Signal<Client>) -> Element {
    let is_safari = client.read().os == ClientOS::IOS(IsSafari(true));
    rsx! {
        p { "Füge diese App zu deinem Home-Bildschirm hinzu:" }
        ul {
            if is_safari {
                li {
                    "1.) Klicke unten auf "
                    Points {}
                }
                li { "2.) Klicke auf „Teilen“" }
                li { "3.) Klicke auf „Zum Home-Bildschirm“" }
            } else {
                li {
                    "1.) Öffne diese Seite in Safari "
                    div { class: "safari-icon-container", Safari {} }
                }
                li {
                    "2.) Klicke dann unten auf "
                    Points {}
                }
                li { "3.) Klicke auf „Teilen“" }
                li { "4.) Klicke auf „Zum Home-Bildschirm“" }
            }
        }
    }
}

#[component]
pub fn WelcomeAndroid(client: Signal<Client>) -> Element {
    let is_safari = client.read().os == ClientOS::IOS(IsSafari(true));
    rsx! {
        p { "Füge diese App zu deinem Home-Bildschirm hinzu:" }
        ul {
            if is_safari {
                li {
                    "1.) Klicke unten auf "
                    Points {}
                }
                li { "2.) Klicke auf „Teilen“" }
                li { "3.) Klicke auf „Zum Home-Bildschirm“" }
            } else {
                li {
                    "1.) Öffne diese Seite in Safari "
                    div { class: "safari-icon-container", Safari {} }
                }
                li {
                    "2.) Klicke dann unten auf "
                    Points {}
                }
                li { "3.) Klicke auf „Teilen“" }
                li { "4.) Klicke auf „Zum Home-Bildschirm“" }
            }
        }
    }
}

#[component]
pub fn WelcomeDesktop() -> Element {
    rsx!{
        p { "Öffne diese Seite auf deinem Smartphone!" }
        if let Some(qr_code) = url_qr_code() {
            p { "oder scanne diesen QR-Code:" }
            {qr_code}
        }
    }
}

#[component]
pub fn Points() -> Element {
    rsx! {
        div { class: "welcome-screen-points",
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                fill: "none",
                view_box: "0 0 24 24",
                stroke_width: "1.5",
                stroke: "currentColor",
                class: "size-2",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    d: "M8.625 12a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H8.25m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H12m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0h-.375M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
                }
            }
        }
    }
}

fn url_qr_code() -> Option<Element> {
    let qr_code = qrcode_generator::to_svg_to_string(
        "https://floscodes.github.io/wl-monitor/",
        QrCodeEcc::Low, 1024, None::<&str>,
    ).ok()?;
    Some(rsx! {
        div { class: "qr-container", dangerous_inner_html: "{qr_code}" }
    })
}