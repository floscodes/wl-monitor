//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component  to be used in our app.
pub mod monitor;
pub use monitor::Monitor;
pub mod input;
mod line_badges;
pub mod search_area;
pub use search_area::SearchArea;
pub mod icons;
pub use icons::*;
pub mod scroll_area;
