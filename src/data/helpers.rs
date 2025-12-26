use serde_json::Value;
use time::OffsetDateTime;

pub fn build_js_code(url: String) -> String {
    format!(
        r#"
    let response = await fetch("{}");
    let response_json = await response.json();
    return response_json;
    "#,
        url
    )
}

pub fn trim_station_name(station_name: String) -> String {
    station_name
        .rsplit_once(",")
        .ok_or_else(|| station_name.clone())
        .unwrap_or((station_name.as_str(), ""))
        .0
        .trim()
        .to_string()
}

pub fn calculate_countdown(server_time: &str, departures: Value) -> Vec<String> {
    let mut countdown = vec![];
            if let Some(departures) = departures.as_array() {
                for departure in departures {
                    if let Value::String(estimated_time) = departure["estimatedAt"].clone() {
                        if let Ok(estimated_departure_timestamp) = OffsetDateTime::parse(
                            &estimated_time,
                            &time::format_description::well_known::Rfc3339,
                        ) && let Ok(server_time) = OffsetDateTime::parse(
                            server_time,
                            &time::format_description::well_known::Rfc3339,
                        ) {
                            let duration = estimated_departure_timestamp - server_time;
                            let mut minutes = duration.whole_minutes();
                            if minutes < 0 {
                                minutes = 0;
                            }
                            countdown.push(minutes.to_string());
                        }
                    }
                }
            }
    countdown
}
