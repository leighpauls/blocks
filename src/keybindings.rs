use crate::input::{InputEvent, KeyStateMachine, RepeatingKeyStateMachine, SingleKeyStateMachine};
use crate::position::{RotateDir, ShiftDir};
use crate::time::GameTime;
use quicksilver::input::{ButtonState, Key};
use std::ops::Index;
use std::time::Duration;

#[derive(Copy, Clone)]
pub enum Trigger {
    Shift(ShiftDir),
    SoftDown,
    Rotate(RotateDir),
    HardDrop,
    HoldPiece,
}

pub struct KeyboardStates {
    bindings: Vec<Binding>,
}

struct Binding {
    key: Key,
    trigger: Trigger,
    state: Box<KeyStateMachine>,
}

impl KeyboardStates {
    pub fn new() -> KeyboardStates {
        KeyboardStates {
            bindings: vec![
                bind_shift(Key::Left, Trigger::Shift(ShiftDir::Left)),
                bind_shift(Key::Right, Trigger::Shift(ShiftDir::Right)),
                bind_drop(Key::Down, Trigger::SoftDown),
                bind_single(Key::Z, Trigger::Rotate(RotateDir::CCW)),
                bind_single(Key::X, Trigger::Rotate(RotateDir::CW)),
                bind_single(Key::Space, Trigger::HardDrop),
                bind_single(Key::Up, Trigger::HardDrop),
                bind_single(Key::C, Trigger::HoldPiece),
            ],
        }
    }

    pub fn update<T>(&mut self, keyboard: &T, now: GameTime) -> Vec<Trigger>
    where
        T: Index<Key, Output = ButtonState>,
    {
        let mut result = vec![];
        for binding in self.bindings.iter_mut() {
            if let InputEvent::Fire = binding.state.update(keyboard[binding.key].is_down(), now) {
                result.push(binding.trigger);
            }
        }
        result
    }
}

fn bind_shift(key: Key, trigger: Trigger) -> Binding {
    bind(
        key,
        trigger,
        Box::new(RepeatingKeyStateMachine::new(
            Duration::from_millis(120),
            Duration::from_millis(40),
        )),
    )
}

fn bind_drop(key: Key, trigger: Trigger) -> Binding {
    let duration = Duration::from_millis(40);
    bind(
        key,
        trigger,
        Box::new(RepeatingKeyStateMachine::new(duration, duration)),
    )
}

fn bind_single(key: Key, trigger: Trigger) -> Binding {
    bind(key, trigger, Box::new(SingleKeyStateMachine::new()))
}

fn bind(key: Key, trigger: Trigger, ksm: Box<KeyStateMachine>) -> Binding {
    Binding {
        key: key,
        trigger: trigger,
        state: ksm,
    }
}
