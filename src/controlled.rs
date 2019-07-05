use crate::field;
use crate::position::{p, Pos, ShiftDir};
use crate::shapes::{shape_positions, Rotations, Shape};
use crate::time::GameTime;
use std::time::Duration;

pub trait CheckField {
    fn is_open(&self, pos: Pos) -> bool;
}

pub struct ControlledBlocks {
    root_pos: Pos,
    shape: Shape,
    next_drop_time: GameTime,
}

const DROP_PERIOD: Duration = Duration::from_millis(1000);

fn start_pos() -> Pos {
    p(3, field::PLAYING_BOUNDARY_HEIGHT - 2)
}

pub enum DropResult {
    Continue,
    Stop,
}

impl ControlledBlocks {
    pub fn new(start_time: GameTime, shape: Shape) -> ControlledBlocks {
        ControlledBlocks {
            root_pos: start_pos(),
            shape: shape,
            next_drop_time: start_time + DROP_PERIOD,
        }
    }

    pub fn positions(&self) -> [Pos; 4] {
        let shape = self.shape;
        self.root_pos + shape_positions(shape, Rotations::Zero)
    }

    pub fn is_controlled(&self, target: Pos) -> bool {
        for pos in self.positions().iter() {
            if *pos == target {
                return true;
            }
        }
        return false;
    }

    pub fn shift(&mut self, field: &CheckField, dir: ShiftDir) {
        // Don't move if it's not legal
        for pos in self.positions().iter() {
            if !field.is_open(*pos + dir) {
                return;
            }
        }
        self.root_pos = self.root_pos + dir;
    }

    pub fn maybe_periodic_drop(&mut self, field: &CheckField, now: GameTime) -> DropResult {
        if self.next_drop_time > now {
            return DropResult::Continue;
        }
        self.next_drop_time += DROP_PERIOD;
        self.soft_drop(field)
    }

    pub fn manual_soft_drop(&mut self, field: &CheckField, now: GameTime) -> DropResult {
        self.next_drop_time = now + DROP_PERIOD;
        self.soft_drop(field)
    }

    fn soft_drop(&mut self, field: &CheckField) -> DropResult {
        let delta = p(0, -1);
        for pos in self.positions().iter() {
            if !field.is_open(*pos + delta) {
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
    use crate::time::GameClock;

    mock_trait!(MockCheckField, is_open(Pos) -> bool);
    impl CheckField for MockCheckField {
        mock_method!(is_open(&self, pos: Pos) -> bool);
    }

    #[test]
    fn controlled() {
        let b = blocks();
        assert_eq!(true, b.is_controlled(start_pos()));
        assert_eq!(false, b.is_controlled(p(0, 0)));
    }

    #[test]
    fn shift() {
        let mock_field = MockCheckField::default();
        mock_field
            .is_open
            .return_value_for(start_pos() + p(2, 0), false);
        mock_field.is_open.return_value(true);

        let mut b = blocks();
        b.shift(&mock_field, ShiftDir::Right);
        b.shift(&mock_field, ShiftDir::Right);

        // Asset I shifted only once
        assert_eq!(false, b.is_controlled(start_pos()));
        assert_eq!(true, b.is_controlled(start_pos() + ShiftDir::Right));
    }

    #[test]
    fn periodic_drop() {
        let mock_field = MockCheckField::default();
        mock_field.is_open.return_value(true);

        let clock = GameClock::new();
        let start_time = clock.now();
        let mut b = ControlledBlocks::new(start_time, Shape::TestShape);

        b.maybe_periodic_drop(&mock_field, start_time + Duration::from_millis(10));
        assert_eq!(true, b.is_controlled(start_pos()));

        b.maybe_periodic_drop(&mock_field, start_time + Duration::from_millis(1010));
        assert_eq!(false, b.is_controlled(start_pos()));
    }

    fn blocks() -> ControlledBlocks {
        ControlledBlocks::new(GameClock::new().now(), Shape::TestShape)
    }
}
