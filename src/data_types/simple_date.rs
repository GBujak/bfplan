use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SimpleDate {
    pub day: i8, // Dzień od początku zjazdu
    pub hour: i8,
}

impl SimpleDate {
    pub fn new(day: i8, hour: i8) -> Self {
        assert!(hour < 23, "Niepoprawna godzina");
        assert!(
            hour % 2 == 0,
            "Początek zajęć musi być o parzystej godzinie"
        );
        Self { day, hour }
    }
}
