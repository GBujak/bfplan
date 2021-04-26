use std::collections::HashMap;

use super::annealing_buffer::{AnnealingBuffer, Lesson};

struct SGroupDay {
    pub group: u8,
    pub day: u8,
}

struct TeacherDay {
    pub teacher: u8,
    pub day: u8,
}

#[derive(Default)]
pub struct BufferStatistics {
    last_lesson_of_group: HashMap<u8, u8>,
    group_gaps: u8,
    last_lesson_of_teacher: HashMap<u8, u8>,
    teacher_gaps: u8,

    group_lessons_in_day: HashMap<SGroupDay, u8>,
    teacher_lessons_in_day: HashMap<TeacherDay, u8>,
}

impl BufferStatistics {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn emplace_of_buffer(&mut self, buffer: &AnnealingBuffer) {
        for lesson in 0..buffer.lessons.len() {
            let Lesson {
                time,
                teacher,
                classroom,
                group,
            } = buffer.lessons[lesson];
            todo!()
        }
    }

    pub fn energy(&self) -> u32 {
        todo!()
    }
}
