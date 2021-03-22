use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Classroom {
    pub name: String,
    pub capacity: i32,
}
