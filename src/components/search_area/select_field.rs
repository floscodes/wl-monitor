use super::super::scroll_area::*;
use crate::{
    components::Spinner,
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
                            .map(|station| {
                                let station = station.clone();
                                rsx! {
                                    div {
                                        class: "select-field-item",
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
                                                        let mut station_cache = station_cache.write();
                                                        station_cache.insert(0, station.clone());
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
                                }
                            })
                    }
                }
            }
        }
    }
}
