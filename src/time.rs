use std::ops::{Add, AddAssign};
use std::time::{Duration, Instant};

#[derive(Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Debug)]
pub struct GameTime {
    since_start: Duration,
}

pub struct GameClock {
    start_time: Instant,
}

impl GameClock {
    pub fn new() -> GameClock {
        GameClock {
            start_time: Instant::now(),
        }
    }

    pub fn now(&self) -> GameTime {
        return GameTime {
            since_start: Instant::now() - self.start_time,
        };
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
