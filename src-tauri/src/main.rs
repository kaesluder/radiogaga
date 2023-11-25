// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use radiobrowser::blocking::RadioBrowserAPI;
use radiobrowser::ApiStation;
use radiobrowser::ApiTag;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct JSStation {
    name: String,
    stationuuid: String,
    url: String,
    homepage: String,
    favicon: String,
    codec: String,
    bitrate: u32,
    clickcount: u32,
    votes: i32,
}

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

fn station_convert(rb_station: ApiStation) -> JSStation {
    JSStation {
        name: rb_station.name,
        stationuuid: rb_station.stationuuid,
        url: rb_station.url,
        homepage: rb_station.homepage,
        favicon: rb_station.favicon,
        codec: rb_station.codec,
        bitrate: rb_station.bitrate,
        clickcount: rb_station.clickcount,
        votes: rb_station.votes,
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, stations, tags])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// # stations
///
/// A function that takes a search string as input and returns a vector of `JSStation` objects.
///
/// ## Example Usage
/// ```rust
/// let search_string = "rock";
/// let result = stations(search_string);
/// ```
///
/// ## Arguments
/// - `search_string`: A string representing the search query for stations.
///
/// ## Returns
/// A vector of `JSStation` objects representing the stations that match the search query.
#[tauri::command]
fn stations(search_string: &str) -> Vec<JSStation> {
    let api = RadioBrowserAPI::new().expect("Unable to initialize RadioBrowserAPI");
    let limit = String::from("100");
    let stations = api
        .get_stations()
        .name(search_string)
        .order(radiobrowser::StationOrder::Votes)
        .reverse(true)
        .limit(limit)
        .send()
        .expect("Unable to download stations.");
    let mut jss: Vec<JSStation> = Vec::new();
    for rb in stations.iter() {
        jss.push(station_convert(rb.clone()));
    }

    jss
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
