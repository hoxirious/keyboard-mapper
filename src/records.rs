use rdev::{Event, EventType, Key};

use crate::{errors::EventRecordErrors, actions::Action};


#[derive(Debug)]
pub struct EventRecord {
   pub key_pressed_record: Vec<EventType>,
   pub key_released_record: Vec<EventType>,
}

impl EventRecord {
    /// Return List of keys if success.
    pub fn get_combination(&mut self) -> Result<Vec<EventType>, EventRecordErrors>{
        let mut combination: Vec<EventType> = Vec::new();
            for i in 0..self.key_pressed_record.len() {
                    // Push valid key to combination
                    combination.push(self.key_pressed_record[i]);
            }
            match combination.len() > 0 {
                true => {
                    self.reset_records();
                    Ok(combination)
                },
                false => {
                    self.reset_records();
                    Err(EventRecordErrors::GetCombinationError)
                },
            }
    }

    /// Only record a key that is not being pressed or a key that is already released.
    pub fn on_key_pressed(&mut self, event: Event) {
        let is_given_event_key_pressing = self.key_pressed_record.contains(&event.event_type);
        let should_listen_mode: bool = self.should_listen_mode();
        match !is_given_event_key_pressing && should_listen_mode {
            true => self.key_pressed_record.push(event.event_type),
            false => {},
        }
    }

    /// `should_listen_mode` will reinforce the bool check for `on_key_pressed()`\
    /// Revent from creating combination of non-special keys and support special key combination
    pub fn should_listen_mode(&mut self) -> bool {
        let pressed_record_len = self.key_pressed_record.len();
        if pressed_record_len == 0 {
            return true;
        }
        // key pressed record must have value to create combination
        // And the present values must be special key
        else if pressed_record_len > 0 {
            let first_key_pressed = self.key_pressed_record[0].get_event_type_value();
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
            match special_key_list.contains(&first_key_pressed) {
                true => return true,
                false => return false,
            }
        }
        false
    }

    /// `on_key_released` triggered means the end of the combination records.
    /// This function will call `get_combination()` and reflect with the mapper list
    /// to emit a corresponding action.
    pub fn on_key_released(&mut self) {
        if !self.key_pressed_record.is_empty() {
            let combination_result = self.get_combination();
            match combination_result {
                Ok(combination) => {
                    Action::new(combination);
                },
                Err(error) => println!("{:?}", error),
            }
        }
    }

    /// Reset after combination has been extracted
    pub fn reset_records(&mut self) {
        self.key_pressed_record.clear();
        self.key_released_record.clear();
    }
}