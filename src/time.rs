use instant::Instant;
use std::ops::{Add, AddAssign, Sub};
use std::time::Duration;

#[derive(Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Debug)]
pub struct GameTime {
    since_start: Duration,
}

pub struct GameClock {
    start_time: Instant,
}

pub struct PausedClock {
    orig_start_time: Instant,
    pause_time: Instant,
}

impl GameClock {
    pub fn new() -> GameClock {
        GameClock {
            start_time: Instant::now(),
        }
    }

    pub fn pause(self) -> PausedClock {
        PausedClock {
            orig_start_time: self.start_time,
            pause_time: Instant::now(),
        }
    }

    pub fn now(&self) -> GameTime {
        return GameTime {
            since_start: Instant::now() - self.start_time,
        };
    }
}

impl PausedClock {
    pub fn resume(self) -> GameClock {
        GameClock {
            start_time: self.orig_start_time + (Instant::now() - self.pause_time),
        }
    }
}

impl Add<Duration> for GameTime {
    type Output = Self;
    fn add(self, other: Duration) -> Self {
        GameTime {
            since_start: self.since_start + other,
        }
    }
}

impl AddAssign<Duration> for GameTime {
    fn add_assign(&mut self, other: Duration) {
        *self = *self + other;
    }
}

impl Sub<GameTime> for GameTime {
    type Output = Duration;
    fn sub(self, other: GameTime) -> Duration {
        self.since_start - other.since_start
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn compare() {
        let a = GameTime {
            since_start: Duration::from_secs(1),
        };
        let b = GameTime {
            since_start: Duration::from_secs(2),
        };
        let c = GameTime {
            since_start: Duration::from_secs(1),
        };
        assert!(a < b);
        assert_eq!(a, c);
        assert_eq!(a + Duration::from_secs(1), b);
    }
}
