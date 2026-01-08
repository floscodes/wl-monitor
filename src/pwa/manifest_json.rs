use dioxus::prelude::Asset;
use urlencoding;


pub fn generate_manifest_href(icon_192: Asset, icon_512: Asset) -> String {
    let base_url = "https://floscodes.github.io/wl-monitor/";
    let icon_192 = format!("{base_url}{icon_192}{icon_192}");
    let icon_512 = format!("{base_url}{icon_512}{icon_512}");
    let manifest_json = format!(r##"
        {{
            "name": "Wiener Linien Abfahrtszeiten Monitor",
            "short_name": "WL-Monitor",
            "description": "Alternative departure times monitor for Vienna's public transport services",
            "start_url": {base_url},
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

    format!("data:application/manifest+json,{}", urlencoding::encode(&manifest_json))
}