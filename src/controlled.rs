use crate::position::{Pos, ShiftDir};
use std::time::{Duration, Instant};

pub trait CheckField {
    fn is_open(&self, pos: Pos) -> bool;
}

pub struct ControlledBlocks {
    root_pos: Pos,
    relative_poses: [Pos; 4],
    next_drop_time: Instant,
}

const DROP_PERIOD: Duration = Duration::from_millis(1000);

pub enum DropResult {
    Continue,
    Stop,
}

impl ControlledBlocks {
    pub fn new() -> ControlledBlocks {
        ControlledBlocks {
            root_pos: Pos::new(0, 0),
            relative_poses: [
                Pos::new(0, 0),
                Pos::new(1, 0),
                Pos::new(2, 0),
                Pos::new(3, 0),
            ],
            next_drop_time: Instant::now() + DROP_PERIOD,
        }
    }

    pub fn positions(&self) -> [Pos; 4] {
        let mut result = self.relative_poses;
        for pos in result.iter_mut() {
            *pos = *pos + self.root_pos;
        }
        result
    }

    pub fn is_controlled(&self, target: Pos) -> bool {
        for relative in self.relative_poses.iter() {
            if self.root_pos + *relative == target {
                return true;
            }
        }
        return false;
    }

    pub fn shift(&mut self, field: &CheckField, dir: ShiftDir) {
        // Don't move if it's not legal
        for pos in self.relative_poses.iter() {
            if !field.is_open(self.root_pos + *pos + dir) {
                return;
            }
        }
        self.root_pos = self.root_pos + dir;
    }

    pub fn maybe_periodic_drop(&mut self, field: &CheckField) -> DropResult {
        if self.next_drop_time > Instant::now() {
            return DropResult::Continue;
        }
        self.next_drop_time += DROP_PERIOD;
        self.soft_drop(field)
    }

    pub fn manual_soft_drop(&mut self, field: &CheckField) -> DropResult {
        self.next_drop_time = Instant::now() + DROP_PERIOD;
        self.soft_drop(field)
    }

    fn soft_drop(&mut self, field: &CheckField) -> DropResult {
        let delta = Pos::new(0, 1);
        for pos in self.relative_poses.iter() {
            if !field.is_open(self.root_pos + *pos + delta) {
                return DropResult::Stop;
            }
        }
        self.root_pos = self.root_pos + delta;
        DropResult::Continue
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mock_trait!(MockCheckField, is_open(Pos) -> bool);
    impl CheckField for MockCheckField {
        mock_method!(is_open(&self, pos: Pos) -> bool);
    }

    #[test]
    fn controlled() {
        let b = ControlledBlocks::new();
        assert_eq!(true, b.is_controlled(Pos::new(0, 0)));
        assert_eq!(false, b.is_controlled(Pos::new(0, 1)));
    }

    #[test]
    fn shift() {
        let mock_field = MockCheckField::default();
        mock_field.is_open.return_value_for(Pos::new(5, 0), false);
        mock_field.is_open.return_value(true);
        
        let mut b = ControlledBlocks::new();
        b.shift(&mock_field, ShiftDir::Right);
        b.shift(&mock_field, ShiftDir::Right);

        // Asset I shifted only once
        assert_eq!(false, b.is_controlled(Pos::new(0, 0)));
        assert_eq!(true, b.is_controlled(Pos::new(1, 0)));
    }
}
