// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use radiobrowser::blocking::RadioBrowserAPI;
use radiobrowser::ApiStation;
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
}

fn station_convert(rb_station: ApiStation) -> JSStation {
    let result = JSStation {
        name: rb_station.name,
        stationuuid: rb_station.stationuuid,
        url: rb_station.url,
        homepage: rb_station.homepage,
        favicon: rb_station.favicon,
        codec: rb_station.codec,
        bitrate: rb_station.bitrate,
        clickcount: rb_station.clickcount,
    };
    return result;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, stations])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn stations(search_string: &str) -> Vec<JSStation> {
    let api = RadioBrowserAPI::new().expect("Unable to initialize RadioBrowserAPI");
    let limit = String::from("100");
    let stations = api
        .get_stations()
        .name(search_string)
        .order(radiobrowser::StationOrder::Clickcount)
        .reverse(true)
        .limit(limit)
        .send()
        .expect("Unable to download stations.");
    let mut jss: Vec<JSStation> = Vec::new();
    for rb in stations.iter() {
        jss.push(station_convert(rb.clone()));
    }

    return jss;
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
