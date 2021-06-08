use super::{annealing_state::AnnealingState, energy::{BufferStatistics, EnergyWeights}, illegal_buffer::IllegalBuffer, inner_state::{InnerCollision, InnerState}, mutation::*};

use std::collections::{HashMap, HashSet};

pub use super::inner_state::{ClassroomTimeKey, Lesson, TeacherTimeKey};

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

#[derive(Default)]
pub struct AnnealingBuffer {
    pub teacher_count: u8,
    pub classroom_count: u8,
    pub max_time: u8,

    pub can_teach: HashSet<CanTeach>,
    pub can_hold: HashSet<CanHold>,

    pub inner_state: InnerState,
}

impl std::fmt::Debug for AnnealingBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnnealingBuffer")
            .field("InnerState", &self.inner_state)
            .finish()
    }
}

impl AnnealingBuffer {
    pub fn new(number_of_lessons: usize, max_time: u8) -> Self {
        Self {
            max_time,
            inner_state: InnerState::new(number_of_lessons),
            ..Default::default()
        }
    }

    pub fn place_lesson(
        &mut self,
        lesson: usize,
        teacher: u8,
        classroom: u8,
        time: u8,
        group: u8,
    ) -> bool {
        self.inner_state
            .place_lesson(lesson, teacher, classroom, time, group)
    }

    fn apply_mutation(&mut self, mutation: Mutation) -> ReverseMutation {
        let previous_lesson_state = self.inner_state.state_ref().lessons[mutation.target_lesson];
        let rev_mutation = mutation.reverse_mutation(previous_lesson_state);
        self.inner_state.apply_mutation(mutation);
        rev_mutation
    }

    fn apply_reverse_mutation(&mut self, reverse_mutation: ReverseMutation) {
        self.inner_state.apply_mutation(reverse_mutation.get());
    }

    pub fn anneal_iterations(&mut self, iterations: usize, weights: &EnergyWeights) {
        let mut annealing_state = AnnealingState::new(iterations);
        let mut statistics = BufferStatistics::new();
        statistics.emplace_of_buffer(self);
        let mut rejected = 0_f64;

        for i in 0..iterations {
            let last_energy = statistics.energy(weights);
            for j in 1..=1_000_000 {
                let mutation = Mutation::legal_of_buffer(self);
                let rev_mutation = self.apply_mutation(mutation);
                statistics.emplace_of_buffer(self);
                let new_energy = statistics.energy(weights);
                if !annealing_state.should_accept_state(last_energy, new_energy) {
                    self.apply_reverse_mutation(rev_mutation);
                    rejected += 1.0;
                } else {
                    break;
                }
                if j == 1_000_000 {
                    println!("\n1,000,000 odrzuconych mutacji, przerywam");
                    return;
                }
            }
            print!(
                "\rPrzyjęto {} mutacji, energia = {}, średnio odrzucone na przyjęte: {}   ",
                i + 1,
                statistics.energy(weights),
                rejected / i as f64
            );
            self.assert_maps_synchronized("After mutation accepted");
            annealing_state.do_step();
        }
    }

    pub fn assert_maps_synchronized(&self, msg: &str) {
        self.inner_state.assert_maps_synchronized(msg);
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
