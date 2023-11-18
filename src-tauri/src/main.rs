// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::records::{process_event, EventTypeMap};
use rdev::{Event, EventType, GrabError, Key, grab};
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::{fs, thread};
use tauri::State;
#[macro_use]
extern crate lazy_static;

mod records;

lazy_static! {
    static ref MAPPER: Mutex<HashMap<String, Vec<EventType>>> = Mutex::new({
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
    });
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

struct Speaker(Arc<(Arc<Mutex<Sender<()>>>, Arc<Mutex<Receiver<()>>>)>);

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(speaker: State<Speaker>) {
    let w = speaker.0.clone();
    thread::spawn(move || {
        // let rx = w.1.lock().unwrap();
        println!("Working...");
        start();
        // match rx.try_recv() {
        //     Ok(_) | Err(TryRecvError::Disconnected) => {
        //         println!("Terminating.");
        //         break;
        //     }
        //     Err(TryRecvError::Empty) => {}
        // }
    });
}

#[tauri::command]
fn meet(speaker: State<Speaker>) -> String {
    let w = speaker.0.clone();
    let tx = w.0.lock().unwrap();
    let mut line = String::new();
    let stdin = io::stdin();
    let _ = stdin.lock().read_line(&mut line);

    let _ = tx.send(());
    "happy".to_string()
}

fn main() {
    tauri::Builder::default()
        .manage(Speaker({
            let (tx, rx) = channel::<()>();
            let tx = Arc::new(Mutex::new(tx));
            let rx = Arc::new(Mutex::new(rx));
            Arc::new((tx, rx))
        }))
        .invoke_handler(tauri::generate_handler![greet, meet])
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
