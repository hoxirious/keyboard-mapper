use std::fmt::{Display, self};

#[derive(Debug)]
pub enum EventRecordErrors {
    GetCombinationError,
}

impl Display for EventRecordErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "There is an error: ({}). Please check if the key combination has been mapped", self)
    }
}