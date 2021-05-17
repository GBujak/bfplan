use crate::data_types::SimpleDate;
use serde::Serialize;

#[derive(Serialize)]
pub struct LessonOwned {
    pub group: String,
    pub teacher: String,
    pub classroom: String,
    pub subject_name: String,
    pub time: SimpleDate,
}

#[derive(Serialize)]
pub struct PlanOutput {
    lessons: Vec<LessonOwned>,
}

impl PlanOutput {
    pub fn new() -> Self {
        Self {
            lessons: Vec::new(),
        }
    }

    pub fn push_lesson(&mut self, lesson_owned: LessonOwned) {
        self.lessons.push(lesson_owned);
    }

    pub fn len(&self) -> usize {
        self.lessons.len()
    }
}
