use crate::{annealing::inner_state::Lesson, data_types::SimpleDate};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum IllegalStateSubject {
    StudentGroup(u8),
    Teacher(u8),
    Classroom(u8),
}

#[derive(Serialize, Deserialize)]
pub enum IllegalStateObject {
    StudentGroup(u8),
    Teacher(u8),
    Day(u8),
    DayHour(SimpleDate),
    Classroom(u8),
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
        let contains_subject = match self.subject {
            IllegalStateSubject::StudentGroup(x) if lesson.group == x => true,
            IllegalStateSubject::Teacher(x) if lesson.teacher == x => true,
            IllegalStateSubject::Classroom(x) if lesson.classroom == x => true,
            _ => false,
        };

        let contains_object = match self.object {
            IllegalStateObject::Day(x) if SimpleDate::from_u8_time(lesson.time).day == x => true,
            IllegalStateObject::DayHour(x) if SimpleDate::from_u8_time(lesson.time) == x => true,
            IllegalStateObject::Teacher(x) if lesson.teacher == x => true,
            IllegalStateObject::Classroom(x) if lesson.classroom == x => true,
            IllegalStateObject::StudentGroup(x) if lesson.group == x => true,
            _ => false,
        };

        (contains_subject, contains_object) == (true, true)
    }
}
