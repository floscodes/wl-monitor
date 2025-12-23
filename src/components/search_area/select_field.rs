use super::super::scroll_area::*;
use crate::{
    components::Spinner,
    data::dataset::{MonitorData, StationDataSet},
};
use dioxus::prelude::*;
use dioxus_primitives::scroll_area::ScrollDirection;

#[component]
pub fn SelectField(
    stations: Signal<Vec<StationDataSet>>,
    select_field_visibility: Signal<String>,
    selected_station_name: Signal<String>,
    monitor_data: Signal<MonitorData>,
    cache: Signal<Vec<StationDataSet>>,
    loading_stations: Signal<bool>,
    station_selected: Signal<bool>,
    monitor_loading: Signal<bool>,
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
                                        onclick: move |_| {
                                            let station = station.clone();
                                            spawn(async move {
                                                station_selected.set(true);
                                                select_field_visibility.set(String::from("hidden"));
                                                if *selected_station_name.read() == station.name {
                                                    return;
                                                }
                                                selected_station_name.set(station.name.clone());
                                                monitor_loading.set(true);
                                                let station_clone = station.clone();
                                                let data = MonitorData::from_vao(station_clone.vao.clone()).await;
                                                if let Ok(data) = &data {
                                                    monitor_data.set(data.clone());
                                                    monitor_loading.set(false);
                                                    let mut cache = cache.write();
                                                    cache.insert(0, station.clone());
                                                }
                                            });
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
