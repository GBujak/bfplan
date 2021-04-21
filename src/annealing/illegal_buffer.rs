use std::collections::HashSet;

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub struct CanTeach {
    pub lesson_id: u8,
    pub teacher_id: u8,
}

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub struct CanHold {
    pub classroom_id: u8,
    pub teacher_id: u8,
}

pub struct IllegalBuffer {
    can_teach: HashSet<CanTeach>,
    can_hold: HashSet<CanHold>,
}

impl IllegalBuffer {
    pub fn new(can_teach: HashSet<CanTeach>, can_hold: HashSet<CanHold>) -> Self {
        Self {
            can_teach,
            can_hold,
        }
    }

    pub fn can_teach(&self, can_teach: CanTeach) -> bool {
        self.can_teach.contains(&can_teach)
    }

    pub fn can_hold(&self, can_hold: CanHold) -> bool {
        self.can_hold.contains(&can_hold)
    }

    pub fn insert_can_teach(&mut self, can_teach: CanTeach) -> bool {
        self.can_teach.insert(can_teach)
    }

    pub fn insert_can_hold(&mut self, can_hold: CanHold) -> bool {
        self.can_hold.insert(can_hold)
    }
}
