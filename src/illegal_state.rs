use crate::data_types::SimpleDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum IllegalStateSubject {
    StudentGroup(String),
    Teacher(String),
    Classroom(String),
}

#[derive(Serialize, Deserialize)]
pub enum IllegalStateObject {
    Teacher(String),
    Day(u8),
    DayHour(SimpleDate),
    Classroom(String),
}

#[derive(Serialize, Deserialize)]
struct IllegalState {
    pub subject: IllegalStateSubject,
    pub object: IllegalStateObject,
}

impl IllegalState {
    pub fn is_logic_error(&self) -> bool {
        match &self.subject {
            &IllegalStateSubject::StudentGroup(_) => match &self.object {
                _ => return false,
            },
            &IllegalStateSubject::Teacher(_) => match &self.object {
                &IllegalStateObject::Teacher(_) => return true,
                _ => return false,
            },
            &IllegalStateSubject::Classroom(_) => match &self.object {
                &IllegalStateObject::Classroom(_) => return true,
                _ => return false,
            },
        }
    }
}
