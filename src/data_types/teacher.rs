use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Teacher {
    pub name: String,
    pub can_teach: Vec<String>,
}

impl Teacher {
    pub fn new(name: String, can_teach: Vec<String>) -> Self {
        Self { name, can_teach }
    }
}
