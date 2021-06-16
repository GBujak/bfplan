use std::collections::HashMap;

use crate::data_types::SimpleDate;

use super::annealing_buffer::AnnealingBuffer;

#[derive(PartialEq, Eq, Hash)]
struct GroupDay {
    pub group: u8,
    pub day: u8,
}

#[derive(PartialEq, Eq, Hash)]
struct TeacherDay {
    pub teacher: u8,
    pub day: u8,
}

pub struct EnergyWeights {
    pub student_gap_weight: f32,
    pub teacher_gap_weight: f32,
    pub teacher_lessons_in_day_weight: f32,
    pub group_lessons_in_day_weight: f32,
}

#[derive(Default)]
pub struct BufferStatistics {
    last_lesson_of_group: HashMap<u8, u8>,
    group_gaps: u8,
    last_lesson_of_teacher: HashMap<u8, u8>,
    teacher_gaps: u8,

    group_lessons_in_day: HashMap<GroupDay, u8>,
    teacher_lessons_in_day: HashMap<TeacherDay, u8>,

    max_day: u8,
    max_group: u8,
    max_teacher: u8,
}

impl BufferStatistics {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn emplace_of_buffer(&mut self, buffer: &AnnealingBuffer) {
        self.clear();

        let mut lesson_buffer = buffer.inner_state.state_ref().lessons.clone();
        lesson_buffer.sort_by_key(|l| l.time);

        for lesson in lesson_buffer.iter() {
            // Lekcje w ciągu dnia
            {
                let teacher = lesson.teacher;
                let group = lesson.group;
                let day = SimpleDate::from_u8_time(lesson.time).day;

                self.max_day = self.max_day.max(day);
                self.max_group = self.max_group.max(group);
                self.max_teacher = self.max_teacher.max(teacher);

                if let Some(current) = self
                    .teacher_lessons_in_day
                    .remove(&TeacherDay { teacher, day })
                {
                    self.teacher_lessons_in_day
                        .insert(TeacherDay { teacher, day }, current + 1);
                } else {
                    self.teacher_lessons_in_day
                        .insert(TeacherDay { teacher, day }, 1);
                }

                if let Some(current) = self.group_lessons_in_day.remove(&GroupDay { group, day }) {
                    self.group_lessons_in_day
                        .insert(GroupDay { group, day }, current + 1);
                } else {
                    self.group_lessons_in_day.insert(GroupDay { group, day }, 1);
                }
            }

            // Okienka studentów
            if let Some(last_group_lesson) =
                self.last_lesson_of_group.insert(lesson.group, lesson.time)
            {
                // Okienka liczą się tylko w tym samym dniu
                if SimpleDate::from_u8_time(last_group_lesson).day
                    == SimpleDate::from_u8_time(lesson.time).day
                {
                    let gap = lesson.time - last_group_lesson;
                    self.group_gaps += gap;
                }
            }

            // Okienka nauczycieli
            if let Some(last_teacher_lesson) = self
                .last_lesson_of_teacher
                .insert(lesson.teacher, lesson.time)
            {
                // Okienka liczą się tylko w tym samym dniu
                if SimpleDate::from_u8_time(last_teacher_lesson).day
                    == SimpleDate::from_u8_time(lesson.time).day
                {
                    let gap = lesson.time - last_teacher_lesson;
                    self.teacher_gaps += gap;
                }
            }
        }
    }

    fn gaps_energy(&self) -> (f32, f32) {
        let mut group_energy = 0.0;
        let mut teacher_energy = 0.0;

        for day in 0..self.max_day {
            for group in 0..self.max_group {
                if let Some(lessons) = self.group_lessons_in_day.get(&GroupDay { group, day }) {
                    let diff_from_perfect = (4.0 - *lessons as f32).abs();
                    group_energy += diff_from_perfect;
                }
            }
            for teacher in 0..self.max_teacher {
                if let Some(lessons) = self
                    .teacher_lessons_in_day
                    .get(&TeacherDay { teacher, day })
                {
                    let diff_from_perfect = (4.0 - *lessons as f32).abs();
                    teacher_energy += diff_from_perfect;
                }
            }
        }

        (group_energy, teacher_energy)
    }

    pub fn energy(&self, energy_weights: &EnergyWeights) -> f32 {
        let group_gap_energy = self.group_gaps as f32 * energy_weights.student_gap_weight;
        let student_gap_energy = self.teacher_gaps as f32 * energy_weights.teacher_gap_weight;
        let (raw_group_lessons_in_day_energy, raw_teacher_lessons_in_day_energy) =
            self.gaps_energy();

        group_gap_energy
            + student_gap_energy
            + raw_group_lessons_in_day_energy * energy_weights.teacher_lessons_in_day_weight
            + raw_teacher_lessons_in_day_energy * energy_weights.teacher_lessons_in_day_weight
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
