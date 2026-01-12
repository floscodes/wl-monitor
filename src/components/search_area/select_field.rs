use super::super::scroll_area::*;
use crate::{
    components::{Clock, CloseIcon, Spinner},
    data::dataset::{MonitorData, StationDataSet},
};
use dioxus::prelude::*;
use dioxus_primitives::scroll_area::ScrollDirection;
use gloo_storage::{LocalStorage, Storage};

#[component]
pub fn SelectField(
    stations: Signal<Vec<StationDataSet>>,
    select_field_visibility: Signal<String>,
    selected_station_name: Signal<String>,
    monitor_data: Signal<MonitorData>,
    station_cache: Signal<Vec<StationDataSet>>,
    loading_stations: Signal<bool>,
    station_selected: Signal<bool>,
    monitor_loading: Signal<bool>,
    clear_visibility: Signal<String>,
) -> Element {
    let stations_vec = stations.read();

    rsx! {
        ScrollArea {
            direction: ScrollDirection::Vertical,
            class: "select-field-container",
            div {
                class: "select-field mr-3",
                visibility: "{select_field_visibility}",
                if *loading_stations.read() {
                    Spinner {}
                } else {
                    {
                        stations_vec
                            .iter()
                            .enumerate()
                            .map(|(index, station)| {
                                let station = station.clone();
                                rsx! {
                                    div { class: "select-field-item",
                                        if station.cached {
                                            ChachedIconClock {}
                                        }
                                        span {
                                            class: "select-field-item-station-name",
                                            onclick: move |_| {
                                                let station = station.clone();
                                                async move {
                                                    let station = station.clone();
                                                    spawn(async move {
                                                        station_selected.set(true);
                                                        select_field_visibility.set(String::from("hidden"));
                                                        selected_station_name.set(station.name.clone());
                                                        monitor_loading.set(true);
                                                        clear_visibility.set(String::from("hidden"));
                                                        let station_clone = station.clone();
                                                        let data = MonitorData::from_vao(station_clone.vao.clone()).await;
                                                        if let Ok(data) = &data {
                                                            monitor_data.set(data.clone());
                                                            monitor_loading.set(false);
                                                            let mut station_to_cache = station.clone();
                                                            station_to_cache.cached = true;
                                                            let mut station_cache = station_cache.write();
                                                            station_cache.insert(0, station_to_cache);
                                                            if station_cache.len() > 7 {
                                                                let _ = station_cache.pop();
                                                            }
                                                            LocalStorage::set("persistent_station_cache", &*station_cache)
                                                                .map_err(|_e| {
                                                                    LocalStorage::set(
                                                                            "persistent_station_cache",
                                                                            vec![station.clone()],
                                                                        )
                                                                        .unwrap_or_default()
                                                                })
                                                                .ok();
                                                            LocalStorage::set("persistent_monitor_data", &data)
                                                                .unwrap_or_default();
                                                            LocalStorage::set(
                                                                    "persistent_last_selected_station_name",
                                                                    &station.name,
                                                                )
                                                                .unwrap_or_default();
                                                        }
                                                    });
                                                }
                                            },
                                            "{station.name}"
                                        }

                

                                        DeleteButton {
                                            index,
                                            stations,
                                            station_cache,
                                            station_cached: station.cached,
                                            select_field_visibility,
                                        }
                
                                    }
                                }
                            })
                    }
                }
            }
        }
    }
}

#[component]
fn DeleteButton(
    index: usize,
    stations: Signal<Vec<StationDataSet>>,
    station_cache: Signal<Vec<StationDataSet>>,
    station_cached: bool,
    select_field_visibility: Signal<String>,
) -> Element {
    let onclick = move |_: MouseEvent| {
        if !station_cached {
            let mut stations_write = stations.write();
            stations_write.remove(index);
            return;
        }
        let mut cache = station_cache.read().clone();
        cache.remove(index);
        stations.set(cache.clone());
        station_cache.set(cache.clone());
        LocalStorage::set("persistent_station_cache", &cache).unwrap_or_default();
        let stations_len = stations.read().len();
        if stations_len == 0 {
            select_field_visibility.set(String::from("hidden"));
        }
    };
    rsx! {
        div { class: "select-field-item-close-button", onclick, CloseIcon {} }
    }
}

#[component]
fn ChachedIconClock() -> Element {
    rsx! {
        span { class: "select-field-item-cached-icon-clock", Clock {} }
    }
}
