use rdev::{grab, Event, EventType, Key};
use crate::errors::EventRecordErrors;

mod errors;

#[derive(Debug)]
struct EventRecord {
   key_pressed_record: Vec<Event>,
   key_released_record: Vec<Event>,
}

impl EventRecord {
    /// Check if key_pressed_record and recordReleased match.
    /// Return List of keys if success.
    fn get_combination(&self) -> Result<Vec<EventType>, EventRecordErrors>{
        let mut combination: Vec<EventType> = Vec::new();
        
        // If the two records have the same length, proceed
        if &self.key_pressed_record.len() == &self.key_released_record.len() {
            for i in 0..self.key_pressed_record.len() {
                //Not the correct comparision

                let key_pressed = self.key_pressed_record[i].event_type.get_event_type_value();
                let key_released = self.key_released_record[i].event_type.get_event_type_value();

                // Not sure if this is right??
                if key_pressed == key_released {
                    combination.push(self.key_released_record[i].event_type);
                }
                else {
                    return Err(EventRecordErrors::EventRecordsNotValid);
                }
            }
            
            Ok(combination)
        }
        // If two records are not the same length, throw error
        else {
            return Err(EventRecordErrors::EventRecordsNotValid);
        }
    }
}

// Static event records
static mut RECORDS: EventRecord = EventRecord {key_pressed_record: Vec::new() , key_released_record: Vec::new() };

fn main() {
    // This will block.
    if let Err(error) = grab(callback) {
        println!("Error: {:?}", error)
    }
}

fn callback(event: Event) -> Option<Event>{
    println!("My callback {:?}", event);
    match event.event_type {
        EventType::KeyPress(_) => {
            unsafe { RECORDS.key_pressed_record.push(event);
                println!("This is pressed record\n: {:?}", RECORDS)
              };
            None
        }
        EventType::KeyRelease(_) => {
            unsafe { RECORDS.key_released_record.push(event);
                println!("This is released record\n: {:?}", RECORDS)
              };
            None
        }
        _ => Some(event),
    }
}