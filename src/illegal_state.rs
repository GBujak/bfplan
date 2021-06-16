use crate::{annealing::inner_state::Lesson, data_types::SimpleDate};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum IllegalStateSubject {
    StudentGroup(String),
    Teacher(String),
    Classroom(String),
}

#[derive(Serialize, Deserialize)]
pub enum IllegalStateObject {
    StudentGroup(String),
    Teacher(String),
    Day(u8),
    DayHour(SimpleDate),
    Classroom(String),
}

#[derive(Serialize, Deserialize)]
pub struct IllegalState {
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

    pub fn is_violated_by(&self, lesson: Lesson) -> bool {
        todo!();

        // let contains_subject = match self.subject {
        //     IllegalStateSubject::StudentGroup(_) => todo!(),
        //     IllegalStateSubject::Teacher(_) => todo!(),
        //     IllegalStateSubject::Classroom(_) => todo!(),
        // };

        let simple_date = SimpleDate::from_u8_time(lesson.time);
        false
    }
}
