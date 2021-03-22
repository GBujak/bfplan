use crate::data_types::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct PlanInput {
    pub student_groups: Vec<StudentGroup>,
    pub teachers: Vec<Teacher>,
    pub classrooms: Vec<Classroom>,
}
