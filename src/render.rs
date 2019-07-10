use crate::field::{CheckableField, Field};
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
    pub playing_field: PlayingFieldRenderBlocksInstructions<'a>,
}

pub trait BlockRenderInstructions<I>
where
    I: Iterator<Item = RenderBlockInfo>,
{
    fn blocks(&self) -> I;

    fn height_blocks(&self) -> Coord;
    fn width_blocks(&self) -> Coord;
}

pub fn render_blocks<T, I>(
    instructions: &T,
    scale_transform: Transform,
    position_transform: Transform,
    window: &mut Window,
) where
    I: Iterator<Item = RenderBlockInfo>,
    T: BlockRenderInstructions<I>,
{
    for block in instructions.blocks() {
        window.draw_ex(
            &Rectangle::new(
                position_transform
                    * Vector::new(
                        block.pos.x as f32,
                        (instructions.height_blocks() - block.pos.y - 1) as f32,
                    ),
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

pub struct PlayingFieldRenderBlocksIterator<'a> {
    field: &'a Field,
    controlled_minos: MinoSet,
    ghost_minos: MinoSet,
    next_pos: Pos,
}

impl<'a> Iterator for PlayingFieldRenderBlocksIterator<'a> {
    type Item = RenderBlockInfo;

    fn next(&mut self) -> Option<RenderBlockInfo> {
        if self.next_pos.y >= Field::VISIBLE_HEIGHT {
            return None;
        }
        let result = Some(RenderBlockInfo {
            pos: self.next_pos,
            block_type: if self.controlled_minos.contains(self.next_pos) {
                DrawBlockType::Controlled
            } else if self.ghost_minos.contains(self.next_pos) {
                DrawBlockType::GhostPiece
            } else if self.next_pos.y >= Field::PLAYING_BOUNDARY_HEIGHT {
                DrawBlockType::OutOfPlay
            } else if self.field.is_open(self.next_pos) {
                DrawBlockType::Empty
            } else {
                DrawBlockType::Occupied
            },
        });

        self.next_pos = self.next_pos + p(1, 0);
        if self.next_pos.x >= Field::WIDTH {
            self.next_pos = p(0, self.next_pos.y + 1);
        }
        result
    }
}

pub struct PlayingFieldRenderBlocksInstructions<'a> {
    field: &'a Field,
    controlled_minos: MinoSet,
    ghost_minos: MinoSet,
}

impl<'a> PlayingFieldRenderBlocksInstructions<'a> {
    pub fn new(field: &'a Field, controlled_minos: MinoSet, ghost_minos: MinoSet) -> Self {
        Self {
            field: field,
            controlled_minos: controlled_minos,
            ghost_minos: ghost_minos,
        }
    }
}

impl<'a> BlockRenderInstructions<PlayingFieldRenderBlocksIterator<'a>>
    for PlayingFieldRenderBlocksInstructions<'a>
{
    fn height_blocks(&self) -> Coord {
        Field::VISIBLE_HEIGHT
    }
    fn width_blocks(&self) -> Coord {
        Field::WIDTH
    }

    fn blocks(&self) -> PlayingFieldRenderBlocksIterator<'a> {
        PlayingFieldRenderBlocksIterator::<'a> {
            field: self.field,
            controlled_minos: self.controlled_minos.clone(),
            ghost_minos: self.ghost_minos.clone(),
            next_pos: p(0, 0),
        }
    }
}
