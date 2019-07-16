use crate::field::{CheckableField, Field};
use crate::position::{p, Pos, RotateDir, ShiftDir};
use crate::shapes::{MinoSet, Shape};
use crate::tetromino::Tetromino;
use crate::time::GameTime;
use std::time::Duration;

pub struct ControlledBlocks {
    pub tetromino: Tetromino,
    next_drop_time: GameTime,
    lock_time_accumulated: Duration,
    locking_prev_time: Option<GameTime>,
    drop_period: Duration,
}

fn start_pos() -> Pos {
    p(3, Field::PLAYING_BOUNDARY_HEIGHT - 2)
}

pub enum DropResult {
    Continue,
    Stop,
}

impl ControlledBlocks {
    pub fn new(start_time: GameTime, shape: Shape, drop_period: Duration) -> ControlledBlocks {
        ControlledBlocks {
            tetromino: Tetromino::new(start_pos(), shape),
            next_drop_time: start_time + drop_period,
            lock_time_accumulated: Duration::from_millis(0),
            locking_prev_time: None,
            drop_period: drop_period,
        }
    }

    pub fn minos(&self) -> MinoSet {
        self.tetromino.to_minos()
    }

    pub fn shift(&mut self, field: &CheckableField, dir: ShiftDir) {
        self.manual_movement(self.tetromino.try_shift(dir, field));
    }

    pub fn rotate(&mut self, field: &CheckableField, dir: RotateDir) {
        self.manual_movement(self.tetromino.try_rotate(dir, field));
    }

    fn manual_movement(&mut self, new_tetromino: Option<Tetromino>) {
        const MOVEMENT_REGAIN_LOCK: Duration = Duration::from_millis(10);
        if let Some(tet) = new_tetromino {
            self.tetromino = tet;
            if self.lock_time_accumulated > MOVEMENT_REGAIN_LOCK {
                self.lock_time_accumulated -= MOVEMENT_REGAIN_LOCK;
            } else {
                self.lock_time_accumulated = Duration::from_millis(0);
            }
        }
    }

    pub fn hard_drop(&mut self, field: &CheckableField) {
        self.tetromino = self.tetromino.hard_drop(field);
    }

    pub fn maybe_periodic_drop(&mut self, field: &CheckableField, now: GameTime) -> DropResult {
        match self.tetromino.try_down(field) {
            None => self.consume_lock_delay(now),
            Some(dropped) => {
                self.locking_prev_time = None;
                if self.next_drop_time <= now {
                    self.next_drop_time += self.drop_period;
                    self.tetromino = dropped;
                }
                DropResult::Continue
            }
        }
    }

    pub fn manual_soft_drop(&mut self, field: &CheckableField, now: GameTime) -> DropResult {
        match self.tetromino.try_down(field) {
            None => self.consume_lock_delay(now),
            Some(dropped) => {
                self.locking_prev_time = None;
                self.next_drop_time = now + self.drop_period;
                self.tetromino = dropped;
                DropResult::Continue
            }
        }
    }

    fn consume_lock_delay(&mut self, now: GameTime) -> DropResult {
        if let Some(prev_time) = self.locking_prev_time {
            self.lock_time_accumulated += now - prev_time;
        }
        self.locking_prev_time = Some(now);

        if self.lock_time_accumulated > Duration::from_millis(500) {
            DropResult::Stop
        } else {
            self.next_drop_time = now + self.drop_period;
            DropResult::Continue
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
        let mut b = ControlledBlocks::new(start_time, Shape::I, Duration::from_secs(1));

        b.maybe_periodic_drop(&mock_field, start_time + Duration::from_millis(10));

        b.maybe_periodic_drop(&mock_field, start_time + Duration::from_millis(1010));
    }
}
