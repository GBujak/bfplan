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

pub enum ApplyMutationResult {
    OkNoCollisions,
    OkCollidedWithLesson(u8),
    AbortedTooComplex,
}

pub enum CollisionCheck {
    NoCollision,
    Collision(u8),
    TooComplex,
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

        self.classroom_time_map.insert(ClassroomTimeKey{classroom, time}, lesson);
        self.teacher_time_map.insert(TeacherTimeKey{teacher, time}, lesson);

        let lesson_ref = self.lessons.get_mut(lesson as usize);
        *lesson_ref.unwrap() = Lesson {
            classroom,
            teacher,
            time,
            group,
        };

        true
    }

    pub fn check_collision(&self, time: u8, classroom: u8, teacher: u8) -> CollisionCheck {
        use CollisionCheck::*;

        let classroom_col = self
            .classroom_time_map
            .get(&ClassroomTimeKey { classroom, time });
        let teacher_col = self.teacher_time_map.get(&TeacherTimeKey { teacher, time });

        match (classroom_col, teacher_col) {
            (Some(_), Some(_)) => TooComplex,
            (Some(x), None) => Collision(*x),
            (None, Some(x)) => Collision(*x),
            (None, None) => NoCollision,
        }
    }

    fn move_lesson_in_time_no_check(&mut self, target_lesson: u8, new_time: u8) {
        let lesson = self.lessons[target_lesson as usize];

        self.teacher_time_map.remove(&TeacherTimeKey {
            teacher: lesson.teacher,
            time: lesson.time,
        });
        self.teacher_time_map.insert(
            TeacherTimeKey {
                teacher: lesson.teacher,
                time: new_time,
            },
            target_lesson,
        );

        self.classroom_time_map.remove(&ClassroomTimeKey {
            classroom: lesson.classroom,
            time: lesson.time,
        });
        self.classroom_time_map.insert(
            ClassroomTimeKey {
                classroom: lesson.classroom,
                time: new_time,
            },
            target_lesson,
        );

        self.lessons[target_lesson as usize].time = new_time;
    }

    fn swap_lessons_in_time_no_check(&mut self, first_lesson_index: u8, second_lesson_index: u8) {
        let first_lesson = self.lessons[first_lesson_index as usize];
        let second_lesson = self.lessons[second_lesson_index as usize];

        self.teacher_time_map.insert(
            TeacherTimeKey {
                teacher: first_lesson.teacher,
                time: second_lesson.time,
            },
            first_lesson_index,
        );
        self.classroom_time_map.insert(
            ClassroomTimeKey {
                classroom: first_lesson.classroom,
                time: second_lesson.time,
            },
            first_lesson_index,
        );

        self.teacher_time_map.insert(
            TeacherTimeKey {
                teacher: second_lesson.teacher,
                time: first_lesson.time,
            },
            second_lesson_index,
        );
        self.classroom_time_map.insert(
            ClassroomTimeKey {
                classroom: second_lesson.classroom,
                time: first_lesson.time,
            },
            second_lesson_index,
        );

        self.lessons[first_lesson_index as usize].time = second_lesson.time;
        self.lessons[second_lesson_index as usize].time = first_lesson.time;
    }

    fn apply_mutation_impl(&mut self, mutation: Mutation) -> ApplyMutationResult {
        use super::mutation::MutationType::*;
        use ApplyMutationResult::*;
        use CollisionCheck::*;

        let Mutation {
            target_lesson,
            mutation_type,
        } = mutation;

        let lesson = self.lessons[target_lesson as usize];
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
                self.lessons[target_lesson as usize].teacher = new_teacher;
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
                self.lessons[target_lesson as usize].classroom = new_classroom;
            }

            ChangeTime(new_time) => {
                match self.check_collision(new_time, lesson.classroom, lesson.teacher) {
                    NoCollision => self.move_lesson_in_time_no_check(target_lesson, new_time),
                    Collision(swap_with_index) => {
                        let swap_with = self.lessons[swap_with_index as usize];
                        let rec_collision = self.check_collision(
                            lesson.time,
                            swap_with.classroom,
                            swap_with.teacher,
                        );
                        match rec_collision {
                            Collision(l) if l == target_lesson => {}
                            TooComplex => return AbortedTooComplex,
                            NoCollision => unreachable!(
                                "NoCollision but swap_with_index collides with target_lesson"
                            ),
                            Collision(other) => unreachable!(
                                "Collision should be with target_lesson ({}) but got different lesson index: {}", target_lesson, other
                            ),
                        }
                        self.swap_lessons_in_time_no_check(target_lesson, swap_with_index);
                    }
                    TooComplex => result = AbortedTooComplex,
                }
            }
        };

        result
    }

    fn apply_mutation(&mut self, mutation: Mutation) -> ReverseMutation {
        let applied_to_id = mutation.target_lesson;
        let previous_lesson_state = self.lessons[applied_to_id as usize];
        self.apply_mutation_impl(mutation);
        mutation.reverse_mutation(applied_to_id, previous_lesson_state)
    }

    fn apply_reverse_mutation(&mut self, reverse_mutation: ReverseMutation) {
        self.apply_mutation_impl(reverse_mutation.get());
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn placing_lesson_works() {
        let mut annealing_buffer = AnnealingBuffer::new(3, 10);
        assert!(annealing_buffer.place_lesson(0, 0, 0, 0, 0));
        assert!(annealing_buffer.place_lesson(1, 0, 0, 1, 0));
        assert_eq!(false, annealing_buffer.place_lesson(2, 0, 0, 0, 0));
    }
}
