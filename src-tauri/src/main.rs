// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{sync::Mutex, fs};
use rdev::{Event, EventType, Key, grab};
use std::collections::HashMap;
use crate::records::{EventTypeMap, process_event};
#[macro_use]
extern crate lazy_static;

mod records;

lazy_static! {
    static ref MAPPER: Mutex<HashMap<String, Vec<EventType>>> = Mutex::new({
        let mut m = HashMap::new();
        let data = fs::read_to_string("./maplist.json").expect("Unable to read file");
        let map_list: Vec<EventTypeMap> = serde_json::from_str(&data).expect("JSON does not have correct format.");

        for each in map_list.iter() {
            let key_struct = serde_json::to_string(&each.key).unwrap();
            let value = each.value.to_owned();
            m.insert(key_struct, value);
        }
        m
    });

    static ref SPECIAL_KEY_LIST: Mutex<HashMap<Key,bool>> = Mutex::new({
        let mut m = HashMap::new();
            m.insert(Key::ControlLeft, false);
            m.insert(Key::ControlRight, false);
            m.insert(Key::ShiftLeft, false);
            m.insert(Key::ShiftRight, false);
            m.insert(Key::MetaRight, false);
            m.insert(Key::MetaLeft, false);
            m.insert(Key::Alt, false);
            m.insert(Key::AltGr, false);
        m
    });
}


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // start();
}


fn start() {
    // This will block.
    if let Err(err) = grab(
        //test_infinity_loop
        move |event: Event|
        {
            match event.event_type {
                EventType::KeyPress(key) => {
                    let is_special_key = SPECIAL_KEY_LIST.lock().unwrap().contains_key(&key) ;
                    if is_special_key {
                        SPECIAL_KEY_LIST.lock().unwrap().insert(key, true);
                        return Some(event);
                    }

                    return process_event(event.to_owned());
                }
                EventType::KeyRelease(key) => {
                    let is_special_key = SPECIAL_KEY_LIST.lock().unwrap().contains_key(&key);
                    if is_special_key {
                        SPECIAL_KEY_LIST.lock().unwrap().insert(key, false);
                        return Some(event);
                    }

                    return process_event(event.to_owned());
                }
                _ => Some(event),
        }}
    ) {
        println!("grab listen error: {:?}", err);
    };
}
