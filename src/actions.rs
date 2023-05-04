use std::{time, thread};

use rdev::{EventType, simulate, SimulateError};
use serde::{Deserialize, Serialize};
use crate::MAPPER;

#[derive(Serialize, Deserialize)]
pub struct Action {
    pub key_combination: Vec<EventType>,
    pub key_mapper: Vec<EventType>
}

impl Action {
    /// `new` will search for the corresponding mapper, and create an Action instance
    pub fn new(key_combination: Vec<EventType>) -> Self{
        let key_mapper = get_key_mapper(&key_combination);
        Action {
            key_combination,
            key_mapper
        }
    }

    /// After `new` is called to construct the Action struct,
    /// `emit` will simulate the corresponding key combination
    pub fn emit(&self) {
        if !self.key_mapper.is_empty() {
            for event_type in self.key_mapper.iter() {
                let delay = time::Duration::from_millis(20);
                    match simulate(event_type) {
                    Ok(()) => println!("success"),
                    Err(SimulateError) => {
                        println!("We could not send {:?}", event_type);
                    }
                }
                // Let ths OS catchup (at least MacOS)
                thread::sleep(delay);
            }
        }
        else {
            println!("The key combination has not been mapped!")
        }
    }
}

/// Search for the mapper of the given input `key_combination` from the static hashmap
fn get_key_mapper(key_combination: &Vec<EventType>) -> Vec<EventType> {
    let key_string: String = serde_json::to_string(key_combination).unwrap();

    let binding = MAPPER.lock().unwrap();
    let value = binding.get(&key_string);

    match value {
        Some(value) => {
            println!("{:?}", value);
            value.to_vec()
        },
        None => todo!(),
    }
}