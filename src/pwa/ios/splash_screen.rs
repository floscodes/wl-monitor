use dioxus::prelude::*;

#[component]
pub fn SplashScreenLinkTags() -> Element {
rsx! {
    // 1. iPhone 14 Pro / 15 Pro (Die aktuelle Standard-Größe)
    link {
        rel: "apple-touch-startup-image",
        media: "screen and (device-width: 393px) and (device-height: 852px) and (-webkit-device-pixel-ratio: 3) and (orientation: portrait)",
        href: asset!("/assets/icons/iOS-icons/splash-screen/apple-splash-1179-2556.png"),
    }

    // 2. iPhone 14 Pro Max / 15 Pro Max (Die große Pro-Größe)
    link {
        rel: "apple-touch-startup-image",
        media: "screen and (device-width: 430px) and (device-height: 932px) and (-webkit-device-pixel-ratio: 3) and (orientation: portrait)",
        href: asset!("/assets/icons/iOS-icons/splash-screen/apple-splash-1290-2796.png"),
    }

    // 3. iPhone 12 / 13 / 14 / 15 (Die Standard-Größe für Basismodelle)
    link {
        rel: "apple-touch-startup-image",
        media: "screen and (device-width: 390px) and (device-height: 844px) and (-webkit-device-pixel-ratio: 3) and (orientation: portrait)",
        href: asset!("/assets/icons/iOS-icons/splash-screen/apple-splash-1170-2532.png"),
    }

    // Bonus: iPhone SE / 8 / 7 (Für Nutzer mit kleineren Bildschirmen/Home-Button)
    link {
        rel: "apple-touch-startup-image",
        media: "screen and (device-width: 375px) and (device-height: 667px) and (-webkit-device-pixel-ratio: 2) and (orientation: portrait)",
        href: asset!("/assets/icons/iOS-icons/splash-screen/apple-splash-750-1334.png"),
    }
}
}