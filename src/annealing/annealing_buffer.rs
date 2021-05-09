use crate::{
    data_types::{Classroom, Teacher},
    input::PlanInput,
};

use super::mutation::*;
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

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

pub enum ApplyMutationResult {
    OkNoCollisions,
    OkCollidedWithLesson(u8),
    AbortedTooComplex,
}

#[derive(Default)]
pub struct AnnealingBuffer {
    pub teacher_count: u8,
    pub classroom_count: u8,
    pub max_time: u8,

    pub can_teach: HashSet<CanTeach>,
    pub can_hold: HashSet<CanHold>,

    pub classroom_time_map: HashMap<ClassroomTimeKey, u8>,
    pub teacher_time_map: HashMap<TeacherTimeKey, u8>,
    pub lessons: Vec<Lesson>,
}

impl AnnealingBuffer {
    pub fn new(number_of_lessons: usize, max_time: u8) -> Self {
        Self {
            max_time,
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

    fn apply_mutation_impl(&mut self, mutation: Mutation) -> ApplyMutationResult {
        use super::mutation::MutationType::*;
        use ApplyMutationResult::*;

        let Mutation {
            target_lesson,
            mutation_type,
        } = mutation;

        let mut lesson = *self.lessons.get(target_lesson as usize).unwrap();
        let mut result = OkNoCollisions;

        match mutation_type {
            ChangeTeacher(new_teacher) => {
                if let Some(swap_with) = self.teacher_time_map.insert(
                    TeacherTimeKey {
                        teacher: new_teacher,
                        time: lesson.time,
                    },
                    target_lesson,
                ) {
                    self.lessons[swap_with as usize].teacher = lesson.teacher;
                    result = OkCollidedWithLesson(swap_with);
                };
                lesson.teacher = new_teacher;
            }
            ChangeClassroom(new_classroom) => {
                if let Some(swap_with) = self.classroom_time_map.insert(
                    ClassroomTimeKey {
                        classroom: new_classroom,
                        time: lesson.time,
                    },
                    target_lesson,
                ) {
                    self.lessons[swap_with as usize].classroom = lesson.classroom;
                    result = OkCollidedWithLesson(swap_with);
                };
                lesson.classroom = new_classroom;
            }
            ChangeTime(new_time) => {
                let classroom_collides = self.classroom_time_map.contains_key(&ClassroomTimeKey {
                    classroom: lesson.classroom,
                    time: new_time,
                });
                let teacher_collides = self.teacher_time_map.contains_key(&TeacherTimeKey {
                    teacher: lesson.teacher,
                    time: new_time,
                });

                if classroom_collides && teacher_collides {
                    return AbortedTooComplex;
                }

                if let Some(swap_with) = self.classroom_time_map.insert(
                    ClassroomTimeKey {
                        classroom: lesson.classroom,
                        time: new_time,
                    },
                    target_lesson,
                ) {}
            }
        };

        result
    }

    fn apply_mutation(&mut self, mutation: Mutation) -> ReverseMutation {
        todo!()
    }

    fn apply_reverse_mutation(&mut self, reverse_mutation: ReverseMutation) {
        todo!()
    }

    fn anneal_step(&mut self) {
        todo!()
    }

    pub fn anneal_iterations(&mut self, iterations: usize) {
        for _ in 0..iterations {
            self.anneal_step();
        }
    }
}
