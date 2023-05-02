use rdev::EventType;

pub struct Action {
    key_combination: Vec<EventType>,
    key_mapper: Vec<EventType>
}

impl Action {
    pub fn new(key_combination: Vec<EventType>) -> Self{
        println!("My callback {:?}", key_combination);
        let key_mapper = get_key_mapper(&key_combination);
        Action {
            key_combination,
            key_mapper
        }
    }
}

fn get_key_mapper(key_combination: &Vec<EventType>) -> Vec<EventType> {
    let a = key_combination.clone();

    return a;
}