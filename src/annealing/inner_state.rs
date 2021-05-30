use std::collections::HashMap;

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
}

#[derive(Default)]
pub struct InnerState {
    lessons: Vec<Lesson>,

    teacher_time: HashMap<TeacherTimeKey, usize>,
    classroom_time: HashMap<ClassroomTimeKey, usize>,
    group_time: HashMap<GroupTimeKey, usize>,
}

impl InnerState {
    pub fn new(lesson_count: usize) -> Self {
        Self {
            lessons: vec![Default::default(); lesson_count],
            ..Default::default()
        }
    }

    fn collision_checks(&self, lesson: Lesson) -> [Option<&usize>; 3] {
        let group_collision = self.group_time.get(&lesson.group_time_key());
        let teacher_collision = self.teacher_time.get(&lesson.teacher_time_key());
        let classroom_collision = self.classroom_time.get(&lesson.classroom_time_key());
        [group_collision, teacher_collision, classroom_collision]
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
}

#[derive(PartialEq, Eq)]
pub enum InnerCollision {
    NoCollisions,
    CollidesWithOne(usize),
    TooComplex,
}
