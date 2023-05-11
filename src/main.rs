use std::{sync::{Mutex}, fs, collections::HashSet};
use rdev::{Event, EventType, Key, listen};
use records::EventRecord;
use std::collections::HashMap;

use crate::records::EventTypeMap;
#[macro_use]
extern crate lazy_static;

mod errors;
mod records;

lazy_static! {
    static ref RECORDS: Mutex<EventRecord> = Mutex::new(EventRecord {record: HashSet::new()});
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

// Static event records
fn main() {
    // This will block.
    if let Err(err) = listen(
        //test_infinity_loop
        move |event: Event|
        {
            match event.event_type {
                EventType::KeyPress(key) => {
                    println!("-------------detect KEY PRESSED");
                    let is_special_key = SPECIAL_KEY_LIST.lock().unwrap().contains_key(&key);
                    if is_special_key {
                        SPECIAL_KEY_LIST.lock().unwrap().insert(key, true);
                        return;
                    }
                    RECORDS.lock().unwrap().process_event(event);
                }
                EventType::KeyRelease(key) => {
                    println!("-------------detect KEY RELEASED");
                    let is_special_key = SPECIAL_KEY_LIST.lock().unwrap().contains_key(&key);
                    if is_special_key {
                        SPECIAL_KEY_LIST.lock().unwrap().insert(key, false);
                        return;
                    }
                    RECORDS.lock().unwrap().process_event(event);
                }
                _ => {},
        }}
    ) {
        println!("listen error: {:?}", err);
    };
}