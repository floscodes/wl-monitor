use super::icons::CloseIcon;
use super::input::Input;
use crate::data::dataset::{MonitorData, StationDataSet};
use dioxus::prelude::*;
use search_input::*;
use select_field::*;

mod search_input;
mod select_field;

#[component]
pub fn SearchArea(monitor_data: Signal<MonitorData>, monitor_loading: Signal<bool>) -> Element {
    let stations: Signal<Vec<StationDataSet>> = use_signal(|| Vec::new());
    let select_field_visibility = use_signal(|| String::from("hidden"));
    let selected_station_name = use_signal(|| String::new());
    let cache = use_signal(|| Vec::<StationDataSet>::new());
    let loading_stations = use_signal(|| true);
    let station_selected = use_signal(|| false);
    let clear_visibility = use_signal(|| String::from("hidden"));

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        SearchInput {
            stations,
            select_field_visibility,
            selected_station_name,
            cache,
            loading_stations,
            station_selected,
            clear_visibility,
        }
        SelectField {
            stations,
            select_field_visibility,
            selected_station_name,
            monitor_data,
            cache,
            loading_stations,
            station_selected,
            monitor_loading,
            clear_visibility,
        }
    }
}
