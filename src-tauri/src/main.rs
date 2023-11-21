// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::records::{process_event, EventTypeMap};
use rdev::{grab, grab_t, Event, EventType, Key};
use records::get_keybind;
use std::collections::HashMap;
use std::sync::Mutex;
use std::{fs, thread};
#[macro_use]
extern crate lazy_static;

mod records;

fn load_db() -> Mutex<HashMap<String, Vec<EventType>>> {
    Mutex::new({
        let mut m = HashMap::new();
        let data = fs::read_to_string("./maplist.json").expect("Unable to read file");
        let map_list: Vec<EventTypeMap> =
            serde_json::from_str(&data).expect("JSON does not have correct format.");

        for each in map_list.iter() {
            let key_struct = serde_json::to_string(&each.key).unwrap();
            let value = each.value.to_owned();
            m.insert(key_struct, value);
        }
        m
    })
}

lazy_static! {
    static ref IN_MEMORY_KEYBIND: Mutex<String> = Mutex::new(String::new());
    static ref MAPPER: Mutex<HashMap<String, Vec<EventType>>> = load_db();
    static ref SPECIAL_KEY_LIST: Mutex<HashMap<Key, bool>> = Mutex::new({
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
#[tauri::command(async)]
fn start_mapping() {
    start();
}

#[tauri::command(async)]
async fn save_db(db: String) {
    fs::write("./maplist.json", &db).expect("Unable to write file");
    // Reload the maplist
    MAPPER.lock().unwrap().clear();
    MAPPER.lock().unwrap().extend(load_db().lock().unwrap().to_owned());
}

#[tauri::command(async)]
async fn record() -> String {
    let handler = thread::spawn(move || {
        record_keybind();
    });
    handler.join().unwrap();
    println!("Record Successfully");
    IN_MEMORY_KEYBIND.lock().unwrap().to_owned()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_mapping, save_db, record])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn start() {
    // This will block.
    if let Err(err) = grab(
        //test_infinity_loop
        move |event: Event| match event.event_type {
            EventType::KeyPress(key) => {
                let is_special_key = SPECIAL_KEY_LIST.lock().unwrap().contains_key(&key);
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
            EventType::ButtonPress(_) => {
                println!("Mouse button pressed");
                return None;
            }
            _ => {
                return Some(event);
            }
        },
    ) {
        println!("grab listen error: {:?}", err);
    };
}

fn record_keybind() {
    if let Err(err) = grab_t(
        // Exit once none
        move |event: Event| match event.event_type {
            EventType::KeyPress(key) => {
                let is_special_key = SPECIAL_KEY_LIST.lock().unwrap().contains_key(&key);
                if is_special_key {
                    SPECIAL_KEY_LIST.lock().unwrap().insert(key, true);
                    return Some(event);
                }

                return get_keybind(event.to_owned());
            }
            EventType::KeyRelease(key) => {
                let is_special_key = SPECIAL_KEY_LIST.lock().unwrap().contains_key(&key);
                if is_special_key {
                    SPECIAL_KEY_LIST.lock().unwrap().insert(key, false);
                    return Some(event);
                }

                return get_keybind(event.to_owned());
            }
            _ => {
                return Some(event);
            }
        },
    ) {
        println!("grab listen error: {:?}", err);
    };
}
