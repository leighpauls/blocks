use crate::position::Coord;
use crate::position::Pos;
use crate::shapes::Shape;

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
