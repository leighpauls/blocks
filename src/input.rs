use crate::time::GameTime;
use std::time::Duration;

#[derive(PartialEq, Debug)]
pub enum InputEvent {
    Fire,
    Unchanged,
}

pub trait KeyStateMachine {
    fn update(&mut self, is_down: bool, now: GameTime) -> InputEvent;
}

pub struct RepeatingKeyStateMachine {
    next_repeat_time: Option<GameTime>,
    first_repeat_duration: Duration,
    continued_repeat_duration: Duration,
}

impl RepeatingKeyStateMachine {
    pub fn new(first: Duration, continued: Duration) -> Self {
        RepeatingKeyStateMachine {
            next_repeat_time: None,
            first_repeat_duration: first,
            continued_repeat_duration: continued,
        }
    }
}

impl KeyStateMachine for RepeatingKeyStateMachine {
    fn update(&mut self, is_down: bool, now: GameTime) -> InputEvent {
        match (self.next_repeat_time, is_down) {
            (None, true) => {
                self.next_repeat_time = Some(now + self.first_repeat_duration);
                InputEvent::Fire
            }
            (None, false) => InputEvent::Unchanged,
            (Some(repeat_time), true) => {
                if repeat_time < now {
                    self.next_repeat_time = Some(repeat_time + self.continued_repeat_duration);
                    InputEvent::Fire
                } else {
                    InputEvent::Unchanged
                }
            }
            (Some(_), false) => {
                self.next_repeat_time = None;
                InputEvent::Unchanged
            }
        }
    }
}

pub struct SingleKeyStateMachine {
    was_down: bool,
}

impl SingleKeyStateMachine {
    pub fn new() -> Self {
        SingleKeyStateMachine { was_down: false }
    }
}

impl KeyStateMachine for SingleKeyStateMachine {
    fn update(&mut self, is_down: bool, _now: GameTime) -> InputEvent {
        let result = if is_down && !self.was_down {
            InputEvent::Fire
        } else {
            InputEvent::Unchanged
        };
        self.was_down = is_down;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::GameClock;

    const FIRST_DURATION: Duration = Duration::from_millis(100);
    const CONTINUED_DURATION: Duration = Duration::from_millis(40);

    #[test]
    fn states() {
        let clock = GameClock::new();
        let start_time = clock.now();

        let mut ksm = RepeatingKeyStateMachine::new(FIRST_DURATION, CONTINUED_DURATION);
        assert_eq!(InputEvent::Unchanged, ksm.update(false, start_time));
        assert_eq!(InputEvent::Fire, ksm.update(true, start_time));
        assert_eq!(InputEvent::Unchanged, ksm.update(true, start_time));
        assert_eq!(InputEvent::Unchanged, ksm.update(false, start_time));
    }

    #[test]
    fn repeat() {
        let clock = GameClock::new();
        let mut time = clock.now();

        let small_duration = Duration::from_millis(1);

        let mut ksm = RepeatingKeyStateMachine::new(FIRST_DURATION, CONTINUED_DURATION);

        assert_eq!(InputEvent::Fire, ksm.update(true, time));

        time = time + small_duration;
        assert_eq!(InputEvent::Unchanged, ksm.update(true, time));

        time = time + FIRST_DURATION;
        assert_eq!(InputEvent::Fire, ksm.update(true, time));

        time = time + small_duration;
        assert_eq!(InputEvent::Unchanged, ksm.update(true, time));

        time = time + CONTINUED_DURATION;
        assert_eq!(InputEvent::Fire, ksm.update(true, time));
    }
}
