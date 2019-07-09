use crate::controlled::{ControlledBlocks, DropResult};
use crate::field::{self, Field};
use crate::position::{Pos, RotateDir, ShiftDir};
use crate::shapes::{MinoSet, Shape};
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
        self.handle_soft_drop(drop_result);
    }

    pub fn on_input_shift(&mut self, dir: ShiftDir) {
        self.controlled_blocks.shift(&self.field, dir);
    }

    pub fn on_input_soft_drop(&mut self) {
        let drop_result = self
            .controlled_blocks
            .manual_soft_drop(&self.field, self.clock.now());
        self.handle_soft_drop(drop_result)
    }

    pub fn on_input_rotate(&mut self, dir: RotateDir) {
        self.controlled_blocks.rotate(&self.field, dir);
    }

    pub fn on_input_hard_drop(&mut self) {
        self.controlled_blocks.hard_drop(&self.field);
        self.replace_controlled_piece();
    }

    pub fn render_info(&self) -> RenderInfo {
        let playing_field = render_blocks_for_field(
            &self.field,
            &self.controlled_blocks.minos(),
            &self.controlled_blocks.ghost_minos(&self.field),
        );

        RenderInfo {
            field: playing_field,
        }
    }

    fn handle_soft_drop(&mut self, drop_result: DropResult) {
        if let DropResult::Stop = drop_result {
            self.replace_controlled_piece();
        }
    }

    fn replace_controlled_piece(&mut self) {
        self.controlled_blocks
            .minos()
            .apply_to_field(&mut self.field);

        self.field.remove_lines();

        // Replace the stopped blocks with new ones
        self.controlled_blocks = ControlledBlocks::new(self.clock.now(), Shape::random());
    }
}

fn render_blocks_for_field(
    field: &Field,
    controlled_minos: &MinoSet,
    ghost_minos: &MinoSet,
) -> Vec<RenderBlockInfo> {
    let mut playing_field = Vec::with_capacity((field::VISIBLE_HEIGHT * field::WIDTH) as usize);

    for b in field.iter() {
        let block_type = if controlled_minos.contains(b.pos) {
            DrawBlockType::Controlled
        } else if ghost_minos.contains(b.pos) {
            DrawBlockType::GhostPiece
        } else if b.pos.y >= field::PLAYING_BOUNDARY_HEIGHT {
            DrawBlockType::OutOfPlay
        } else if b.is_occupied {
            DrawBlockType::Occupied
        } else {
            DrawBlockType::Empty
        };

        playing_field.push(RenderBlockInfo {
            pos: b.pos,
            block_type: block_type,
        });
    }
    playing_field
}
