use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StudentGroup {
    pub name: String,
    pub size: i32,
    pub subjects: Vec<String>,
}
