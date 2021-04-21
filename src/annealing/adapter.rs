use std::collections::HashMap;

use crate::{
    data_types::{Classroom, StudentGroup, Subject, Teacher},
    input::PlanInput,
};

#[derive(Default)]
pub struct AnnealingAdapter<'a> {
    subject_info: HashMap<&'a str, SubjectInfo<'a>>,
    lesson_info: Vec<LessonInfo<'a>>,
    plan_input: Option<&'a PlanInput>,
}

#[derive(Default)]
struct SubjectInfo<'a> {
    pub can_teach: Vec<&'a Teacher>,
    pub can_hold: Vec<&'a Classroom>,
}

struct LessonInfo<'a> {
    pub student_group: &'a StudentGroup,
    pub subject_name: &'a str,
}

impl<'a> AnnealingAdapter<'a> {
    pub fn of_plan_input(plan_input: &'a PlanInput) -> Self {
        let mut result: Self = Default::default();
        result.plan_input = Some(plan_input);

        // Generuj informacje o przedmiotach
        for subj in &plan_input.subjects {
            let can_teach = plan_input
                .teachers
                .iter()
                .filter(|x| x.can_teach.contains(&subj.name))
                .collect::<Vec<_>>();

            let can_hold = plan_input.classrooms.iter().collect::<Vec<_>>(); // TODO: Filtruj po pojemności

            result.subject_info.insert(
                subj.name.as_ref(),
                SubjectInfo {
                    can_teach,
                    can_hold,
                },
            );
        }

        // Generuj informacje o zajęciach
        for student_group in &plan_input.student_groups {
            for subject_name in &student_group.subjects {
                result.lesson_info.push(LessonInfo {
                    student_group,
                    subject_name,
                })
            }
        }

        result
    }
}
