use crate::controlled::{ControlledBlocks, DropResult};
use crate::field::{Field, IterableField};
use crate::position::{p, Coord, Pos, RotateDir, ShiftDir};
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

pub struct RenderInfo<'a> {
    pub field: RenderBlockIterator<'a, Field>,
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
        RenderInfo {
            field: render_blocks_for_field(
                &self.field,
                self.controlled_blocks.minos(),
                self.controlled_blocks.ghost_minos(&self.field),
            ),
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

fn render_blocks_for_field<'a>(
    field: &'a Field,
    controlled_minos: MinoSet,
    ghost_minos: MinoSet,
) -> RenderBlockIterator<'a, Field> {
    RenderBlockIterator {
        field: field,
        controlled_minos: controlled_minos,
        ghost_minos: ghost_minos,
        next_pos: p(0, 0),
    }
}

pub struct RenderBlockIterator<'a, TField: IterableField> {
    field: &'a TField,
    controlled_minos: MinoSet,
    ghost_minos: MinoSet,
    next_pos: Pos,
}

impl<'a, TField: IterableField> RenderBlockIterator<'a, TField> {
    pub fn width_blocks(&self) -> Coord {
        TField::WIDTH
    }
    pub fn height_blocks(&self) -> Coord {
        TField::VISIBLE_HEIGHT
    }
}

impl<'a, TField: IterableField> Iterator for RenderBlockIterator<'a, TField> {
    type Item = RenderBlockInfo;

    fn next(&mut self) -> Option<RenderBlockInfo> {
        if self.next_pos.y >= TField::VISIBLE_HEIGHT {
            return None;
        }
        let result = Some(RenderBlockInfo {
            pos: self.next_pos,
            block_type: if self.controlled_minos.contains(self.next_pos) {
                DrawBlockType::Controlled
            } else if self.ghost_minos.contains(self.next_pos) {
                DrawBlockType::GhostPiece
            } else if self.next_pos.y >= TField::PLAYING_BOUNDARY_HEIGHT {
                DrawBlockType::OutOfPlay
            } else if self.field.is_open(self.next_pos) {
                DrawBlockType::Empty
            } else {
                DrawBlockType::Occupied
            },
        });

        self.next_pos = self.next_pos + p(1, 0);
        if self.next_pos.x >= TField::WIDTH {
            self.next_pos = p(0, self.next_pos.y + 1);
        }
        result
    }
}
