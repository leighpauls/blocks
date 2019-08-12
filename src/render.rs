use crate::position::Coord;
use crate::position::Pos;
use crate::shapes::Shape;
use crate::Game;

use quicksilver::{
    geom::{Rectangle, Transform, Vector},
    graphics::{Background, Color, FontStyle, Image},
    lifecycle::Window,
    Result,
};

use crate::resources::Images;

pub enum DrawBlockType {
    Empty,
    Occupied(Shape),
    OutOfPlay,
    GhostPiece(Shape),
    ClearingLine,
}

fn image_for_mino<'a>(images: &'a Images, b: &DrawBlockType) -> Background<'a> {
    match b {
        DrawBlockType::Empty => Background::Img(&images.empty_mino),
        DrawBlockType::Occupied(shape) => Background::Img(image_for_shape(images, *shape)),
        DrawBlockType::GhostPiece(shape) => Background::Blended(
            image_for_shape(images, *shape),
            Color::from_rgba(0x90, 0x90, 0x90, 1.0),
        ),
        DrawBlockType::OutOfPlay => Background::Col(Color::WHITE),
        DrawBlockType::ClearingLine => Background::Col(Color::from_rgba(0x80, 0x80, 0x80, 1.0)),
    }
}

fn image_for_shape<'a>(images: &'a Images, shape: Shape) -> &'a Image {
    match shape {
        Shape::I => &images.i_mino,
        Shape::O => &images.o_mino,
        Shape::T => &images.t_mino,
        Shape::S => &images.s_mino,
        Shape::Z => &images.z_mino,
        Shape::J => &images.j_mino,
        Shape::L => &images.l_mino,
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

fn render_blocks<T, I>(
    instructions: &T,
    scale_transform: Transform,
    position_transform: Transform,
    images: &Images,
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
            image_for_mino(images, &block.block_type),
            Transform::IDENTITY,
            0,
        );
    }
}

const BLOCK_SIZE_RATIO: f32 = 0.04;

pub fn draw_field(window: &mut Window, game: &Game) -> Result<()> {
    window.clear(Color::WHITE)?;

    let screen_size = &game.screen_size;
    let full_height = screen_size.y;
    let block_size = BLOCK_SIZE_RATIO * full_height;

    let render_info = game.state.render_info();

    let scale_transform = Transform::scale((block_size, block_size));
    let position_transform = Transform::translate((
        screen_size.x * 0.5 - (0.5 * block_size * render_info.playing_field.width_blocks() as f32),
        screen_size.y * 0.5 - (0.5 * block_size * render_info.playing_field.height_blocks() as f32),
    )) * scale_transform;

    render_blocks(
        &render_info.playing_field,
        scale_transform,
        position_transform,
        &game.resources.images,
        window,
    );

    let preview_block_size = 0.03 * full_height;
    let preview_scale_transform = Transform::scale((preview_block_size, preview_block_size));
    let preview_root_position =
        Transform::translate((screen_size.x * 0.7, screen_size.y * 0.2)) * preview_scale_transform;
    for (i, shape) in render_info.previews.iter().enumerate() {
        render_blocks(
            &*shape,
            preview_scale_transform,
            preview_root_position * Transform::translate((0, 3 * i as i32)),
            &game.resources.images,
            window,
        );
    }

    if let Some(hold_piece) = render_info.hold_piece {
        let hold_piece_position =
            Transform::translate((screen_size.x * 0.22, screen_size.y * 0.25))
                * preview_scale_transform;
        render_blocks(
            &hold_piece,
            preview_scale_transform,
            hold_piece_position,
            &game.resources.images,
            window,
        );
    }

    let style = FontStyle::new(24.0, Color::BLACK);
    let score_image = game.resources.font.render(
        &format!(
            "Lines: {}\nLevel: {}",
            render_info.cleared_lines, render_info.level
        ),
        &style,
    )?;

    use quicksilver::geom::Shape;
    window.draw(
        &score_image
            .area()
            .translate((screen_size.x * 0.7, screen_size.y * 0.7)),
        Background::Img(&score_image),
    );

    Ok(())
}
