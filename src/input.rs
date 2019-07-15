use crate::time::GameTime;
use std::time::Duration;

#[derive(PartialEq, Debug)]
pub enum InputEvent {
    Fire,
    Unchanged,
}

pub struct KeyStateMachine {
    next_repeat_time: Option<GameTime>,
}

const FIRST_REPEAT_DURATION: Duration = Duration::from_millis(150);
const CONTINUED_REPEAT_DURATION: Duration = Duration::from_millis(50);

impl KeyStateMachine {
    pub fn new() -> KeyStateMachine {
        KeyStateMachine {
            next_repeat_time: None,
        }
    }

    pub fn update(&mut self, is_down: bool, now: GameTime) -> InputEvent {
        match self.next_repeat_time {
            None => {
                if is_down {
                    self.next_repeat_time = Some(now + FIRST_REPEAT_DURATION);
                    InputEvent::Fire
                } else {
                    InputEvent::Unchanged
                }
            }
            Some(repeat_time) => {
                if is_down {
                    if repeat_time < now {
                        self.next_repeat_time = Some(repeat_time + CONTINUED_REPEAT_DURATION);
                        InputEvent::Fire
                    } else {
                        InputEvent::Unchanged
                    }
                } else {
                    self.next_repeat_time = None;
                    InputEvent::Unchanged
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::GameClock;

    #[test]
    fn states() {
        let clock = GameClock::new();
        let start_time = clock.now();

        let mut ksm = KeyStateMachine::new();
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

        let mut ksm = KeyStateMachine::new();

        assert_eq!(InputEvent::Fire, ksm.update(true, time));

        time = time + small_duration;
        assert_eq!(InputEvent::Unchanged, ksm.update(true, time));

        time = time + FIRST_REPEAT_DURATION;
        assert_eq!(InputEvent::Fire, ksm.update(true, time));

        time = time + small_duration;
        assert_eq!(InputEvent::Unchanged, ksm.update(true, time));

        time = time + CONTINUED_REPEAT_DURATION;
        assert_eq!(InputEvent::Fire, ksm.update(true, time));
    }
}
