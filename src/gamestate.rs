use crate::controlled::{ControlledBlocks, DropResult};
use crate::field::{Field, PlayingFieldRenderBlocksInstructions};
use crate::keybindings::{KeyboardStates, Trigger};
use crate::position::Coord;
use crate::random_bag::RandomBag;
use crate::shapes::Shape;
use crate::time::{GameClock, GameTime};
use quicksilver::input::{ButtonState, Key};
use std::ops::Index;
use std::time::Duration;

pub struct GameState {
    field: Field,
    control: Control,
    clock: GameClock,
    random_bag: RandomBag,
    hold_piece: Option<Shape>,
    can_hold: bool,
    keyboard_states: KeyboardStates,
    cleared_lines: i32,
}

pub struct RenderInfo<'a> {
    pub playing_field: PlayingFieldRenderBlocksInstructions<'a>,
    pub previews: Vec<Shape>,
    pub hold_piece: Option<Shape>,
    pub cleared_lines: i32,
    pub level: i32,
}

enum Control {
    Blocks(ControlledBlocks),
    WaitForClear(Vec<Coord>, GameTime),
}

impl Control {
    fn as_blocks(&mut self) -> Option<&mut ControlledBlocks> {
        match self {
            Control::Blocks(cb) => Some(cb),
            _ => None,
        }
    }
}

impl GameState {
    pub fn new() -> GameState {
        let clock = GameClock::new();
        let now = clock.now();
        let mut rb = RandomBag::new();
        let level = 1;

        GameState {
            field: Field::new(),
            control: Control::Blocks(ControlledBlocks::new(
                now,
                rb.take_next(),
                level_drop_period(level),
            )),
            clock: clock,
            random_bag: rb,
            hold_piece: None,
            can_hold: true,
            keyboard_states: KeyboardStates::new(),
            cleared_lines: 0,
        }
    }

    pub fn update<T>(&mut self, keyboard: &T) -> Option<()>
    where
        T: Index<Key, Output = ButtonState>,
    {
        let now = self.clock.now();

        if let Control::WaitForClear(lines, end_time) = &mut self.control {
            if *end_time <= now {
                self.field.remove_lines(&lines);
                self.control = Control::Blocks(self.take_next_block(now));
            }
        }

        for trigger in self.keyboard_states.update(keyboard, now) {
            self.handle_input(trigger, now);
        }
        let drop_result = self
            .control
            .as_blocks()?
            .maybe_periodic_drop(&self.field, now);
        self.handle_soft_drop(drop_result, now);
        None
    }

    fn handle_input(&mut self, trigger: Trigger, now: GameTime) -> Option<()> {
        let blocks = self.control.as_blocks()?;
        match trigger {
            Trigger::Shift(dir) => blocks.shift(&self.field, dir),
            Trigger::SoftDown => {
                let drop_result = blocks.manual_soft_drop(&self.field, now);
                self.handle_soft_drop(drop_result, now);
            }
            Trigger::Rotate(dir) => blocks.rotate(&self.field, dir),
            Trigger::HardDrop => {
                blocks.hard_drop(&self.field);
                self.replace_controlled_piece(now);
            }
            Trigger::HoldPiece => {
                if self.can_hold {
                    let new_piece = match self.hold_piece {
                        Some(shape) => shape,
                        None => self.random_bag.take_next(),
                    };
                    self.hold_piece = Some(blocks.minos().shape());
                    self.control = Control::Blocks(ControlledBlocks::new(
                        now,
                        new_piece,
                        level_drop_period(self.level()),
                    ));
                    self.can_hold = false;
                }
            }
        };
        None
    }

    pub fn render_info(&self) -> RenderInfo {
        RenderInfo {
            playing_field: match &self.control {
                Control::Blocks(b) => {
                    PlayingFieldRenderBlocksInstructions::new_controlled(&self.field, b.tetromino)
                }
                Control::WaitForClear(lines, _) => {
                    PlayingFieldRenderBlocksInstructions::new_clearing(&self.field, lines.clone())
                }
            },
            previews: self.random_bag.previews(),
            hold_piece: self.hold_piece,
            cleared_lines: self.cleared_lines,
            level: self.level(),
        }
    }

    fn handle_soft_drop(&mut self, drop_result: DropResult, now: GameTime) {
        if let DropResult::Stop = drop_result {
            self.replace_controlled_piece(now);
        }
    }

    fn replace_controlled_piece(&mut self, now: GameTime) -> Option<()> {
        self.control
            .as_blocks()?
            .minos()
            .apply_to_field(&mut self.field);
        self.can_hold = true;

        let lines = self.field.find_lines();
        if lines.is_empty() {
            // Replace the stopped blocks with new ones
            self.control = Control::Blocks(self.take_next_block(now));
        } else {
            self.cleared_lines += lines.len() as i32;
            self.control = Control::WaitForClear(lines, now + Duration::from_millis(500));
        }
        None
    }

    fn level(&self) -> i32 {
        const LINES_PER_LEVEL: i32 = 10;
        self.cleared_lines / LINES_PER_LEVEL + 1
    }

    fn take_next_block(&mut self, now: GameTime) -> ControlledBlocks {
        ControlledBlocks::new(
            now,
            self.random_bag.take_next(),
            level_drop_period(self.level()),
        )
    }
}

fn level_drop_period(level: i32) -> Duration {
    let time_seconds = (0.8 - ((level - 1) as f32 * 0.007)).powi(level - 1);
    Duration::from_millis((time_seconds * 1000.0) as u64)
}
