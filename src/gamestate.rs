use crate::controlled::{ControlledBlocks, DropResult};
use crate::field::{Field, PlayingFieldRenderBlocksInstructions};
use crate::keybindings::{KeyboardStates, Trigger};
use crate::position::{p, Coord, Pos};
use crate::random_bag::RandomBag;
use crate::shapes::Shape;
use crate::tetromino::Tetromino;
use crate::time::{GameClock, GameTime};
use quicksilver::input::{ButtonState, Key};
use std::ops::Index;
use std::time::Duration;

pub struct GameState {
    field: Field,
    control: Control,
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

pub enum GameCondition {
    Playing,
    Won,
    Lost,
}

enum Control {
    Blocks(ControlledBlocks),
    WaitForClear(Vec<Coord>, GameTime),
    TakeHold(Shape),
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
    pub fn new() -> (GameState, GameClock) {
        let clock = GameClock::new();
        (
            GameState {
                field: Field::new(),
                control: Control::WaitForClear(vec![], clock.now()),
                random_bag: RandomBag::new(),
                hold_piece: None,
                can_hold: true,
                keyboard_states: KeyboardStates::new(),
                cleared_lines: 0,
            },
            clock,
        )
    }

    pub fn update<T>(&mut self, keyboard: &T, now: GameTime) -> GameCondition
    where
        T: Index<Key, Output = ButtonState>,
    {
        if let Control::TakeHold(shape) = &mut self.control {
            let s = *shape;
            self.control = match self.make_controlled_blocks(now, s) {
                Some(t) => Control::Blocks(t),
                None => {
                    return GameCondition::Lost;
                }
            };
        }

        if let Control::WaitForClear(lines, end_time) = &mut self.control {
            if *end_time <= now {
                self.field.remove_lines(&lines);
                let shape = self.random_bag.take_next();
                self.control = match self.make_controlled_blocks(now, shape) {
                    Some(t) => Control::Blocks(t),
                    None => {
                        return GameCondition::Lost;
                    }
                };
            }
        }

        for trigger in self.keyboard_states.update(keyboard, now) {
            self.handle_input(trigger, now);
        }

        if let Some(b) = self.control.as_blocks() {
            let drop = b.periodic_drop(&self.field, now);
            self.handle_soft_drop(drop, now);
        }

        const MAX_LEVEL: i32 = 15;
        if self.level() > MAX_LEVEL {
            GameCondition::Won
        } else {
            GameCondition::Playing
        }
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
                    let new_hold_shape = blocks.minos().shape();
                    self.control = match self.hold_piece {
                        Some(s) => Control::TakeHold(s),
                        None => Control::WaitForClear(vec![], now),
                    };
                    self.hold_piece = Some(new_hold_shape);
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
                Control::TakeHold(_) => {
                    PlayingFieldRenderBlocksInstructions::new_clearing(&self.field, vec![])
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
            self.control = Control::WaitForClear(vec![], now);
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

    fn make_controlled_blocks(&mut self, now: GameTime, shape: Shape) -> Option<ControlledBlocks> {
        let new_tetromino = Tetromino::try_new(start_pos(), shape, &self.field)?;
        Some(ControlledBlocks::new(
            now,
            new_tetromino,
            level_drop_period(self.level()),
        ))
    }
}

fn start_pos() -> Pos {
    p(3, Field::PLAYING_BOUNDARY_HEIGHT - 2)
}

fn level_drop_period(level: i32) -> Duration {
    let time_seconds = (0.8 - ((level - 1) as f32 * 0.007)).powi(level - 1);
    Duration::from_millis((time_seconds * 1000.0) as u64)
}
