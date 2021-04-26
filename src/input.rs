use crate::data_types::*;
use crate::illegal_state::IllegalState;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PlanInput {
    pub student_groups: Vec<StudentGroup>,
    pub teachers: Vec<Teacher>,
    pub classrooms: Vec<Classroom>,
    pub illegal_states: Vec<IllegalState>,
    pub subjects: Vec<Subject>,
    pub days: u8,
}

impl PlanInput {
    pub fn new(
        student_groups: Vec<StudentGroup>,
        teachers: Vec<Teacher>,
        classrooms: Vec<Classroom>,
        illegal_states: Vec<IllegalState>,
        subjects: Vec<Subject>,
        days: u8,
    ) -> Self {
        Self {
            student_groups,
            teachers,
            classrooms,
            illegal_states,
            subjects,
            days,
        }
    }

    pub fn find_subject(&self, name: &str) -> Option<&Subject> {
        self.subjects.iter().filter(|x| x.name == name).next()
    }

    pub fn possible_lessons(&self) -> Option<Vec<LessonPossible>> {
        let mut result = Vec::new();
        for student_group in self.student_groups.iter() {
            for subject_name in student_group.subjects.iter() {
                let subject = self.find_subject(subject_name)?;
                for _ in 0..subject.count {
                    result.push(LessonPossible::new(
                        student_group,
                        subject,
                        &self.teachers,
                        &self.classrooms,
                    ));
                }
            }
        }
        Some(result)
    }
}
