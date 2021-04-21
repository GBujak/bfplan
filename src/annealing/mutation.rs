use super::annealing_buffer::Lesson;

pub enum MutationType {
    ChangeTeacher(u8),
    ChangeTime(u8),
    ChangeClassroom(u8),
}

use MutationType::*;

pub struct Mutation {
    pub target_lesson: u8,
    pub mutation_type: MutationType,
}

impl Mutation {
    pub fn new(target_lesson: u8, mutation_type: MutationType) -> Self {
        Self {
            target_lesson,
            mutation_type,
        }
    }

    // Stwórz mutację, której wykonanie przywróci stan do stanu przed wykonaniem
    // mutacji `self`
    pub fn reverse_mutation(&self, applied_to_id: u8, applied_to: &Lesson) -> Self {
        Self {
            target_lesson: applied_to_id,
            mutation_type: match self.mutation_type {
                ChangeTeacher(_) => ChangeTeacher(applied_to.teacher),
                ChangeTime(_) => ChangeTime(applied_to.time),
                ChangeClassroom(_) => ChangeClassroom(applied_to.classroom),
            },
        }
    }
}
