use super::icons::CloseIcon;
use super::input::Input;
use crate::data::dataset::{MonitorData, StationDataSet};
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use search_input::*;
use select_field::*;

mod search_input;
mod select_field;

#[component]
pub fn SearchArea(
    monitor_data: Signal<MonitorData>,
    station_cache: Signal<Vec<StationDataSet>>,
    monitor_loading: Signal<bool>,
    select_field_visibility: Signal<String>,
) -> Element {
    let stations: Signal<Vec<StationDataSet>> = use_signal(|| Vec::new());
    let selected_station_name = use_signal(|| {
        LocalStorage::get("persistent_last_selected_station_name").unwrap_or(String::new())
    });
    let loading_stations = use_signal(|| false);
    let station_selected = use_signal(|| false);
    let clear_visibility = use_signal(|| String::from("hidden"));

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        SearchInput {
            stations,
            select_field_visibility,
            selected_station_name,
            station_cache,
            loading_stations,
            station_selected,
            clear_visibility,
        }
        SelectField {
            stations,
            select_field_visibility,
            selected_station_name,
            monitor_data,
            station_cache,
            loading_stations,
            station_selected,
            monitor_loading,
            clear_visibility,
        }
    }
}
