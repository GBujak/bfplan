use std::collections::HashMap;

use super::mutation::{
    Mutation, MutationType
};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct ClassroomTimeKey {
    pub classroom: u8,
    pub time: u8,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct TeacherTimeKey {
    pub teacher: u8,
    pub time: u8,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct GroupTimeKey {
    pub group: u8,
    pub time: u8,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Lesson {
    pub time: u8,
    pub teacher: u8,
    pub classroom: u8,
    pub group: u8,
}

impl Lesson {
    pub fn teacher_time_key(&self) -> TeacherTimeKey {
        TeacherTimeKey {
            teacher: self.teacher,
            time: self.time,
        }
    }

    pub fn classroom_time_key(&self) -> ClassroomTimeKey {
        ClassroomTimeKey {
            classroom: self.classroom,
            time: self.time,
        }
    }

    pub fn group_time_key(&self) -> GroupTimeKey {
        GroupTimeKey {
            group: self.group,
            time: self.time,
        }
    }

    pub fn with_teacher(&self, teacher: u8) -> Lesson {
        Lesson { teacher, ..*self }
    }

    pub fn with_classroom(&self, classroom: u8) -> Lesson {
        Lesson { classroom, ..*self }
    }

    pub fn with_time(&self, time: u8) -> Lesson {
        Lesson { time, ..*self }
    }
}

#[derive(Default)]
pub struct InnerState {
    lessons: Vec<Lesson>,

    teacher_time: HashMap<TeacherTimeKey, usize>,
    classroom_time: HashMap<ClassroomTimeKey, usize>,
    group_time: HashMap<GroupTimeKey, usize>,
}

pub struct InnerStateRef<'a> {
    pub lessons: &'a Vec<Lesson>,

    pub teacher_time: &'a HashMap<TeacherTimeKey, usize>,
    pub classroom_time: &'a HashMap<ClassroomTimeKey, usize>,
    pub group_time: &'a HashMap<GroupTimeKey, usize>,
}

impl InnerState {
    pub fn new(lesson_count: usize) -> Self {
        Self {
            lessons: vec![Default::default(); lesson_count],
            ..Default::default()
        }
    }

    pub fn state_ref(&self) -> InnerStateRef {
        InnerStateRef {
            lessons: &self.lessons,
            teacher_time: &self.teacher_time,
            classroom_time: &self.classroom_time,
            group_time: &self.group_time,
        }
    }

    fn collision_checks(&self, lesson: Lesson) -> [Option<&usize>; 3] {
        let group_collision = self.group_time.get(&lesson.group_time_key());
        let teacher_collision = self.teacher_time.get(&lesson.teacher_time_key());
        let classroom_collision = self.classroom_time.get(&lesson.classroom_time_key());
        [group_collision, teacher_collision, classroom_collision]
    }

    fn all_no_collision(&self, lesson: Lesson) -> bool {
        self.collision_checks(lesson) == [None; 3]
    }

    pub fn check_collision(&self, lesson: Lesson) -> InnerCollision {
        let mut result = InnerCollision::NoCollisions;
        for col_option in &self.collision_checks(lesson) {
            if let Some(col) = *col_option {
                match result {
                    InnerCollision::NoCollisions => result = InnerCollision::CollidesWithOne(*col),
                    InnerCollision::CollidesWithOne(_) => result = InnerCollision::TooComplex,
                    InnerCollision::TooComplex => {}
                }
            }
        }
        result
    }

    fn put_lesson(&mut self, lesson: Lesson, lesson_id: usize) {
        let insert_results = [
            self.classroom_time
                .insert(lesson.classroom_time_key(), lesson_id),
            self.teacher_time
                .insert(lesson.teacher_time_key(), lesson_id),
            self.group_time.insert(lesson.group_time_key(), lesson_id),
        ];

        self.lessons[lesson_id] = lesson;

        assert_eq!(insert_results, [None; 3]);
    }

    pub fn place_lesson(
        &mut self,
        lesson_id: usize,
        teacher: u8,
        classroom: u8,
        time: u8,
        group: u8,
    ) -> bool {
        assert!(
            self.lessons.len() > lesson_id,
            "Lesson buffer is shorter than lesson id"
        );

        let lesson = Lesson {
            teacher,
            time,
            group,
            classroom,
        };

        if !self.all_no_collision(lesson) {
            return false;
        }

        self.put_lesson(lesson, lesson_id);
        true
    }

    pub fn assert_maps_synchronized(&self) {
        for (lesson_id, lesson) in self.lessons.iter().enumerate() {
            assert_eq!(self.collision_checks(*lesson), [Some(&lesson_id); 3]);
        }
    }

    fn apply_non_time_mutation(&mut self, mutation: Mutation) {
        let lesson = self.lessons[mutation.target_lesson];
    }

    fn apply_time_mutation(&mut self, mutation: Mutation) {}

    pub fn apply_mutation(&mut self, mutation: Mutation) {
        match &mutation.mutation_type {
            &MutationType::ChangeTime(_) => self.apply_time_mutation(mutation),
            _ => self.apply_non_time_mutation(mutation),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum InnerCollision {
    NoCollisions,
    CollidesWithOne(usize),
    TooComplex,
}

impl std::fmt::Debug for InnerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InnerState")
            .field("lessons", &self.lessons)
            .field("teacher_time", &self.teacher_time)
            .field("classroom_time", &self.classroom_time)
            .field("group_time", &self.group_time)
            .finish()
    }
}
