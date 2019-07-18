#[cfg(test)]
#[macro_use]
extern crate double;

#[cfg(test)]
#[macro_use]
extern crate hamcrest2;

extern crate quicksilver;
#[macro_use]
extern crate num_derive;
extern crate futures;
extern crate num_traits;
extern crate rand;

mod controlled;
mod field;
mod gamestate;
mod input;
mod keybindings;
mod lockdelay;
mod position;
mod random_bag;
mod render;
mod shapes;
mod tetromino;
mod time;

use futures::Async;
use gamestate::GameState;
use quicksilver::{
    geom::{Shape, Transform, Vector},
    graphics::{Background::Img, Color, Font, FontStyle},
    input::{ButtonState, Key},
    lifecycle::{run, Event, Settings, State, Window},
    Error, Future, Result,
};
use render::{render_blocks, BlockRenderInstructions};

const BLOCK_SIZE_RATIO: f32 = 0.04;

struct Game {
    state: GameState,
    screen_size: Vector,
    score_font: Font,
}

type FontFuture = Box<Future<Item = Font, Error = Error>>;

enum LoadingGame {
    InProgress(FontFuture),
    Loaded(Game),
    Swap,
}

impl LoadingGame {
    fn evolve(&mut self, window: &Window) -> Result<()> {
        if let LoadingGame::Loaded(_) = self {
            return Ok(());
        }

        let mut previous = std::mem::replace(self, LoadingGame::Swap);
        if let LoadingGame::InProgress(ref mut font_future) = previous {
            if let Async::Ready(font) = font_future.poll()? {
                *self = LoadingGame::Loaded(Game {
                    state: GameState::new(),
                    screen_size: window.screen_size(),
                    score_font: font,
                });
            }
        }
        Ok(())
    }
}

struct GameWrapper {
    // Initialzied on the first loop
    loading_game: LoadingGame,
}

impl State for GameWrapper {
    fn new() -> Result<GameWrapper> {
        Ok(GameWrapper {
            loading_game: LoadingGame::InProgress(Box::new(Font::load("Roboto-Medium.ttf"))),
        })
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        let game: &mut Game = match self.loading_game {
            LoadingGame::Loaded(ref mut g) => g,
            _ => {
                return Ok(());
            }
        };

        window.clear(Color::WHITE)?;

        let screen_size = game.screen_size;
        let full_height = screen_size.y;
        let block_size = BLOCK_SIZE_RATIO * full_height;

        let render_info = game.state.render_info();

        let scale_transform = Transform::scale((block_size, block_size));
        let position_transform = Transform::translate((
            screen_size.x * 0.5
                - (0.5 * block_size * render_info.playing_field.width_blocks() as f32),
            screen_size.y * 0.5
                - (0.5 * block_size * render_info.playing_field.height_blocks() as f32),
        )) * scale_transform;

        render_blocks(
            &render_info.playing_field,
            scale_transform,
            position_transform,
            window,
        );

        let preview_block_size = 0.03 * full_height;
        let preview_scale_transform = Transform::scale((preview_block_size, preview_block_size));
        let preview_root_position =
            Transform::translate((screen_size.x * 0.7, screen_size.y * 0.2))
                * preview_scale_transform;
        for (i, shape) in render_info.previews.iter().enumerate() {
            render_blocks(
                &*shape,
                preview_scale_transform,
                preview_root_position * Transform::translate((0, 3 * i as i32)),
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
                window,
            );
        }

        let style = FontStyle::new(24.0, Color::BLACK);
        let score_image = game.score_font.render(
            &format!(
                "Lines: {}\nLevel: {}",
                render_info.cleared_lines, render_info.level
            ),
            &style,
        )?;
        window.draw(
            &score_image
                .area()
                .translate((screen_size.x * 0.7, screen_size.y * 0.7)),
            Img(&score_image),
        );

        Ok(())
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.loading_game.evolve(window)?;

        if let LoadingGame::Loaded(ref mut game) = self.loading_game {
            game.state.update(window.keyboard());
        }

        Ok(())
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match event {
            Event::Key(Key::Escape, ButtonState::Pressed) => window.close(),
            _ => (),
        }
        Ok(())
    }
}

fn main() {
    run::<GameWrapper>(
        "Blocks",
        Vector::new(800, 600),
        Settings {
            update_rate: 1000.0 / 120.0,
            ..Settings::default()
        },
    );
}
