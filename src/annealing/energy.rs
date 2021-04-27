use std::collections::HashMap;

use super::annealing_buffer::AnnealingBuffer;

struct SGroupDay {
    pub group: u8,
    pub day: u8,
}

struct TeacherDay {
    pub teacher: u8,
    pub day: u8,
}

pub struct EnergyWeights {
    student_gap_weight: f32,
    teacher_gap_weight: f32,
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
        self.clear();

        let mut lesson_buffer = buffer.lessons.clone();
        lesson_buffer.sort_by_key(|l| l.time);

        for lesson in lesson_buffer.iter() {
            if let Some(last_group_lesson) =
                self.last_lesson_of_group.insert(lesson.group, lesson.time)
            {
                let gap = lesson.time - last_group_lesson;
                self.group_gaps += gap;
            }

            if let Some(last_teacher_lesson) = self
                .last_lesson_of_teacher
                .insert(lesson.teacher, lesson.time)
            {
                let gap = lesson.time - last_teacher_lesson;
                self.teacher_gaps += gap;
            }
        }
    }

    pub fn energy(&self, energy_weights: &EnergyWeights) -> f32 {
        let group_gap_energy = self.group_gaps as f32 * energy_weights.student_gap_weight;
        let student_gap_energy = self.teacher_gaps as f32 * energy_weights.teacher_gap_weight;
        group_gap_energy + student_gap_energy
    }

    pub fn clear(&mut self) {
        self.last_lesson_of_teacher.clear();
        self.last_lesson_of_group.clear();
        self.group_lessons_in_day.clear();
        self.teacher_lessons_in_day.clear();
        self.group_gaps = 0;
        self.teacher_gaps = 0;
    }
}
