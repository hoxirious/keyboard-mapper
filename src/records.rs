use rdev::{Event, EventType, Key};
use serde::Deserialize;
use crate::{errors::EventRecordErrors, actions::Action};

#[derive(Deserialize)]
pub struct EventTypeMap {
    pub key: Vec<EventType>,
    pub value: Vec<EventType>
}

#[derive(Debug)]
pub struct EventRecord {
   pub key_pressed_record: Vec<Event>,
   pub key_released_record: Vec<Event>,
}

impl EventRecord {
    /// Return List of keys if success.
    pub fn get_combination(&mut self) -> Result<Vec<Event>, EventRecordErrors>{

            match self.key_pressed_record.len() > 0 {
                true => {
                    Ok(self.key_pressed_record.to_owned())
                }
                false => {
                    Err(EventRecordErrors::GetCombinationError)
                },
            }
    }
    pub fn is_key_pressing(&self, event_type: EventType) -> bool {
        match event_type {
            EventType::KeyPress(_) => {
                for event in self.key_pressed_record.iter() {
                    if event.event_type == event_type {
                        return true;
                    }
                }
                false
            },
            EventType::KeyRelease(_) => {
                for event in self.key_released_record.iter() {
                    if event.event_type == event_type {
                        return true;
                    }
                }
                false
            },
            _ => todo!(),
        }
    }
    /**
     * When a key is pressed: two types of key
     *  1. Special keys
     *  2. Non-special keys
     *
     *  - Special key means combination is coming -> we need to record it to map the combination **Only record one time
     *  - Non-special keys means:
     *      + if no special keys in front, it is single action so no record needed -> emit
     *      + otherwise -> record -> emit -> reset
     */
    pub fn on_key_pressed(&mut self, event: Event) {
        let special_key_list = vec![
            EventType::KeyPress(Key::Alt).get_event_type_value(),
            EventType::KeyPress(Key::ControlLeft).get_event_type_value(),
            EventType::KeyPress(Key::ControlRight).get_event_type_value(),
            EventType::KeyPress(Key::AltGr).get_event_type_value(),
            EventType::KeyPress(Key::MetaLeft).get_event_type_value(),
            EventType::KeyPress(Key::MetaRight).get_event_type_value(),
            EventType::KeyPress(Key::ShiftLeft).get_event_type_value(),
            EventType::KeyPress(Key::ShiftRight).get_event_type_value(),
        ];
        
        // Which type of key is this.
        let is_special_key = special_key_list.contains(&event.event_type.get_event_type_value());
        let is_key_pressing = self.is_key_pressing(event.event_type);

        match is_special_key {
            true => {
                println!("{:?}", self.key_pressed_record);
                if !is_key_pressing { self.key_pressed_record.push(event);}
            },
            false => {
                // if key_pressed_record > 0, it means that a special key has been pushed
                match self.key_pressed_record.len() > 0 {
                    true => {
                        self.key_pressed_record.push(event);
                        let combination_result = self.get_combination();
                        match combination_result {
                            Ok(combination) => {
                                let action = Action::new(combination);
                                action.emit();
                            },
                                Err(error) => println!("{:?}", error),
                            }
                            self.reset_records();
                        },
                    false => {
                        let action = Action::new(vec![event]);
                        action.emit();
                    },
                }
            },
        }
    }

    /// `on_key_released` triggered means the end of the combination records.
    /// This function will call `get_combination()` and reflect with the mapper list
    /// to emit a corresponding action.
    pub fn on_key_released(&mut self, event: Event) {
        //if !self.key_pressed_record.is_empty() {
            // self.key_released_record.push(event);
            // if self.key_pressed_record.len() == self.key_released_record.len() {
            //     let combination_result = self.get_combination();
            //     match combination_result {
            //         Ok(combination) => {
            //             let action = Action::new(combination);
            //             action.emit();

            //     },
            //         Err(error) => println!("{:?}", error),
            //     }
            // }
        //}
        println!("on_key_released: {:?}", event.event_type);
        let action = Action::new(vec![event]);
        action.emit();
        self.reset_records();
    }

    /// Reset after combination has been extracted
    pub fn reset_records(&mut self) {
        self.key_pressed_record.clear();
        self.key_released_record.clear();
    }
}