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
    mut cache: Signal<Vec<StationDataSet>>,
    loading_stations: Signal<bool>,
    station_selected: Signal<bool>,
    clear_visibility: Signal<String>,
) -> Element {
    let onfocus = move |_: FocusEvent| {
        let mut cache_write = cache.write();
        if cache_write.len() == 0 {
            return;
        }
        *cache_write = remove_double_values_from_vec(&*cache_write);
        stations.set(cache_write.clone());
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
    };
    let mut cache_clone = cache.clone();
    let mut search_fn = use_action(
        move |value: String, mut stations: Signal<Vec<StationDataSet>>| async move {
            if !*station_selected.read() {
                loading_stations.set(true);
                sleep(Duration::from_millis(350)).await;
                stations.set(StationDataSet::search_request(value).await);
                let mut cache_vec = cache_clone.write();
                cache_vec.extend(stations.read().clone());
                *cache_vec = remove_double_values_from_vec(&*cache_vec);
                cache_vec.truncate(7);
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
                stations.set(cache.read().clone());
                return;
            }
            1..4 => {
                stations.set(check_cache_and_filter(&search_string, cache));
                return;
            }
            _ => {
                if search_string.trim().is_empty() {
                    stations.set(Vec::new());
                    return;
                }
                select_field_visibility.set(String::from("visible"));
                stations.set(check_cache_and_filter(&search_string, cache));

                if !stations.read().len() == 0 {
                    return;
                }
            }
        }
        search_fn
            .call(search_string.clone(), stations.clone())
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
    for station in cache.read().iter() {
        if station
            .name
            .trim()
            .to_lowercase()
            .contains(&search_string.trim().to_lowercase())
        {
            if !filtered_cache.contains(station) {
                filtered_cache.push(station.clone());
            }
        }
    }
    filtered_cache
}

fn remove_double_values_from_vec(vec: &Vec<StationDataSet>) -> Vec<StationDataSet> {
    let mut unique_vec = Vec::new();
    for station in vec {
        if !unique_vec.contains(station) {
            unique_vec.push(station.clone());
        }
    }
    unique_vec
}
