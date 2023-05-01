use rdev::{Event, EventType, Key};

use crate::errors::EventRecordErrors;


#[derive(Debug)]
pub struct EventRecord {
   pub key_pressed_record: Vec<Event>,
   pub key_released_record: Vec<Event>,
}

impl EventRecord {
    /// Check if key_pressed_record and recordReleased match.
    /// Return List of keys if success.
    pub fn get_combination(&mut self) -> Result<Vec<EventType>, EventRecordErrors>{
        let mut combination: Vec<EventType> = Vec::new();

        // If the two records have the same length, proceed
        if self.key_pressed_record.len() > 0 && self.key_pressed_record.len() == self.key_released_record.len() {
            for i in 0..self.key_pressed_record.len() {

                // Extract Key that is attached from EventType enum
                let key_pressed = self.key_pressed_record[i].event_type.get_event_type_value();
                let key_released = self.key_released_record[i].event_type.get_event_type_value();

                // Apply trait PartialEq to compare two structs
                if key_pressed == key_released {

                    // Push valid key to combination
                    combination.push(self.key_released_record[i].event_type);
                }
                else {
                    self.reset_records();
                    return Err(EventRecordErrors::EventRecordsNotValid);
                }
            }

            self.reset_records();
            Ok(combination)
        }
        // If two records are not the same length, throw error
        else {
            self.reset_records();
            return Err(EventRecordErrors::EventRecordsNotValid);
        }
    }

    /// Only record a key that is not being pressed or a key that is already released.
    pub fn on_key_pressed(&mut self, event: Event) {
        let is_key_pressing = self.key_pressed_record.contains(&event);
        let should_listen_mode = self.should_listen_mode();

        match is_key_pressing && should_listen_mode {
            true => self.key_pressed_record.push(event),
            false => {},
        }
    }

    pub fn should_listen_mode(&mut self) -> bool{

        let pressed_record_len = self.key_pressed_record.len();
        if pressed_record_len > 0 {
            let first_key_pressed = self.key_pressed_record[0].event_type.get_event_type_value();
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

    /// End of recording, get combination, emit corresponding action
    pub fn on_key_released(&mut self, event: Event) {
    }

    /// Reset after combination has been extracted
    pub fn reset_records(&mut self) {
        self.key_pressed_record.clear();
        self.key_released_record.clear();
    }
}