use dioxus::prelude::*;
use urlencoding;

const APP_ICON_192: Asset = asset!("/assets/icons/Android-icons/Icon-192.png");
const APP_ICON_512: Asset = asset!("/assets/icons/Android-icons/Icon-512.png");
const SCREENSHOT_MOBILE: Asset = asset!("/assets/icons/screenshot-mobile.png");

pub fn generate_manifest_href() -> String {
    let base_url = "https://floscodes.github.io";
    let icon_192 = format!("{base_url}{APP_ICON_192}");
    let icon_512 = format!("{base_url}{APP_ICON_512}");
    let screenshot_mobile = format!("{base_url}{SCREENSHOT_MOBILE}");
    let manifest_json = format!(r##"
        {{
            "name": "Wiener Linien Abfahrtszeiten Monitor",
            "short_name": "WL-Monitor",
            "description": "Dein einfacher, übersichtlicher und schneller Wiener Linien Monitor für Echtzeit-Abfahrten",
            "start_url": "https://floscodes.github.io/wl-monitor/",
            "display": "standalone",
            "display_override": ["standalone"],
            "background_color": "#49170eff",
            "theme_color": "#8f2e1d",
            "orientation": "portrait",
            "icons": [
                {{ "src": "{icon_192}", "sizes": "192x192", "type": "image/png" }},
                {{ "src": "{icon_512}", "sizes": "512x512", "type": "image/png" }}
            ],
            "screenshots": [
                {{
                    "src": "{screenshot_mobile}",
                    "sizes": "1179x2556",
                    "type": "image/png",
                    "form_factor": "narrow",
                    "label": "Echtzeit-Abfahrtsmonitor für Wien"
                }}
        }}
    "##);

    format!("data:application/manifest+json,{}", urlencoding::encode(&manifest_json))
}