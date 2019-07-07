use crate::controlled::{ControlledBlocks, DropResult};
use crate::field::{self, Field, FieldBlock};
use crate::position::{p, Pos, RotateDir, ShiftDir};
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
    GhostPiece,
}

pub struct RenderBlockInfo {
    pub pos: Pos,
    pub block_type: DrawBlockType,
}

pub struct RenderInfo {
    pub field: Vec<RenderBlockInfo>,
}

impl GameState {
    pub fn new() -> GameState {
        let clock = GameClock::new();
        let now = clock.now();
        GameState {
            field: Field::new(),
            controlled_blocks: ControlledBlocks::new(now, Shape::random()),
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

    pub fn render_block_info(&self) -> RenderInfo {
        let mut blocks = Vec::with_capacity((field::VISIBLE_HEIGHT * field::WIDTH) as usize);
        let controlled_minos = self.controlled_blocks.minos();
        let ghost_minos = self.controlled_blocks.ghost_minos(&self.field);

        for y in 0..field::VISIBLE_HEIGHT {
            for x in 0..field::WIDTH {
                let pos = p(x, y);
                let block_type = match self.field.at(pos) {
                    FieldBlock::Empty => {
                        if controlled_minos.contains(pos) {
                            DrawBlockType::Controlled
                        } else if ghost_minos.contains(pos) {
                            DrawBlockType::GhostPiece
                        } else if pos.y >= field::PLAYING_BOUNDARY_HEIGHT {
                            DrawBlockType::OutOfPlay
                        } else {
                            DrawBlockType::Empty
                        }
                    }
                    FieldBlock::Occupied => DrawBlockType::Occupied,
                };

                blocks.push(RenderBlockInfo {
                    pos: pos,
                    block_type: block_type,
                });
            }
        }
        RenderInfo { field: blocks }
    }

    fn handle_drop(&mut self, drop_result: DropResult) {
        if let DropResult::Continue = drop_result {
            // These blocks are still dropping
            return;
        }
        self.controlled_blocks.minos().apply_to_field(&mut self.field);

        // Replace the stopped blocks with new ones
        self.controlled_blocks = ControlledBlocks::new(self.clock.now(), Shape::random());
    }
}
