use crate::components::icons::train::TrainIcon;

use dioxus::prelude::*;

mod screens;

#[derive(PartialEq, Clone)]
pub struct Client {
    pub screen: ClientScreen,
    pub os: ClientOS,
}

impl Client {
    pub fn new() -> Self {
        Self {
            screen: ClientScreen::Desktop,
            os: ClientOS::Other,
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum ClientScreen {
    Mobile,
    Desktop,
}

#[derive(PartialEq, Clone)]
pub enum ClientOS {
    IOS(IsSafari),
    Android,
    Other,
}

#[derive(PartialEq, Clone)]
pub struct IsSafari(pub bool);

#[component]
pub fn WelcomeScreen(client: Memo<Client>) -> Element {
    let client_read = client.read();
    let client_screen = client_read.screen.clone();
    let client_os = client_read.os.clone();
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { class: "welcome-screen",
            div { class: "welcome-screen-logo", TrainIcon {} }
            h1 { "Hi 👋 😊" }
            match client_screen {
                ClientScreen::Mobile => rsx! {
                    match client_os {
                        ClientOS::IOS(_) => rsx! {
                            screens::WelcomeIOS { client }
                        },
                        _ => rsx! {
                            screens::WelcomeAndroid { client }
                        },
                    }
                },
                ClientScreen::Desktop => rsx! {
                    screens::WelcomeDesktop {}
                },
            }
        }
    }
}
