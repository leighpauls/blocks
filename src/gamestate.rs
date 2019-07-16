use crate::controlled::{ControlledBlocks, DropResult};
use crate::field::{Field, PlayingFieldRenderBlocksInstructions};
use crate::keybindings::{KeyboardStates, Trigger};
use crate::position::{RotateDir, ShiftDir};
use crate::random_bag::RandomBag;
use crate::shapes::Shape;
use crate::time::{GameClock, GameTime};
use quicksilver::input::{ButtonState, Key};
use std::ops::Index;

pub struct GameState {
    field: Field,
    controlled_blocks: ControlledBlocks,
    clock: GameClock,
    random_bag: RandomBag,
    hold_piece: Option<Shape>,
    can_hold: bool,
    keyboard_states: KeyboardStates,
    remaining_lines: i32,
    level: i32,
}

pub struct RenderInfo<'a> {
    pub playing_field: PlayingFieldRenderBlocksInstructions<'a>,
    pub previews: Vec<Shape>,
    pub hold_piece: Option<Shape>,
    pub remaining_lines: i32,
    pub level: i32,
}

impl GameState {
    pub fn new() -> GameState {
        let clock = GameClock::new();
        let now = clock.now();
        let mut rb = RandomBag::new();

        GameState {
            field: Field::new(),
            controlled_blocks: ControlledBlocks::new(now, rb.take_next()),
            clock: clock,
            random_bag: rb,
            hold_piece: None,
            can_hold: true,
            keyboard_states: KeyboardStates::new(),
            remaining_lines: 10,
            level: 1,
        }
    }

    pub fn update<T>(&mut self, keyboard: &T)
    where
        T: Index<Key, Output = ButtonState>,
    {
        let now = self.clock.now();
        for trigger in self.keyboard_states.update(keyboard, now) {
            match trigger {
                Trigger::Shift(dir) => self.on_input_shift(dir),
                Trigger::SoftDown => self.on_input_soft_drop(now),
                Trigger::Rotate(dir) => self.on_input_rotate(dir),
                Trigger::HardDrop => self.on_input_hard_drop(now),
                Trigger::HoldPiece => self.on_input_hold_piece(now),
            }
        }

        let drop_result = self.controlled_blocks.maybe_periodic_drop(&self.field, now);
        self.handle_soft_drop(drop_result, now);
    }

    fn on_input_shift(&mut self, dir: ShiftDir) {
        self.controlled_blocks.shift(&self.field, dir);
    }

    fn on_input_soft_drop(&mut self, now: GameTime) {
        let drop_result = self.controlled_blocks.manual_soft_drop(&self.field, now);
        self.handle_soft_drop(drop_result, now)
    }

    fn on_input_rotate(&mut self, dir: RotateDir) {
        self.controlled_blocks.rotate(&self.field, dir);
    }

    fn on_input_hard_drop(&mut self, now: GameTime) {
        self.controlled_blocks.hard_drop(&self.field);
        self.replace_controlled_piece(now);
    }

    fn on_input_hold_piece(&mut self, now: GameTime) {
        if !self.can_hold {
            return;
        }
        let new_piece = match self.hold_piece {
            Some(shape) => shape,
            None => self.random_bag.take_next(),
        };
        self.hold_piece = Some(self.controlled_blocks.minos().shape());
        self.controlled_blocks = ControlledBlocks::new(now, new_piece);
        self.can_hold = false;
    }

    pub fn render_info(&self) -> RenderInfo {
        RenderInfo {
            playing_field: PlayingFieldRenderBlocksInstructions::new(
                &self.field,
                self.controlled_blocks.minos(),
                self.controlled_blocks.ghost_minos(&self.field),
            ),
            previews: self.random_bag.previews(),
            hold_piece: self.hold_piece,
            remaining_lines: self.remaining_lines,
            level: self.level,
        }
    }

    fn handle_soft_drop(&mut self, drop_result: DropResult, now: GameTime) {
        if let DropResult::Stop = drop_result {
            self.replace_controlled_piece(now);
        }
    }

    fn replace_controlled_piece(&mut self, now: GameTime) {
        self.controlled_blocks
            .minos()
            .apply_to_field(&mut self.field);

        let removed_lines = self.field.remove_lines();
        self.remaining_lines -= removed_lines;
        if self.remaining_lines <= 0 {
            self.remaining_lines = 10;
            self.level += 1;
        }

        self.can_hold = true;

        // Replace the stopped blocks with new ones
        self.controlled_blocks = ControlledBlocks::new(now, self.random_bag.take_next());
    }
}
