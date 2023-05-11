use core::time;
use std::{thread};
use rdev::{Event, EventType, Key, simulate, SimulateError};
use serde::Deserialize;
use crate::{SPECIAL_KEY_LIST, MAPPER};

#[derive(Deserialize)]
pub struct EventTypeMap {
    pub key: Vec<Key>,
    pub value: Vec<EventType>
}
pub fn process_event(event: Event) {
    match event.event_type {
        EventType::KeyPress(key) => {

            let pressing = get_mapper(key);
            let pressing = match pressing {
                Some(mapper) => mapper,
                None => {
                    let mut pressing: Vec<EventType> = Vec::new();
                    for spk in SPECIAL_KEY_LIST.lock().unwrap().iter() {
                        if spk.1.to_owned() {
                            pressing.push(EventType::KeyPress(spk.0.to_owned()));
                        }
                    }
                    return pressing.push(EventType::KeyPress(key));
                },
            };
            println!("Emitting from press: {:?}", pressing);
            emit(pressing);
        },
        EventType::KeyRelease(key) => {
            println!("Emitting from release: {:?}", key);
            emit(vec![EventType::KeyRelease(key)]);
        },
        _ => {}
    };
}

fn get_mapper(key_pressed: Key) -> Option<Vec<EventType>>{

    let mut key = Vec::<Key>::new();
    for spk in SPECIAL_KEY_LIST.lock().unwrap().iter() {
        if spk.1.to_owned() {
            key.push(spk.0.to_owned());
        }
    }

    key.push(key_pressed);

    let key = serde_json::to_string(&key).unwrap();
    println!("Combination found: {key}");
    match MAPPER.lock().unwrap().get(&key) {
        Some(mapper) => Some(mapper.to_vec()),
        None => None,
    }
}

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
        // Let OS catchup (at least MacOS)
        thread::sleep(delay);
    }
}
