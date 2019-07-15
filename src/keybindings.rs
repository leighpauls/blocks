use crate::input::{InputEvent, KeyStateMachine};
use crate::position::{RotateDir, ShiftDir};
use crate::time::GameTime;
use quicksilver::input::{ButtonState, Key};
use std::ops::Index;

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
    state: KeyStateMachine,
}

impl KeyboardStates {
    pub fn new() -> KeyboardStates {
        fn bind(key: Key, trigger: Trigger) -> Binding {
            Binding {
                key: key,
                trigger: trigger,
                state: KeyStateMachine::new(),
            }
        }

        KeyboardStates {
            bindings: vec![
                bind(Key::Left, Trigger::Shift(ShiftDir::Left)),
                bind(Key::Right, Trigger::Shift(ShiftDir::Right)),
                bind(Key::Down, Trigger::SoftDown),
                bind(Key::Z, Trigger::Rotate(RotateDir::CCW)),
                bind(Key::X, Trigger::Rotate(RotateDir::CW)),
                bind(Key::Space, Trigger::HardDrop),
                bind(Key::C, Trigger::HoldPiece),
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
