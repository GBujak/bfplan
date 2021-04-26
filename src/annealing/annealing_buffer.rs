use crate::{data_types::Classroom, input::PlanInput};

use super::mutation::*;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ClassroomTimeKey {
    pub classroom: u8,
    pub time: u8,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct TeacherTimeKey {
    pub teacher: u8,
    pub time: u8,
}

#[derive(Default, Clone, Copy)]
pub struct Lesson {
    pub time: u8,
    pub teacher: u8,
    pub classroom: u8,
    pub group: u8,
}

#[derive(Hash, Eq, PartialEq)]
pub struct CanTeach {
    pub teacher_id: u8,
    pub subject_id: u8,
}

#[derive(Hash, PartialEq, Eq)]
pub struct CanHold {
    pub classroom_id: u8,
    pub subject_id: u8,
}

#[derive(Default)]
pub struct AnnealingBuffer {
    pub teacher_count: u8,
    pub classroom_count: u8,

    pub can_teach: HashSet<CanTeach>,
    pub can_hold: HashSet<CanHold>,

    pub classroom_time_map: HashMap<ClassroomTimeKey, u8>,
    pub teacher_time_map: HashMap<TeacherTimeKey, u8>,
    pub lessons: Vec<Lesson>,
}

impl AnnealingBuffer {
    pub fn new(number_of_lessons: usize) -> Self {
        Self {
            lessons: vec![Default::default(); number_of_lessons],
            ..Default::default()
        }
    }

    pub fn place_lesson(
        &mut self,
        lesson: u8,
        teacher: u8,
        classroom: u8,
        time: u8,
        group: u8,
    ) -> bool {
        assert!(
            self.lessons.len() > lesson as usize,
            "Lesson buffer is shorter than lesson id"
        );

        if self
            .classroom_time_map
            .contains_key(&ClassroomTimeKey { classroom, time })
        {
            return false;
        }

        if self
            .teacher_time_map
            .contains_key(&TeacherTimeKey { teacher, time })
        {
            return false;
        }

        let lesson_ref = self.lessons.get_mut(lesson as usize);
        *lesson_ref.unwrap() = Lesson {
            classroom,
            teacher,
            time,
            group,
        };

        true
    }
}
