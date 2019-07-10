use crate::field::{CheckableField, Field};
use crate::position::{p, Pos, RotateDir, ShiftDir};
use crate::shapes::{MinoSet, Shape};
use crate::tetromino::Tetromino;
use crate::time::GameTime;
use std::time::Duration;

pub struct ControlledBlocks {
    tetromino: Tetromino,
    next_drop_time: GameTime,
}

const DROP_PERIOD: Duration = Duration::from_millis(1000);

fn start_pos() -> Pos {
    p(3, Field::PLAYING_BOUNDARY_HEIGHT - 2)
}

pub enum DropResult {
    Continue,
    Stop,
}

impl ControlledBlocks {
    pub fn new(start_time: GameTime, shape: Shape) -> ControlledBlocks {
        ControlledBlocks {
            tetromino: Tetromino::new(start_pos(), shape),
            next_drop_time: start_time + DROP_PERIOD,
        }
    }

    pub fn minos(&self) -> MinoSet {
        self.tetromino.to_minos()
    }

    pub fn ghost_minos(&self, field: &CheckableField) -> MinoSet {
        self.tetromino.hard_drop(field).to_minos()
    }

    pub fn shift(&mut self, field: &CheckableField, dir: ShiftDir) {
        if let Some(new) = self.tetromino.try_shift(dir, field) {
            self.tetromino = new;
        }
    }

    pub fn rotate(&mut self, field: &CheckableField, dir: RotateDir) {
        if let Some(new) = self.tetromino.try_rotate(dir, field) {
            self.tetromino = new;
        }
    }

    pub fn hard_drop(&mut self, field: &CheckableField) {
        self.tetromino = self.tetromino.hard_drop(field);
    }

    pub fn maybe_periodic_drop(&mut self, field: &CheckableField, now: GameTime) -> DropResult {
        if self.next_drop_time > now {
            return DropResult::Continue;
        }
        self.next_drop_time += DROP_PERIOD;
        self.soft_drop(field)
    }

    pub fn manual_soft_drop(&mut self, field: &CheckableField, now: GameTime) -> DropResult {
        self.next_drop_time = now + DROP_PERIOD;
        self.soft_drop(field)
    }

    fn soft_drop(&mut self, field: &CheckableField) -> DropResult {
        if let Some(new) = self.tetromino.try_down(field) {
            self.tetromino = new;
            DropResult::Continue
        } else {
            DropResult::Stop
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::GameClock;

    mock_trait!(MockCheckableField, is_open(Pos) -> bool);
    impl CheckableField for MockCheckableField {
        mock_method!(is_open(&self, pos: Pos) -> bool);
    }

    #[test]
    fn periodic_drop() {
        let mock_field = MockCheckableField::default();
        mock_field.is_open.return_value(true);

        let clock = GameClock::new();
        let start_time = clock.now();
        let mut b = ControlledBlocks::new(start_time, Shape::I);

        b.maybe_periodic_drop(&mock_field, start_time + Duration::from_millis(10));

        b.maybe_periodic_drop(&mock_field, start_time + Duration::from_millis(1010));
    }
}
