use super::{
    annealing_buffer::{AnnealingBuffer, Lesson},
    illegal_buffer::IllegalBuffer,
};
use rand::random;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MutationType {
    ChangeTeacher(u8),
    ChangeTime(u8),
    ChangeClassroom(u8),
}

use MutationType::*;

#[derive(Clone, Copy, Debug)]
pub struct Mutation {
    pub target_lesson: usize,
    pub mutation_type: MutationType,
}

#[derive(Clone, Copy, Debug)]
pub struct ReverseMutation(Mutation);

impl ReverseMutation {
    pub fn of(mutation: Mutation) -> Self {
        Self(mutation)
    }

    pub fn get(self) -> Mutation {
        self.0
    }
}

impl Mutation {
    pub fn new(target_lesson: usize, mutation_type: MutationType) -> Self {
        Self {
            target_lesson,
            mutation_type,
        }
    }

    pub fn legal_of_buffer(buffer: &AnnealingBuffer) -> Mutation {
        let mut target_lesson: usize;
        let mut mutation_type: MutationType;
        let state_ref = buffer.inner_state.state_ref();

        loop {
            target_lesson = random::<usize>() % state_ref.lessons.len();

            // random::<f32>() mieści się w przedziale [0, 1)
            // 50% szansy na zmianę terminu
            // 30% szansy na zmianę sali
            // 20% szansy na zmianę prowadzącego
            mutation_type = match random::<f32>() {
                r if r < 0.5 => ChangeTime(random::<u8>() % buffer.max_time),
                r if r < 0.8 => ChangeClassroom(random::<u8>() % buffer.classroom_count),
                _r => ChangeTeacher(random::<u8>() % buffer.classroom_count),
            };

            if false {
                // TODO: sprawdzaj, czy wygenerowany stan znajduje się w zbiorze
                // niedozwolonych
                continue;
            }

            break;
        }

        Mutation {
            target_lesson,
            mutation_type,
        }
    }

    // Stwórz mutację, której wykonanie przywróci stan do stanu przed wykonaniem
    // mutacji `self`
    pub fn reverse_mutation(
        &self,
        applied_to_id: usize,
        previous_lesson_state: Lesson,
    ) -> ReverseMutation {
        ReverseMutation::of(Self {
            target_lesson: applied_to_id,
            mutation_type: match self.mutation_type {
                ChangeTeacher(_) => ChangeTeacher(previous_lesson_state.teacher),
                ChangeTime(_) => ChangeTime(previous_lesson_state.time),
                ChangeClassroom(_) => ChangeClassroom(previous_lesson_state.classroom),
            },
        })
    }
}
