use dioxus::prelude::Asset;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};


pub fn generate_manifest_href(icon_192: Asset, icon_512: Asset) -> String {
    let manifest_json = format!(r##"
        {{
            "name": "Wiener Linien Abfahrtszeiten Monitor",
            "short_name": "WL-Monitor",
            "description": "Alternative departure times monitor for Vienna's public transport services",
            "start_url": "/wl-monitor",
            "display": "standalone",
            "background_color": "#49170eff",
            "theme_color": "#8f2e1d",
            "orientation": "portrait",
            "icons": [
                {{ "src": "{icon_192}", "sizes": "192x192", "type": "image/png" }},
                {{ "src": "{icon_512}", "sizes": "512x512", "type": "image/png" }}
            ]
        }}
    "##);

    format!("data:application/manifest+json;base64,{}", URL_SAFE.encode(manifest_json))
}