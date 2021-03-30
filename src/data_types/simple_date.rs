use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SimpleDate {
    pub day: u8, // Dzień od początku zjazdu
    pub hour: u8,
}

impl SimpleDate {
    pub fn new(day: u8, hour: u8) -> Self {
        assert!(hour < 23, "Niepoprawna godzina");
        assert!(
            hour % 2 == 0,
            "Początek zajęć musi być o parzystej godzinie"
        );
        Self { day, hour }
    }
}

#[derive(Debug, Clone)]
pub struct DateList {
    // illegal_dates: HashSet<SimpleDate>,
    days: u8,
    current_day: u8,
    current_hour: u8,
}

impl DateList {
    pub fn new(/*illegal_dates: HashSet<SimpleDate>, */ days: u8) -> Self {
        Self {
            // illegal_dates,
            days,
            current_day: 0,
            current_hour: 8,
        }
    }
}

impl DateList {
    fn set_next(&mut self) {
        self.current_hour += 2;
        if self.current_hour > 20 {
            self.current_hour = 8;
            self.current_day += 1;
        }
    }

    fn is_done(&self) -> bool {
        self.current_day == self.days
    }
}

impl Iterator for DateList {
    type Item = SimpleDate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done() {
            None
        } else {
            let next = SimpleDate::new(self.current_day, self.current_hour);
            self.set_next();
            Some(next)
        }
    }
}
