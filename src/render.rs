use crate::field::{Field, IterableField};
use crate::position::p;
use crate::position::Coord;
use crate::position::Pos;
use crate::shapes::MinoSet;

use quicksilver::{
    geom::{Rectangle, Transform, Vector},
    graphics::Color,
    lifecycle::Window,
};

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

pub struct VisibleBlock {
    pub pos: Pos,
    pub is_occupied: bool,
}

pub struct RenderBlockIterator<'a, TField: IterableField> {
    field: &'a TField,
    controlled_minos: MinoSet,
    ghost_minos: MinoSet,
    next_pos: Pos,
}

impl<'a, TField: IterableField> RenderBlockIterator<'a, TField> {
    pub fn new(field: &'a TField, controlled_minos: MinoSet, ghost_minos: MinoSet) -> Self {
        Self {
            field: field,
            controlled_minos: controlled_minos,
            ghost_minos: ghost_minos,
            next_pos: p(0, 0),
        }
    }

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

pub fn render_field<'a, T: IterableField>(
    blocks: RenderBlockIterator<'a, T>,
    scale_transform: Transform,
    position_transform: Transform,
    window: &mut Window,
) {
    let height = blocks.height_blocks();
    for block in blocks {
        window.draw_ex(
            &Rectangle::new(
                position_transform
                    * Vector::new(block.pos.x as f32, (height - block.pos.y - 1) as f32),
                scale_transform * Vector::new(1, 1),
            ),
            match block.block_type {
                DrawBlockType::Empty => Color::BLUE,
                DrawBlockType::Controlled => Color::GREEN,
                DrawBlockType::Occupied => Color::RED,
                DrawBlockType::OutOfPlay => Color::YELLOW,
                DrawBlockType::GhostPiece => Color::from_rgba(0xff, 0, 0xff, 1.0),
            },
            Transform::IDENTITY,
            0,
        );
    }
}
