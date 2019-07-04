use crate::position::{Pos, ShiftDir, p};
use std::time::Duration;

pub trait CheckField {
    fn is_open(&self, pos: Pos) -> bool;
}

pub struct ControlledBlocks {
    root_pos: Pos,
    relative_poses: [Pos; 4],
    next_drop_time: Duration,
}

const DROP_PERIOD: Duration = Duration::from_millis(1000);

pub enum DropResult {
    Continue,
    Stop,
}

impl ControlledBlocks {
    pub fn new(start_time: Duration) -> ControlledBlocks {
        ControlledBlocks {
            root_pos: p(0, 0),
            relative_poses: [
                p(0, 0),
                p(1, 0),
                p(2, 0),
                p(3, 0),
            ],
            next_drop_time: start_time + DROP_PERIOD,
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

    pub fn maybe_periodic_drop(&mut self, field: &CheckField, now: Duration) -> DropResult {
        if self.next_drop_time > now {
            return DropResult::Continue;
        }
        self.next_drop_time += DROP_PERIOD;
        self.soft_drop(field)
    }

    pub fn manual_soft_drop(&mut self, field: &CheckField, now: Duration) -> DropResult {
        self.next_drop_time = now + DROP_PERIOD;
        self.soft_drop(field)
    }

    fn soft_drop(&mut self, field: &CheckField) -> DropResult {
        let delta = p(0, 1);
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
    use hamcrest2::prelude::*;

    mock_trait!(MockCheckField, is_open(Pos) -> bool);
    impl CheckField for MockCheckField {
        mock_method!(is_open(&self, pos: Pos) -> bool);
    }

    #[test]
    fn controlled() {
        let b = blocks();
        assert_eq!(true, b.is_controlled(p(0, 0)));
        assert_eq!(false, b.is_controlled(p(0, 1)));
    }

    #[test]
    fn shift() {
        let mock_field = MockCheckField::default();
        mock_field.is_open.return_value_for(p(5, 0), false);
        mock_field.is_open.return_value(true);

        let mut b = blocks();
        b.shift(&mock_field, ShiftDir::Right);
        b.shift(&mock_field, ShiftDir::Right);

        // Asset I shifted only once
        assert_eq!(false, b.is_controlled(p(0, 0)));
        assert_eq!(true, b.is_controlled(p(1, 0)));
    }

    #[test]
    fn periodic_drop() {
        let mock_field = MockCheckField::default();
        mock_field.is_open.return_value(true);

        let mut b = blocks();

        b.maybe_periodic_drop(&mock_field, Duration::from_millis(10));
        assert_eq!(true, b.is_controlled(p(0, 0)));

        b.maybe_periodic_drop(&mock_field, Duration::from_millis(1010));
        assert_eq!(false, b.is_controlled(p(0, 0)));
    }

    #[test]
    fn positions() {
        let b = blocks();
        assert_that!(
            &b.positions(),
            contains(vec!(
                p(0, 0),
                p(1, 0),
                p(2, 0),
                p(3, 0)
            ))
            .exactly()
        );
    }

    fn blocks() -> ControlledBlocks {
        ControlledBlocks::new(Duration::from_millis(0))
    }
}
