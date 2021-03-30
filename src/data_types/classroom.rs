use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Classroom {
    pub name: String,
    pub capacity: i32,
}
