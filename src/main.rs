use std::{sync::Mutex, fs};
use rdev::{Event, EventType, Key, listen};
use records::EventRecord;
use std::collections::HashMap;

use crate::records::EventTypeMap;
#[macro_use]
extern crate lazy_static;

mod errors;
mod records;
mod actions;

lazy_static! {
    static ref RECORDS: Mutex<EventRecord> = Mutex::new(EventRecord {key_pressed_record: Vec::new() , key_released_record: Vec::new() });
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

    static ref SPECIAL_KEY_LIST: Mutex<Vec<Key>> = Mutex::new({
        let sp = vec![
            Key::Alt,
            Key::ControlLeft,
            Key::ControlRight,
            Key::AltGr,
            Key::MetaLeft,
            Key::MetaRight,
            Key::ShiftLeft,
            Key::ShiftRight,
        ];
        sp
    });
}

// Static event records
fn main() {
    // This will block.
    std::env::set_var("KEYBOARD_ONLY", "y");
    if let Err(err) = listen(move |event: Event|
        {
            match event.event_type {
                EventType::KeyPress(_) => {
                println!("-------------hello wolrd");
                RECORDS.lock().unwrap().on_key_pressed(event);
            }
            EventType::KeyRelease(_) => {
                RECORDS.lock().unwrap().on_key_released(event);
            }
            _ => {},
        }}) {
        println!("start grab listen error: {:?}", err);
    };
}
