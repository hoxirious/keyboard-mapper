use core::time;
use std::{collections::HashSet, thread};
use rdev::{Event, EventType, Key, simulate, SimulateError};
use serde::Deserialize;
use crate::{SPECIAL_KEY_LIST, MAPPER};

#[derive(Deserialize)]
pub struct EventTypeMap {
    pub key: Vec<Key>,
    pub value: Vec<EventType>
}
#[derive(Debug)]
pub struct EventRecord {
   pub record: HashSet<Key>,
}

impl EventRecord {
    pub fn process_event(&mut self, event: Event) {
        match event.event_type {
            EventType::KeyPress(key) => {
                self.record.insert(key);

                let pressing = EventRecord::get_mapper(key);
                let pressing = match pressing {
                    Some(mapper) => mapper,
                    None => {
                        let mut pressing: Vec<EventType> = Vec::new();
                        println!("Adding to HashSet");
                        for spk in SPECIAL_KEY_LIST.lock().unwrap().iter() {
                            if spk.1.to_owned() {
                                pressing.push(EventType::KeyPress(spk.0.to_owned()));
                            }
                        }
                        return pressing.push(EventType::KeyPress(key));
                    },
                };
                println!("Emitting {:?}", pressing);
                EventRecord::emit(pressing);
            },
            EventType::KeyRelease(key) => {
                self.record.remove(&key);
                EventRecord::emit(vec![EventType::KeyRelease(key)]);
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
        println!("{key}");
        match MAPPER.lock().unwrap().get(&key) {
            Some(mapper) => Some(mapper.to_vec()),
            None => None,
        }
    }

    pub fn emit(key_combination: Vec<EventType>) {
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
}