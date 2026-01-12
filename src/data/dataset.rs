use super::*;
use dioxus::prelude::{*, document::EvalError};
use helpers::*;
use serde_json::{self, Value};
use serde::{Deserialize, Serialize};

const VAO_REGIONAL_START_DIGIT: &'static str = "4";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonitorData {
    pub data: Vec<MonitorDataSet>,
    pub vao: String,
}

impl MonitorData {
    pub fn new() -> Self {
        Self {
            data: Vec::<MonitorDataSet>::new(),
            vao: String::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    pub async fn from_vao(vao: String) -> Result<Self> {
        let mut monitor_data_sets = Vec::new();
        let url = format!("{}{}", MONITOR_URL, vao);
        let js_code = build_js_code(url);
        let request = document::eval(&js_code).await;

        match request {
            Ok(response) => {
                let server_time= response["serverTime"]
                    .as_str()
                    .ok_or(EvalError::InvalidJs("JSON seems to be not valid".to_string()))?;
                if let Some(lines) = response["stations"][0]["station"]["lines"].as_array() {
                    for line in lines {
                        if let Some(line_trips) = line["trips"].as_array() {
                            for line_trip in line_trips {
                                if let Value::String(line_name) = line["name"].clone()
                                    && let Value::String(destination) =
                                        line_trip["tripHeadsign"].clone()
                                {
                                    let departures = line_trip["departures"].clone();
                                    let countdown = calculate_countdown(&server_time, departures);
                                    monitor_data_sets.push(MonitorDataSet::from(
                                        line_name,
                                        destination,
                                        countdown,
                                    ));
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => return Err(e.into()),
        }
        Ok(Self {
            data: monitor_data_sets,
            vao,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonitorDataSet {
    pub line_name: String,
    pub destination: String,
    pub countdown: Vec<String>,
}

impl MonitorDataSet {
    pub fn from(line_name: String, destination: String, countdown: Vec<String>) -> Self {
        Self {
            line_name,
            destination,
            countdown,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StationDataSet {
    pub name: String,
    pub vao: String,
    pub cached: bool,
}

impl StationDataSet {
    pub async fn search_request(search_string: String) -> Vec<Self> {
        let url = format!("{}?s={}", SEARCH_URL, search_string);
        let js_code = build_js_code(url);
        let response = document::eval(&js_code);

        if let Ok(station_json) = response.await {
            let mut station_data_sets = Vec::new();
            if let Some(stations) = station_json.as_array() {
                for station in stations {
                    if let Value::String(ref name) = station["title"]
                        && let Value::String(ref vao) = station["externalId"]
                    {
                        if !vao.replace("vao:", "").starts_with(VAO_REGIONAL_START_DIGIT)
                        {
                            continue;
                        }
                        if station["locationGroupType"] == "station" {
                            let name = trim_station_name(name.clone());
                            let starts_with_search_string =
                                name.to_lowercase().contains(&search_string.to_lowercase());
                            if starts_with_search_string {
                                station_data_sets.push(StationDataSet {
                                    name: name,
                                    vao: vao.clone(),
                                    cached: false,
                                });
                            }
                        }
                    }
                }
            }
            return station_data_sets;
        }
        Vec::new()
    }
}
