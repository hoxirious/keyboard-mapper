use std::{time, thread};

use rdev::{EventType, simulate, SimulateError, Event};
use serde::{Deserialize, Serialize};
use crate::MAPPER;

#[derive(Serialize, Deserialize)]
pub struct Action {
    pub key_combination: Vec<Event>,
    pub key_mapper: Vec<EventType>
}

impl Action {
    /// `new` will search for the corresponding mapper, and create an Action instance
    pub fn new(key_combination: Vec<Event>) -> Self {
        let key_mapper = get_key_mapper(&key_combination);
        Action {
            key_combination,
            key_mapper
        }
    }

    /// After `new` is called to construct the Action struct,
    /// `emit` will simulate the corresponding key combination
    pub fn emit(&mut self) {
        if self.key_mapper.is_empty() {
            println!("No key event to simulate!");
        }

        for event_type in self.key_mapper.iter() {
            let delay = time::Duration::from_millis(50);
                match simulate(event_type) {
                Ok(()) => println!("success: "),
                Err(SimulateError) => {
                    println!("We could not send {:?}", event_type);
                }
            }
            // Let ths OS catchup (at least MacOS)
            thread::sleep(delay);
        }
        self.reset_records();
    }

    pub fn reset_records(&mut self) {
        self.key_combination.clear();
        self.key_mapper.clear();
    }
}

/// Search for the mapper of the given input `key_combination` from the static hashmap
fn get_key_mapper(key_combination: &Vec<Event>) -> Vec<EventType> {

    let mut event_type_list: Vec<EventType> = Vec::new();

    for key in key_combination.iter() {
        event_type_list.push(key.event_type.to_owned());
    }
    let key_string: String = {
        serde_json::to_string(&event_type_list).unwrap()
    };

    let binding = MAPPER.lock().unwrap();
    let value = binding.get(&key_string);

    match value {
        Some(value) => {
            println!("{:?}", value);
            value.to_vec()
        },
        None => {
            println!("{:?}", event_type_list);
            event_type_list
        }
    }
}
