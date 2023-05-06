use std::{sync::Mutex, fs};
use rdev::{grab, Event, EventType};
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
}
// Static event records


fn main() {
    // This will block.
    if let Err(error) = grab(callback) {
        println!("Error: {:?}", error)
    }
}

fn callback(event: Event) -> Option<Event>{
    match event.event_type {
        EventType::KeyPress(_) => {
            RECORDS.lock().unwrap().on_key_pressed(event);
            None
        }
        EventType::KeyRelease(_) => {
            RECORDS.lock().unwrap().on_key_released(event);
            None
        }
        _ => Some(event),
    }
}