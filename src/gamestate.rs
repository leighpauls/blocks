use crate::controlled::{ControlledBlocks, DropResult};
use crate::field::{self, Field, FieldBlock};
use crate::position::{Pos, RotateDir, ShiftDir};
use crate::shapes::Shape;
use crate::time::GameClock;

pub struct GameState {
    field: Field,
    controlled_blocks: ControlledBlocks,
    clock: GameClock,
}

pub enum DrawBlockType {
    Empty,
    Controlled,
    Occupied,
    OutOfPlay,
}

impl GameState {
    pub fn new() -> GameState {
        let clock = GameClock::new();
        let now = clock.now();
        GameState {
            field: Field::new(),
            controlled_blocks: ControlledBlocks::new(now, Shape::I),
            clock: clock,
        }
    }

    pub fn update(&mut self) {
        let drop_result = self
            .controlled_blocks
            .maybe_periodic_drop(&self.field, self.clock.now());
        self.handle_drop(drop_result);
    }

    pub fn on_input_shift(&mut self, dir: ShiftDir) {
        self.controlled_blocks.shift(&self.field, dir);
    }

    pub fn on_input_soft_drop(&mut self) {
        let drop_result = self
            .controlled_blocks
            .manual_soft_drop(&self.field, self.clock.now());
        self.handle_drop(drop_result)
    }

    pub fn on_input_rotate(&mut self, dir: RotateDir) {
        self.controlled_blocks.rotate(&self.field, dir);
    }

    pub fn draw_block_type_at(&self, pos: Pos) -> DrawBlockType {
        match self.field.at(pos) {
            FieldBlock::Empty => {
                if self.controlled_blocks.is_controlled(pos) {
                    DrawBlockType::Controlled
                } else if pos.y >= field::PLAYING_BOUNDARY_HEIGHT {
                    DrawBlockType::OutOfPlay
                } else {
                    DrawBlockType::Empty
                }
            }
            FieldBlock::Occupied => DrawBlockType::Occupied,
        }
    }

    fn handle_drop(&mut self, drop_result: DropResult) {
        if let DropResult::Continue = drop_result {
            // These blocks are still dropping
            return;
        }
        for pos in self.controlled_blocks.positions().iter() {
            self.field.set(*pos, FieldBlock::Occupied);
        }

        // Replace the stopped blocks with new ones
        self.controlled_blocks = ControlledBlocks::new(self.clock.now(), Shape::I);
    }
}
