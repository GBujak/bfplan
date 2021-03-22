use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum SubjectType {
    Laboratorium,
    Cwiczenia,
    Wyklad,
    Projekt,
}

#[derive(Serialize, Deserialize)]
pub struct Subject {
    pub name: String,
    pub subject_type: SubjectType,
    pub count: i8, // Ilość zajęć tego typu w zjeździe
}

impl Subject {
    pub fn new(name: String, subject_type: SubjectType, count: i8) -> Self {
        Self {
            name,
            subject_type,
            count,
        }
    }
}
