use crate::field::{CheckableField, Field};
use crate::position::p;
use crate::position::Coord;
use crate::position::Pos;
use crate::shapes::{MinoSet, Shape};

use quicksilver::{
    geom::{Rectangle, Transform, Vector},
    graphics::Color,
    lifecycle::Window,
};

pub enum DrawBlockType {
    Empty,
    Occupied(Shape),
    OutOfPlay,
    GhostPiece(Shape),
}

impl DrawBlockType {
    pub fn color(&self) -> Color {
        match self {
            DrawBlockType::Empty => Color::BLACK,
            DrawBlockType::OutOfPlay => Color::WHITE,
            DrawBlockType::GhostPiece(shape) => DrawBlockType::Occupied(*shape)
                .color()
                .multiply(Color::from_rgba(0x80, 0x80, 0x80, 1.0)),
            DrawBlockType::Occupied(shape) => match *shape {
                Shape::I => Color::CYAN,
                Shape::O => Color::YELLOW,
                Shape::T => Color::PURPLE,
                Shape::S => Color::GREEN,
                Shape::Z => Color::RED,
                Shape::J => Color::BLUE,
                Shape::L => Color::ORANGE,
            },
        }
    }
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
            block.block_type.color(),
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
                DrawBlockType::Occupied(self.controlled_minos.shape())
            } else if self.ghost_minos.contains(self.next_pos) {
                DrawBlockType::GhostPiece(self.ghost_minos.shape())
            } else if self.next_pos.y >= Field::PLAYING_BOUNDARY_HEIGHT {
                DrawBlockType::OutOfPlay
            } else if self.field.is_open(self.next_pos) {
                DrawBlockType::Empty
            } else {
                DrawBlockType::Occupied(self.field.shape_at(self.next_pos))
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
