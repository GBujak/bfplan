use std::collections::HashMap;

use crate::{
    data_types::{Classroom, StudentGroup, Subject, Teacher},
    input::PlanInput,
};

use super::annealing_buffer::AnnealingBuffer;

#[derive(Default)]
pub struct AnnealingAdapter<'a> {
    subject_info: HashMap<&'a str, SubjectInfo<'a>>,
    lesson_info: Vec<LessonInfo<'a>>,
    plan_input: Option<&'a PlanInput>,
}

#[derive(Default)]
struct SubjectInfo<'a> {
    pub can_teach: Vec<&'a Teacher>,
    pub can_hold: Vec<&'a Classroom>,
}

struct LessonInfo<'a> {
    pub student_group: &'a StudentGroup,
    pub subject_name: &'a str,
}

impl<'a> AnnealingAdapter<'a> {
    pub fn of_plan_input(plan_input: &'a PlanInput) -> Self {
        let mut result: Self = Default::default();
        result.plan_input = Some(plan_input);

        // Generuj informacje o przedmiotach
        for subj in &plan_input.subjects {
            let can_teach = plan_input
                .teachers
                .iter()
                .filter(|x| x.can_teach.contains(&subj.name))
                .collect::<Vec<_>>();

            let can_hold = plan_input.classrooms.iter().collect::<Vec<_>>(); // TODO: Filtruj po pojemności

            result.subject_info.insert(
                subj.name.as_ref(),
                SubjectInfo {
                    can_teach,
                    can_hold,
                },
            );
        }

        // Generuj informacje o zajęciach
        for student_group in &plan_input.student_groups {
            for subject_name in &student_group.subjects {
                result.lesson_info.push(LessonInfo {
                    student_group,
                    subject_name,
                })
            }
        }

        result
    }

    pub fn create_annealing_buffer(&self) -> AnnealingBuffer {
        let plan_input = self.plan_input.unwrap();

        let lesson_count = plan_input
            .student_groups
            .iter()
            .map(|x| x.subjects.len())
            .reduce(|a, b| a + b)
            .unwrap();

        let max_time = plan_input.days * 6; // od 0 = 8:00 do 5 = 18:00
        let mut buffer = AnnealingBuffer::new(lesson_count, max_time);

        let mut lesson_index: u8 = 0;

        for (group_index, _group) in plan_input.student_groups.iter().enumerate() {
            'lesson: for _subject in &plan_input.subjects {
                for time in 0..max_time {
                    for (teacher_index, _teacher) in plan_input.teachers.iter().enumerate() {
                        for (classroom_index, _classroom) in
                            plan_input.classrooms.iter().enumerate()
                        {
                            // TODO: sprawdzaj czy lekcja może się odbyć w sali
                            if buffer.place_lesson(
                                lesson_index,
                                teacher_index as u8,
                                classroom_index as u8,
                                time,
                                group_index as u8,
                            ) {
                                lesson_index += 1;
                                continue 'lesson;
                            }
                        }
                    }
                }
            }
        }

        buffer.max_time = max_time;
        buffer.classroom_count = plan_input.classrooms.len() as u8;

        buffer
    }
}
