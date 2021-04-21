use crate::input::PlanInput;

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

pub struct Lesson {
    pub time: u8,
    pub teacher: u8,
    pub classroom: u8,
}

#[derive(Default)]
pub struct AnnealingBuffer {
    teacher_count: u8,
    classroom_count: u8,

    can_teach: HashSet<(u8, u8)>,
    can_hold: HashSet<(u8, u8)>,

    classroom_time_map: HashMap<ClassroomTimeKey, u8>,
    teacher_time_map: HashMap<TeacherTimeKey, u8>,
    lessons: Vec<Lesson>,
}

impl AnnealingBuffer {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn is_free_teacher(&self, teacher_tk: TeacherTimeKey) -> bool {
        !self.teacher_time_map.contains_key(&teacher_tk)
    }

    pub fn is_free_classroom(&self, classroom_tk: ClassroomTimeKey) -> bool {
        !self.classroom_time_map.contains_key(&classroom_tk)
    }
}
