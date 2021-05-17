use super::{
    annealing_state::AnnealingState,
    energy::{BufferStatistics, EnergyWeights},
    illegal_buffer::IllegalBuffer,
    mutation::*,
};
use std::{
    collections::{HashMap, HashSet},
    fmt::Pointer,
    mem::swap,
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

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
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

impl std::fmt::Debug for AnnealingBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnnealingBuffer")
            .field("lessons", &self.lessons)
            .field("classroom_time_map", &self.classroom_time_map)
            .field("teacher_time_map", &self.teacher_time_map)
            .finish()
    }
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

        self.classroom_time_map
            .insert(ClassroomTimeKey { classroom, time }, lesson);
        self.teacher_time_map
            .insert(TeacherTimeKey { teacher, time }, lesson);

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
            (Some(a), Some(b)) => {
                if *a == *b { Collision(*a) } else { TooComplex }
            },
            (Some(x), None) => Collision(*x),
            (None, Some(x)) => Collision(*x),
            (None, None) => NoCollision,
        }
    }

    fn remove_from_maps(&mut self, lesson: Lesson) {
        let Lesson {
            time,
            teacher,
            classroom,
            ..
        } = lesson;
        self.teacher_time_map
            .remove(&TeacherTimeKey { teacher, time });
        self.classroom_time_map
            .remove(&ClassroomTimeKey { classroom, time });
    }

    fn insert_into_maps(&mut self, lesson: Lesson, lesson_id: u8) {
        let Lesson {
            classroom,
            time,
            teacher,
            ..
        } = lesson;
        assert_eq!(
            None,
            self.classroom_time_map
                .insert(ClassroomTimeKey { classroom, time }, lesson_id)
        );
        assert_eq!(
            None,
            self.teacher_time_map
                .insert(TeacherTimeKey { teacher, time }, lesson_id)
        );
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
        self.assert_maps_synchronized("before swap in time");

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

        self.assert_maps_synchronized(&format!("After swap in time {} and {}: \nfirst: {:?}, \nsecond: {:?}, \ntimemap_first: teacher {:?}, classroom {:?}, \ntimemap_second: teacher {:?}, classroom {:?}",
                                               first_lesson_index, second_lesson_index,
                                               self.lessons[first_lesson_index as usize], self.lessons[second_lesson_index as usize],
                                               self.teacher_time_map.get(&TeacherTimeKey{teacher: self.lessons[first_lesson_index as usize].teacher, time: self.lessons[first_lesson_index as usize].time}).unwrap(),
                                               self.classroom_time_map.get(&ClassroomTimeKey{classroom: self.lessons[first_lesson_index as usize].classroom, time: self.lessons[first_lesson_index as usize].time}).unwrap(),
                                               self.teacher_time_map.get(&TeacherTimeKey{teacher: self.lessons[second_lesson_index as usize].teacher, time: self.lessons[second_lesson_index as usize].time}).unwrap(),
                                               self.classroom_time_map.get(&ClassroomTimeKey{classroom: self.lessons[second_lesson_index as usize].classroom, time: self.lessons[second_lesson_index as usize].time}).unwrap(),
                                               ));
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

        self.assert_maps_synchronized("Pre check");

        let mut change_time_msg = "";

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
                    self.teacher_time_map.insert(
                        TeacherTimeKey {
                            teacher: lesson.teacher,
                            time: self.lessons[swap_with as usize].time,
                        },
                        swap_with,
                    );
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
                    self.classroom_time_map.insert(
                        ClassroomTimeKey {
                            classroom: lesson.classroom,
                            time: self.lessons[swap_with as usize].time,
                        },
                        swap_with,
                    );
                    result = OkCollidedWithLesson(swap_with);
                };
                self.lessons[target_lesson as usize].classroom = new_classroom;
            }

            ChangeTime(new_time) => {
                match self.check_collision(new_time, lesson.classroom, lesson.teacher) {
                    NoCollision => {
                        change_time_msg = "NoCollision";
                        self.move_lesson_in_time_no_check(target_lesson, new_time);
                        result = OkNoCollisions;
                    }
                    Collision(swap_with_index) => {
                        change_time_msg = "Collision";
                        let swap_with = self.lessons[swap_with_index as usize];
                        let rec_collision = self.check_collision(
                            lesson.time,
                            swap_with.classroom,
                            swap_with.teacher,
                        );
                        match rec_collision {
                            Collision(l) if l == target_lesson => {}
                            TooComplex => result = AbortedTooComplex,
                            NoCollision => {} // unreachable!("NoCollision but swap_with_index ({}) collides with target_lesson ({})", swap_with_index, target_lesson),
                            Collision(other) => {} // unreachable!("Collision of {:?} should be with lesson {:?} but got different lesson: {:?}", swap_with, lesson, self.lessons[other as usize]),
                        }
                        if result != AbortedTooComplex {
                            self.swap_lessons_in_time_no_check(target_lesson, swap_with_index);
                            result = OkCollidedWithLesson(swap_with_index);
                        }
                    }
                    TooComplex => result = AbortedTooComplex,
                }
            }
        };

        self.assert_maps_synchronized(&format!(
            "Post check, mutation = {:?} {}, result = {:?}",
            &mutation, change_time_msg, result
        ));
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

    pub fn anneal_iterations(&mut self, iterations: usize, weights: &EnergyWeights) {
        let mut annealing_state = AnnealingState::new(iterations);
        let mut statistics = BufferStatistics::new();
        statistics.emplace_of_buffer(self);

        for i in 0..iterations {
            let last_energy = statistics.energy(weights);
            for i in 1..=1_000_000 {
                let mutation = Mutation::legal_of_buffer(self);
                let rev_mutation = self.apply_mutation(mutation);
                statistics.emplace_of_buffer(self);
                let new_energy = statistics.energy(weights);
                if !annealing_state.should_accept_state(last_energy, new_energy) {
                    self.apply_reverse_mutation(rev_mutation);
                } else {
                    break;
                }
                if i == 1_000_000 {
                    println!("\n1,000,000 odrzuconych mutacji, przerywam");
                    return;
                }
            }
            print!(
                "\rPrzyjęto {} mutacji, energia = {}",
                i,
                statistics.energy(weights)
            );
            annealing_state.do_step();
        }
    }

    pub fn assert_maps_synchronized(&self, debug_message: &str) {
        for (lesson_id, lesson) in self.lessons.iter().enumerate() {
            let lesson_id = lesson_id as u8;
            let Lesson {
                classroom,
                teacher,
                time,
                ..
            } = *lesson;
            let teacher_check = self.teacher_time_map.get(&TeacherTimeKey { teacher, time });
            if teacher_check != Some(&lesson_id) {
               // panic!("Maps desynchronized: lesson_id = {} lesson = {:?}, teacher_check = {:?} \nMessage: {}", lesson_id, lesson, teacher_check, debug_message);
            }
            let classroom_check = self
                .classroom_time_map
                .get(&ClassroomTimeKey { classroom, time });
            if classroom_check != Some(&lesson_id) {
                //panic!("Maps desynchronized: lesson_id = {} lesson = {:?}, classroom_check = {:?} \nMessage: {}", lesson_id, lesson, classroom_check, debug_message);
            }
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

    #[test]
    fn mutations_work() {
        use super::MutationType::*;

        let mut annealing_buffer = AnnealingBuffer::new(3, 10);
        assert!(annealing_buffer.place_lesson(0, 0, 0, 0, 0));
        assert!(annealing_buffer.place_lesson(1, 0, 0, 1, 0));

        let old_lessons = annealing_buffer.lessons.clone();
        let old_classroom = annealing_buffer.classroom_time_map.clone();
        let old_teachers = annealing_buffer.teacher_time_map.clone();

        let mutation = Mutation::new(0, ChangeTime(1));
        let rev_mut = annealing_buffer.apply_mutation(mutation);

        assert_eq!(rev_mut.get().target_lesson, 0);
        assert_eq!(rev_mut.get().mutation_type, ChangeTime(0));

        assert_eq!(annealing_buffer.teacher_time_map, {
            let mut map = HashMap::new();
            map.insert(
                TeacherTimeKey {
                    teacher: 0,
                    time: 1,
                },
                0,
            );
            map.insert(
                TeacherTimeKey {
                    teacher: 0,
                    time: 0,
                },
                1,
            );
            map
        });

        assert_eq!(annealing_buffer.classroom_time_map, {
            let mut map = HashMap::new();
            map.insert(
                ClassroomTimeKey {
                    classroom: 0,
                    time: 1,
                },
                0,
            );
            map.insert(
                ClassroomTimeKey {
                    classroom: 0,
                    time: 0,
                },
                1,
            );
            map
        });

        annealing_buffer.apply_reverse_mutation(rev_mut);

        // Czy działa cofanie mutacji
        assert_eq!(old_lessons, annealing_buffer.lessons);
        assert_eq!(old_teachers, annealing_buffer.teacher_time_map);
        assert_eq!(old_classroom, annealing_buffer.classroom_time_map);
    }
}
