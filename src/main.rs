use rdev::{grab, Event, EventType};
use records::EventRecord;

mod errors;
mod records;

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