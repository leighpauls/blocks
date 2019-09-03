use crate::field::CheckableField;
use crate::lockdelay::LockDelay;
use crate::position::{RotateDir, ShiftDir};
use crate::shapes::MinoSet;
use crate::tetromino::Tetromino;
use crate::time::GameTime;
use core::time::Duration;

pub struct ControlledBlocks {
    pub tetromino: Tetromino,
    next_drop_time: GameTime,
    drop_period: Duration,
    lock_delay: LockDelay,
}

#[derive(PartialEq, Debug)]
pub enum DropResult {
    Continue,
    Stop,
}

impl ControlledBlocks {
    pub fn new(
        start_time: GameTime,
        tetromino: Tetromino,
        drop_period: Duration,
    ) -> ControlledBlocks {
        ControlledBlocks {
            tetromino: tetromino,
            next_drop_time: start_time + drop_period,
            drop_period: drop_period,
            lock_delay: LockDelay::new(),
        }
    }

    pub fn minos(&self) -> MinoSet {
        self.tetromino.to_minos()
    }

    pub fn shift(&mut self, field: &dyn CheckableField, dir: ShiftDir) {
        self.manual_movement(self.tetromino.try_shift(dir, field));
    }

    pub fn rotate(&mut self, field: &dyn CheckableField, dir: RotateDir) {
        self.manual_movement(self.tetromino.try_rotate(dir, field));
    }

    pub fn hard_drop(&mut self, field: &dyn CheckableField) {
        self.tetromino = self.tetromino.hard_drop(field);
    }

    pub fn periodic_drop(&mut self, field: &dyn CheckableField, now: GameTime) -> DropResult {
        while self.next_drop_time <= now {
            match self.tetromino.try_down(field) {
                None => {
                    return self.lock_delay.consume_time(now);
                }
                Some(dropped) => {
                    self.lock_delay.reset();
                    self.next_drop_time += self.drop_period;
                    self.tetromino = dropped;
                }
            }
        }
        DropResult::Continue
    }

    pub fn manual_soft_drop(&mut self, field: &dyn CheckableField, now: GameTime) -> DropResult {
        match self.tetromino.try_down(field) {
            None => self.lock_delay.consume_time(now),
            Some(dropped) => {
                self.lock_delay.reset();
                self.next_drop_time = now + self.drop_period;
                self.tetromino = dropped;
                DropResult::Continue
            }
        }
    }

    fn manual_movement(&mut self, new_tetromino: Option<Tetromino>) {
        if let Some(tet) = new_tetromino {
            self.tetromino = tet;
            self.lock_delay.reset();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::position::{p, Pos};
    use crate::shapes::Shape;
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
        let mut b = ControlledBlocks::new(
            start_time,
            Tetromino::new(p(0, 0), Shape::I),
            Duration::from_secs(1),
        );

        b.periodic_drop(&mock_field, start_time + Duration::from_millis(10));

        b.periodic_drop(&mock_field, start_time + Duration::from_millis(1010));
    }
}
