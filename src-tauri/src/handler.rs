use std::io::BufWriter;

use rdev::EventType;

// This function add new command to the json list
pub fn add_new_command(key_event: Vec<EventType> ): std::io::Result<()> {
    let mut writer = BufWriter::new(File::create("../maplist.json").unwrap());
    serde_json::to_writer_pretty(&mut writer, &key_event).unwrap();
    writer.flush().unwrap();
    Ok(())
}

// This function converts string of command from
// client to event list
pub fn create_event(command: String) -> Vec<EventType> {
}
