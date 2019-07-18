use crate::controlled::DropResult;
use crate::time::GameTime;
use std::time::Duration;

pub struct LockDelay {
    accumulated_time: Duration,
    prev_lock_time: Option<GameTime>,
    num_resets: u32,
}

const LOCK_DELAY: Duration = Duration::from_millis(500);
const ALLOWED_RESETS: u32 = 5;

impl LockDelay {
    pub fn new() -> Self {
        LockDelay {
            accumulated_time: Duration::from_millis(0),
            prev_lock_time: None,
            num_resets: 0,
        }
    }

    pub fn consume_time(&mut self, now: GameTime) -> DropResult {
        if let Some(prev) = self.prev_lock_time {
            self.accumulated_time += now - prev;
        }
        self.prev_lock_time = Some(now);

        if self.accumulated_time > LOCK_DELAY {
            DropResult::Stop
        } else {
            DropResult::Continue
        }
    }

    pub fn reset(&mut self) {
        if let Some(_) = self.prev_lock_time {
            if self.num_resets < ALLOWED_RESETS {
                self.accumulated_time = Duration::from_millis(0);
                self.prev_lock_time = None;
                self.num_resets += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::GameClock;

    const EPS: Duration = Duration::from_millis(1);

    #[test]
    fn simple_delay() {
        let start_time = GameClock::new().now();
        let mut ld = LockDelay::new();
        assert_eq!(ld.consume_time(start_time), DropResult::Continue);
        assert_eq!(
            ld.consume_time(start_time + LOCK_DELAY + EPS),
            DropResult::Stop
        )
    }

    #[test]
    fn reset() {
        let start_time = GameClock::new().now();
        let mut ld = LockDelay::new();
        assert_eq!(ld.consume_time(start_time), DropResult::Continue);
        ld.reset();
        assert_eq!(
            ld.consume_time(start_time + LOCK_DELAY + EPS),
            DropResult::Continue
        );
        assert_eq!(
            ld.consume_time(start_time + (LOCK_DELAY + EPS) * 2),
            DropResult::Stop
        );
    }

    #[test]
    fn consume_resets() {
        let start_time = GameClock::new().now();
        let mut ld = LockDelay::new();

        for i in 0..ALLOWED_RESETS {
            assert_eq!(
                ld.consume_time(start_time + (LOCK_DELAY + EPS) * i),
                DropResult::Continue
            );
            ld.reset();
            ld.reset();
        }

        assert_eq!(
            ld.consume_time(start_time + (LOCK_DELAY + EPS) * ALLOWED_RESETS),
            DropResult::Continue
        );
        ld.reset();
        assert_eq!(
            ld.consume_time(start_time + (LOCK_DELAY + EPS) * (ALLOWED_RESETS + 1)),
            DropResult::Stop
        );
    }
}
