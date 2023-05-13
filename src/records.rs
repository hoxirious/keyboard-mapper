use core::time;
use std::thread;
use rdev::{Event, EventType, Key, simulate, SimulateError};
use serde::Deserialize;
use crate::{SPECIAL_KEY_LIST, MAPPER};

#[derive(Deserialize)]
pub struct EventTypeMap {
    pub key: Vec<Key>,
    pub value: Vec<EventType>
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
                },

                // Otherwise, pass event to the OS
                None => {
                    return Some(event);
                },
            };
        },

        // Pass release events to the OS
        EventType::KeyRelease(key) => {
            println!("Emitting from release: {:?}", key);
            return Some(event);
        },
        // We don't handle the rest, so pass it to the OS
        _ => { return Some(event); }
    };
}

/// Get mapped EventType if available
fn get_mapper(key_pressed: Key) -> Option<Vec<EventType>>{

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

// Simulate the event
fn emit(key_combination: Vec<EventType>) {
    if key_combination.is_empty() {
        println!("No key event to simulate!");
    }
    for event_type in key_combination.iter() {
        let delay = time::Duration::from_millis(20);
            match simulate(event_type) {
                Ok(()) => println!("success: {:?}",event_type),
                Err(SimulateError) => {
                    println!("We could not send {:?} due to {SimulateError}", event_type);
                }
            }
        thread::sleep(delay);
    }
}