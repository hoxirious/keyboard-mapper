use crate::{MAPPER, SPECIAL_KEY_LIST};
use core::time;
use rdev::{simulate, Event, EventType, Key, SimulateError};
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::io::ErrorKind;
use std::thread;

#[derive(Deserialize, Serialize)]
pub struct EventTypeMap {
    pub key: Vec<Key>,
    pub value: Vec<EventType>,
}
pub fn process_event(event: Event) -> Option<Event> {
    match event.event_type {
        EventType::KeyPress(key) => {
            let pressing = get_mapper(key);
            match pressing {
                // If mapper found the pressed hotkey, simulate it and do not pass event to the OS
                Some(mapper) => {
                    emit(mapper);
                    return None;
                }

                // Otherwise, pass event to the OS
                None => {
                    return Some(event);
                }
            };
        }

        // Pass release events to the OS
        EventType::KeyRelease(key) => {
            println!("Emitting from release: {:?}", key);
            return Some(event);
        }
        // We don't handle the rest, so pass it to the OS
        _ => {
            return Some(event);
        }
    };
}

fn read_from_map() -> Vec<EventTypeMap> {
    let f = File::open("./maplist.json");

    let _ = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("./maplist.json") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            _ => panic!("Problem opening the file: {:?}", error),
        },
    };

    let data = fs::read_to_string("./maplist.json").expect("Unable to read file");

    let mut mapper: Vec<EventTypeMap> = Vec::new();
    if fs::metadata("./maplist.json").unwrap().len() != 0 {
        mapper = match serde_json::from_str(&data) {
            Ok(it) => it,
            Err(err) => panic!("Problem parsing the file: {:?}", err),
        };
    }
    mapper
}

fn write_to_map(mapper: Vec<EventTypeMap>) {
    let json: String = serde_json::to_string(&mapper).expect("Unable to serialize");

    fs::write("./maplist.json", &json).expect("Unable to write file");
}

pub fn save_keybind(event: Event) -> Option<Event> {
    match event.event_type {
        EventType::KeyPress(key) => {
            let mut mapper = read_from_map();
            let combination = get_combination(key);
            mapper.push(combination);

            write_to_map(mapper);

            return None;
        }
        _ => {
            return None;
        }
    };
}

/// Get mapped EventType if available
fn get_mapper(key_pressed: Key) -> Option<Vec<EventType>> {
    let mut key = Vec::<Key>::new();
    // Iterating through special key list
    // If value of a key is true, that means that special key has been pressed
    for spk in SPECIAL_KEY_LIST.lock().unwrap().iter() {
        if spk.1.to_owned() {
            key.push(spk.0.to_owned());
        }
    }
    key.push(key_pressed);

    // Serialize key - ready to search
    let key = serde_json::to_string(&key).unwrap();
    println!("Combination found: {key}");

    // Retrieve from json
    match MAPPER.lock().unwrap().get(&key) {
        Some(mapper) => Some(mapper.to_vec()),
        None => None,
    }
}

fn get_combination(key_pressed: Key) -> EventTypeMap {
    let mut key = Vec::<Key>::new();
    // Iterating through special key list
    // If value of a key is true, that means that special key has been pressed
    for spk in SPECIAL_KEY_LIST.lock().unwrap().iter() {
        if spk.1.to_owned() {
            key.push(spk.0.to_owned());
        }
    }
    key.push(key_pressed);

    // Generate vector of key event
    // like this
    // "value": [{"KeyPress":"ControlLeft"},{"KeyPress":"KeyC"},{"KeyRelease":"KeyC"},{"KeyRelease":"ControlLeft"}]

    let combination = EventTypeMap {
        key: key.clone(),
        value: generate_value(&key, &key),
    };

    combination
}

fn generate_value(map_from: &Vec<Key>, map_to: &Vec<Key>) -> Vec<EventType> {
    let mut combination = Vec::<EventType>::new();
    for k in map_from {
        let event = EventType::KeyPress(k.to_owned());
        combination.push(event);
    }

    for k in map_to {
        let event = EventType::KeyRelease(k.to_owned());
        combination.push(event);
    }
    combination
}

fn update_value(map_key: String, replacement: Key) {
    let mut key = Vec::<Key>::new();
    // Iterating through special key list
    // If value of a key is true, that means that special key has been pressed
    for spk in SPECIAL_KEY_LIST.lock().unwrap().iter() {
        if spk.1.to_owned() {
            key.push(spk.0.to_owned());
        }
    }
    key.push(replacement);

    let mut mapper = read_from_map();

    for m in &mut mapper {
        let each = serde_json::to_string(&m.key).unwrap();
        if each == map_key {
            m.value = generate_value(&m.key, &key);
        }
    }

    write_to_map(mapper);
}

// Simulate the event
fn emit(key_combination: Vec<EventType>) {
    if key_combination.is_empty() {
        println!("No key event to simulate!");
    }
    for event_type in key_combination.iter() {
        let delay = time::Duration::from_millis(20);
        match simulate(event_type) {
            Ok(()) => println!("success: {:?}", event_type),
            Err(SimulateError) => {
                println!("We could not send {:?} due to {SimulateError}", event_type);
            }
        }
        thread::sleep(delay);
    }
}
