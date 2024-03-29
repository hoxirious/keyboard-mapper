// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::records::{process_map_event, EventTypeMap};
use rdev::{grab, grab_t, Event, EventType, Key};
use records::process_record_event;
use std::collections::HashMap;
use std::sync::Mutex;
use std::{fs, process, thread};
use tauri::{AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
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

fn make_tray() -> SystemTray {
    // <- a function that creates the system tray
    let menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("toggle".to_string(), "Hide"))
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));
    return SystemTray::new().with_menu(menu);
}
fn handle_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    if let SystemTrayEvent::MenuItemClick { id, .. } = event {
        if id.as_str() == "quit" {
            process::exit(0);
        }
        if id.as_str() == "toggle" {
            let window = app.get_window("main").unwrap();
            let menu_item = app.tray_handle().get_item("toggle");
            if window.is_visible().unwrap() {
                let _ = window.hide();
                let _ = menu_item.set_title("Show");
            } else {
                let _ = window.show();
                let _ = window.center();
                let _ = menu_item.set_title("Hide");
            }
        }
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command(async)]
fn start_mapper() {
    println!("Starting mapper");
    start();
}

#[tauri::command(async)]
async fn save_db(db: String) {
    fs::write("./maplist.json", &db).expect("Unable to write file");
    // Reload the maplist
    MAPPER.lock().unwrap().clear();
    MAPPER
        .lock()
        .unwrap()
        .extend(load_db().lock().unwrap().to_owned());
}

#[tauri::command(async)]
async fn record() -> String {
    let handler = thread::spawn(move || {
        record_combination();
    });
    handler.join().unwrap();
    println!("Record Successfully");
    let rt = IN_MEMORY_KEYBIND.lock().unwrap().to_owned();
    IN_MEMORY_KEYBIND.lock().unwrap().clear();
    rt
}

#[tauri::command]
fn hide_window(app: AppHandle) {
    let window = app.get_window("main").unwrap();
    let menu_item = app.tray_handle().get_item("toggle");
    let _ = window.hide();
    let _ = menu_item.set_title("Show");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            start_mapper,
            save_db,
            record,
            hide_window
        ])
        .system_tray(make_tray())
        .on_system_tray_event(handle_tray_event)
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

                return process_map_event(event.to_owned());
            }
            EventType::KeyRelease(key) => {
                let is_special_key = SPECIAL_KEY_LIST.lock().unwrap().contains_key(&key);
                if is_special_key {
                    SPECIAL_KEY_LIST.lock().unwrap().insert(key, false);
                    return Some(event);
                }

                return process_map_event(event.to_owned());
            }
            _ => {
                return Some(event);
            }
        },
    ) {
        println!("grab listen error: {:?}", err);
    };
}

fn record_combination() {
    if let Err(err) = grab_t(
        // Exit once none
        move |event: Event| match event.event_type {
            EventType::KeyPress(key) => {
                let is_special_key = SPECIAL_KEY_LIST.lock().unwrap().contains_key(&key);
                if is_special_key {
                    SPECIAL_KEY_LIST.lock().unwrap().insert(key, true);
                    return Some(event);
                }

                return process_record_event(event.to_owned());
            }
            EventType::KeyRelease(key) => {
                let is_special_key = SPECIAL_KEY_LIST.lock().unwrap().contains_key(&key);
                if is_special_key {
                    SPECIAL_KEY_LIST.lock().unwrap().insert(key, false);
                    return Some(event);
                }

                return process_record_event(event.to_owned());
            }
            _ => {
                return Some(event);
            }
        },
    ) {
        println!("grab listen error: {:?}", err);
    };
}
