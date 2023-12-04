// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use radiobrowser::blocking::RadioBrowserAPI;
use radiobrowser::ApiStation;
use radiobrowser::ApiTag;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct JSTag {
    name: String,
    stationcount: u32,
}

fn tag_convert(rb_tag: &ApiTag) -> JSTag {
    JSTag {
        name: rb_tag.name.clone(),
        stationcount: rb_tag.stationcount,
    }
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, stations, tags])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn fetch_stations(search_string: &str) -> Vec<ApiStation> {
    let api = RadioBrowserAPI::new().expect("Unable to initialize RadioBrowserAPI");
    let limit = String::from("100");
    let results = api
        .get_stations()
        .name(search_string)
        .order(radiobrowser::StationOrder::Votes)
        .reverse(true)
        .limit(limit)
        .send();
    match results {
        Ok(rb_stations) => rb_stations,
        Err(_error) => Vec::<ApiStation>::new(),
    }
}

/// This function takes in a vector of `ApiStation` objects and a
/// vector of strings called `parts` as input.  It iterates over each
/// `ApiStation` object and checks if its name contains all the
/// keywords specified in `parts`.  If it does, it converts the
/// `ApiStation` object to a `JSStation` object using the
/// `station_convert` function and adds it to a new vector called
/// `jss`.  Finally, it returns the `jss` vector.
///
/// # Arguments
///
/// * `rb_stations` - A vector of `ApiStation` objects representing
///   radio stations.
/// * `parts` - A vector of strings representing
///   keywords to search for in the station names.
///
/// # Example
///
/// ```
/// let rb_stations = Vec::<ApiStations>::new();
/// let parts = vec!["search", "station"];
/// let result = stations_post(rb_stations, parts);
/// println!("{:?}", result);
/// ```
///
fn stations_post(rb_stations: Vec<ApiStation>, parts: Vec<&str>) -> Vec<ApiStation> {
    let mut jss: Vec<ApiStation> = Vec::new();
    for rb in rb_stations.iter() {
        let mut should_include = true;
        let downcase_name = rb.name.to_lowercase();
        if parts.len() > 1 {
            for keyword in &parts[1..] {
                //let downcase_keyword = keyword.to_lowercase();
                if !downcase_name.contains(keyword) {
                    should_include = false;
                    break;
                }
            }
        }
        if should_include {
            jss.push(rb.clone());
        }
    }

    jss
}

#[tauri::command]
fn stations(search_string: &str) -> Vec<ApiStation> {
    let downcase_search_string = search_string.to_lowercase();
    let parts: Vec<&str> = downcase_search_string.split_whitespace().collect();
    let stations: Vec<ApiStation> = if parts.is_empty() {
        fetch_stations(search_string)
    } else {
        fetch_stations(parts[0])
    };

    stations_post(stations, parts)
}

#[tauri::command]
fn tags() -> Vec<JSTag> {
    let api = RadioBrowserAPI::new().expect("Unable to initialize RadioBrowserAPI");
    let result = api.get_tags().send();

    match result {
        Ok(rb_tags) => {
            let mut jst: Vec<JSTag> = Vec::new();
            for rb in rb_tags.iter() {
                jst.push(tag_convert(rb));
            }
            jst
        }

        Err(_error) => Vec::new(),
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Returns a vector of `ApiStation` structs for testing purposes.
    ///
    /// The vector contains two stations with hardcoded values for their respective fields.
    fn test_api_stations() -> Vec<ApiStation> {
        vec![
            ApiStation {
                name: String::from("station1"),
                stationuuid: String::from("uuid1"),
                url: "url1".to_string(),
                homepage: "homepage1".to_string(),
                favicon: "favicon1".to_string(),
                codec: "codec1".to_string(),
                bitrate: 128,
                clickcount: 10,
                votes: 5,
                changeuuid: "".to_string(),
                serveruuid: None,
                url_resolved: "".to_string(),
                tags: "".to_string(),
                country: "".to_string(),
                countrycode: "".to_string(),
                iso_3166_2: None,
                state: "".to_string(),
                language: "".to_string(),
                languagecodes: None,
                lastchangetime_iso8601: None,
                hls: 0,
                lastcheckok: 0,
                lastchecktime_iso8601: None,
                lastcheckoktime_iso8601: None,
                lastlocalchecktime_iso8601: None,
                clicktimestamp_iso8601: None,
                clicktrend: 0,
                ssl_error: None,
                geo_lat: None,
                geo_long: None,
                has_extended_info: None,
            },
            ApiStation {
                name: "station2".to_string(),
                stationuuid: "uuid2".to_string(),
                url: "url2".to_string(),
                homepage: "homepage2".to_string(),
                favicon: "favicon2".to_string(),
                codec: "codec2".to_string(),
                bitrate: 128,
                clickcount: 10,
                votes: 5,
                changeuuid: "".to_string(),
                serveruuid: None,
                url_resolved: "".to_string(),
                tags: "".to_string(),
                country: "".to_string(),
                countrycode: "".to_string(),
                iso_3166_2: None,
                state: "".to_string(),
                language: "".to_string(),
                languagecodes: None,
                lastchangetime_iso8601: None,
                hls: 0,
                lastcheckok: 0,
                lastchecktime_iso8601: None,
                lastcheckoktime_iso8601: None,
                lastlocalchecktime_iso8601: None,
                clicktimestamp_iso8601: None,
                clicktrend: 0,
                ssl_error: None,
                geo_lat: None,
                geo_long: None,
                has_extended_info: None,
            },
        ]
    }

    // Should return a vector of JSStation objects containing all ApiStation objects when given an empty vector of strings as input.
    #[test]
    fn stations_post_jsstation_results_match_apistation_inputs() {
        let rb_stations = test_api_stations();
        let parts = Vec::<&str>::new();
        let result = stations_post(rb_stations, parts);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name, "station1");
        assert_eq!(result[1].name, "station2");
    }

    #[test]
    fn stations_post_returns_only_string_match() {
        let rb_stations = test_api_stations();
        let parts = vec!["", "sta", "ion2"];
        let result = stations_post(rb_stations, parts);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "station2");
    }

    #[test]
    fn stations_post_returns_empty_vec() {
        let rb_stations = test_api_stations();
        let parts = vec!["", "gabblesnack"];
        let result = stations_post(rb_stations, parts);
        assert_eq!(result.len(), 0);
    }
}
