use std::collections::HashMap;

use super::mutation::{Mutation, MutationType};

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

#[derive(Debug)]
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

    pub fn check_collision(&self, lesson: Lesson, lesson_id: usize) -> InnerCollision {
        let mut result = InnerCollision::NoCollisions;
        for col_option in &self.collision_checks(lesson) {
            if let Some(col) = *col_option {
                match result {
                    InnerCollision::NoCollisions => {
                        // Lekcja nie koliduje sama z sobą.
                        if *col != lesson_id {
                            result = InnerCollision::CollidesWithOne(*col);
                        }
                    }
                    InnerCollision::CollidesWithOne(previous_collision) => {
                        // Podwójna kolizja z tą samą lekcją nie powoduje problemów.
                        // Lekcja nie koliduje sama z sobą.
                        if previous_collision != *col && *col != lesson_id {
                            result = InnerCollision::TooComplex;
                        }
                    }
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

    // Używane spoza tego modułu przez AnnealingAdapter
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
            "Lesson buffer is shorter {} than lesson id {}",
            self.lessons.len(), lesson_id
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

    pub fn assert_maps_synchronized(&self, msg: &str) {
        for (lesson_id, lesson) in self.lessons.iter().enumerate() {
            assert_eq!(
                self.collision_checks(*lesson),
                [Some(&lesson_id); 3],
                "Maps not synchronized ({})\n{:?}",
                msg,
                self.state_ref()
            );
        }
    }

    fn remove_lesson(&mut self, lesson_id: usize) {
        let lesson = self.lessons[lesson_id];

        let removed = [
            self.classroom_time.remove(&lesson.classroom_time_key()),
            self.teacher_time.remove(&lesson.teacher_time_key()),
            self.group_time.remove(&lesson.group_time_key()),
        ];

        assert_eq!(
            removed,
            [Some(lesson_id); 3],
            "Unexpected lesson when removing, expected {}",
            lesson_id
        );
    }

    fn replace_lessons(
        &mut self,
        left_id: usize,
        left_new_state: Lesson,
        right_id: usize,
        right_new_state: Lesson,
    ) {
        self.remove_lesson(left_id);
        self.remove_lesson(right_id);

        self.put_lesson(left_new_state, left_id);
        self.put_lesson(right_new_state, right_id);
    }

    fn apply_non_time_mutation(&mut self, mutation: Mutation) -> bool {
        let target_lesson = mutation.target_lesson;
        let lesson = self.lessons[target_lesson];
        let changed_lesson = match mutation.mutation_type {
            MutationType::ChangeTeacher(new_teacher) => lesson.with_teacher(new_teacher),
            MutationType::ChangeClassroom(new_classroom) => lesson.with_classroom(new_classroom),
            _ => unreachable!(),
        };

        let collision = self.check_collision(changed_lesson, mutation.target_lesson);

        match collision {
            InnerCollision::NoCollisions => {
                self.remove_lesson(target_lesson);
                self.put_lesson(changed_lesson, target_lesson);
            }
            InnerCollision::CollidesWithOne(collision_id) => {
                let collision_old_state = self.lessons[collision_id];
                let collision_new_state = match mutation.mutation_type {
                    MutationType::ChangeTeacher(_) => {
                        collision_old_state.with_teacher(lesson.teacher)
                    }
                    MutationType::ChangeClassroom(_) => {
                        collision_old_state.with_classroom(lesson.classroom)
                    }
                    _ => unreachable!(),
                };
                self.replace_lessons(
                    target_lesson,
                    changed_lesson,
                    collision_id,
                    collision_new_state,
                );
            }
            InnerCollision::TooComplex => return false,
        }
        true
    }

    fn apply_time_mutation(&mut self, mutation: Mutation) -> bool {
        let target_lesson = mutation.target_lesson;
        let lesson_old_state = self.lessons[target_lesson];
        let new_time = match mutation.mutation_type {
            MutationType::ChangeTime(time) => time,
            _ => unreachable!(),
        };

        let lesson_new_state = lesson_old_state.with_time(new_time);

        let collision = self.check_collision(lesson_new_state, target_lesson);

        match collision {
            InnerCollision::NoCollisions => {
                self.remove_lesson(target_lesson);
                self.put_lesson(lesson_new_state, target_lesson);
            }
            InnerCollision::CollidesWithOne(collision_id) => {
                let collision_old_state = self.lessons[collision_id];
                let collision_new_state = collision_old_state.with_time(lesson_old_state.time);

                let recursive_collision = self.check_collision(collision_new_state, collision_id);
                if let InnerCollision::TooComplex = recursive_collision {
                    return false;
                }

                // Kolizja rekurencyjna musi wynosić CollidesWithOne(target_lesson) albo TooComplex.
                // Jeśli kolizja jest równa NoCollisions, jest to błąd programu.
                assert_eq!(
                    recursive_collision,
                    InnerCollision::CollidesWithOne(target_lesson),
                    "Recursive collision is not with the original lesson"
                );

                self.remove_lesson(target_lesson);
                self.remove_lesson(collision_id);

                self.put_lesson(lesson_new_state, target_lesson);
                self.put_lesson(collision_new_state, collision_id);
            }
            InnerCollision::TooComplex => return false,
        };
        true
    }

    pub fn apply_mutation(&mut self, mutation: Mutation) -> bool {
        match &mutation.mutation_type {
            &MutationType::ChangeTime(_) => self.apply_time_mutation(mutation),
            _ => self.apply_non_time_mutation(mutation),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum InnerCollision {
    NoCollisions,
    CollidesWithOne(usize),
    TooComplex,
}

impl InnerCollision {
    pub fn is_no_collisions(&self) -> bool {
        if let Self::NoCollisions = self {
            true
        } else {
            false
        }
    }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn time_mutation_works() {
        let mut inner_state = InnerState::new(2);
        assert_eq!(inner_state.place_lesson(0, 0, 0, 0, 0), true);
        assert_eq!(inner_state.place_lesson(1, 0, 0, 0, 0), false);
        assert_eq!(inner_state.place_lesson(1, 0, 0, 1, 0), true);

        let mutation = Mutation::new(0, MutationType::ChangeTime(1));
        let rev_mutation = mutation.reverse_mutation(inner_state.lessons[0]);

        assert_eq!(inner_state.apply_mutation(mutation), true);

        assert_eq!(inner_state.state_ref().lessons[0].time, 1);
        assert_eq!(inner_state.state_ref().lessons[1].time, 0);

        assert_eq!(inner_state.apply_mutation(rev_mutation.get()), true);

        assert_eq!(inner_state.state_ref().lessons[0].time, 0);
        assert_eq!(inner_state.state_ref().lessons[1].time, 1);
    }
}
