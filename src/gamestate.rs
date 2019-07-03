use crate::controlled::{ControlledBlocks, DropResult};
use crate::field::{Field, FieldBlock};
use crate::position::{Pos, ShiftDir};
use std::time::{Duration, Instant};

pub struct GameState {
    field: Field,
    controlled_blocks: ControlledBlocks,
    game_start_time: Instant,
}

pub enum DrawBlockType {
    Empty,
    Controlled,
    Occupied,
}

impl GameState {
    pub fn new() -> GameState {
        let start_time = Instant::now();
        GameState {
            field: Field::new(),
            controlled_blocks: ControlledBlocks::new(Instant::now() - start_time),
            game_start_time: start_time,
        }
    }

    pub fn update(&mut self) {
        let drop_result = self
            .controlled_blocks
            .maybe_periodic_drop(&self.field, self.game_time());
        self.handle_drop(drop_result);
    }

    pub fn on_input_shift(&mut self, dir: ShiftDir) {
        self.controlled_blocks.shift(&self.field, dir);
    }

    pub fn on_input_soft_drop(&mut self) {
        let drop_result = self
            .controlled_blocks
            .manual_soft_drop(&self.field, self.game_time());
        self.handle_drop(drop_result)
    }

    pub fn draw_block_type_at(&self, pos: Pos) -> DrawBlockType {
        match self.field.at(pos) {
            FieldBlock::Empty => {
                if self.controlled_blocks.is_controlled(pos) {
                    DrawBlockType::Controlled
                } else {
                    DrawBlockType::Empty
                }
            }
            FieldBlock::Occupied => DrawBlockType::Occupied,
        }
    }

    fn game_time(&self) -> Duration {
        Instant::now() - self.game_start_time
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
        self.controlled_blocks = ControlledBlocks::new(self.game_time());
    }
}
