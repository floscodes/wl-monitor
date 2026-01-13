use super::CloseIcon;
use super::Input;
use crate::data::dataset::StationDataSet;
use anyhow;
use dioxus::prelude::*;
use std::time::Duration;

#[cfg(not(feature = "web"))]
use tokio::time::sleep;

#[cfg(feature = "web")]
use wasmtimer::tokio::sleep;

#[component]
pub fn SearchInput(
    stations: Signal<Vec<StationDataSet>>,
    select_field_visibility: Signal<String>,
    selected_station_name: Signal<String>,
    mut station_cache: Signal<Vec<StationDataSet>>,
    loading_stations: Signal<bool>,
    station_selected: Signal<bool>,
    clear_visibility: Signal<String>,
) -> Element {
    let onfocus = move |_: FocusEvent| {
        let mut station_cache_write = station_cache.write();
        if station_cache_write.len() == 0 {
            return;
        }
        *station_cache_write = remove_double_values_from_cache(&*station_cache_write);
        stations.set(station_cache_write.clone());
        select_field_visibility.set(String::from("visible"));

        let input_value = selected_station_name.read().clone();
        if input_value.len() > 0 {
            clear_visibility.set(String::from("visible"));
        } else {
            clear_visibility.set(String::from("hidden"));
        }
    };
    let onfocusout = move |_: FocusEvent| {
        clear_visibility.set(String::from("hidden"));
        let selected_station_name_string = selected_station_name.read().clone();
        selected_station_name.set(selected_station_name_string);
    };
    let mut search_fn = use_action(
        move |value: String, mut stations: Signal<Vec<StationDataSet>>| async move {
            if !*station_selected.read() {
                loading_stations.set(true);
                let cached_stations = stations.read().clone();
                sleep(Duration::from_millis(350)).await;
                let fetched_stations = StationDataSet::search_request(value).await;
                let concated_stations = concat_fetched_stations_and_cache(cached_stations, fetched_stations);
                stations.set(concated_stations);
                loading_stations.set(false);
            } else {
                station_selected.set(false);
            }
            Result::<(), anyhow::Error>::Ok(())
        },
    );
    let oninput = move |e: FormEvent| async move {
        let search_string = e.value();

        match search_string.len() {
            0 => {
                stations.set(station_cache.read().clone());
                return;
            }
            1..4 => {
                stations.set(check_cache_and_filter(&search_string, station_cache));
                return;
            }
            _ => {
                if search_string.trim().is_empty() {
                    stations.set(Vec::new());
                    return;
                }
                select_field_visibility.set(String::from("visible"));
                stations.set(check_cache_and_filter(&search_string, station_cache));

                if !stations.read().len() == 0 {
                    return;
                }
            }
        }
        search_fn
            .call(search_string, stations)
            .await;
    };

    rsx! {
        div { class: "input-wrapper",
            Input {
                id: "search_input",
                placeholder: "Suche Haltestelle...",
                onfocus,
                onfocusout,
                oninput,
                value: selected_station_name,
            }
            div {
                class: "search-input-clear-button",
                visibility: "{clear_visibility}",
                onpointerdown: move |_| {
                    selected_station_name.set(String::new());
                    clear_visibility.set(String::from("hidden"));
                },
                CloseIcon {}
            }
        }
    }
}

fn check_cache_and_filter(
    search_string: &String,
    cache: Signal<Vec<StationDataSet>>,
) -> Vec<StationDataSet> {
    let mut filtered_cache = Vec::new();
    let cache = cache.read();
    for cached_station in cache.iter() {
        if cached_station
            .name
            .trim()
            .to_lowercase()
            .contains(&search_string.trim().to_lowercase())
        {
            if !filtered_cache.contains(cached_station) {
                filtered_cache.push(cached_station.clone());
            }
        }
    }
    filtered_cache
}

fn remove_double_values_from_cache(vec: &Vec<StationDataSet>) -> Vec<StationDataSet> {
    let mut unique_vec = Vec::new();
    for station in vec {
        if !unique_vec.contains(station) {
            unique_vec.push(station.clone());
        }
    }
    unique_vec
}

fn concat_fetched_stations_and_cache(cached_stations: Vec<StationDataSet>, fetched_stations: Vec<StationDataSet>) -> Vec<StationDataSet> {
    let mut concated_stations = cached_stations;

    for fetched_station in &fetched_stations {
        if !concated_stations.contains(fetched_station) {
            concated_stations.push(fetched_station.clone());
        }
    }
    concated_stations
} 
