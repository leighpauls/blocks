#[cfg(test)]
#[macro_use]
extern crate double;

#[cfg(test)]
#[macro_use]
extern crate hamcrest2;

extern crate quicksilver;
#[macro_use]
extern crate num_derive;
extern crate num_traits;
extern crate rand;

mod controlled;
mod field;
mod gamestate;
mod position;
mod render;
mod shapes;
mod tetromino;
mod time;

use gamestate::GameState;
use position::{RotateDir, ShiftDir};
use quicksilver::{
    geom::{Transform, Vector},
    graphics::Color,
    input::{ButtonState, Key},
    lifecycle::{run, Event, Settings, State, Window},
    Result,
};
use render::render_field;

const BLOCK_SIZE_RATIO: f32 = 0.04;

struct Game {
    // Initialzied on the first loop
    state_option: Option<(GameState, Vector)>,
}

impl Game {
    fn game_state(&mut self) -> &mut GameState {
        &mut self
            .state_option
            .as_mut()
            .expect("Getting game state beore first loop")
            .0
    }
    fn screen_size(&self) -> Vector {
        self.state_option
            .as_ref()
            .expect("Getting screen size before first loop")
            .1
    }
}

impl State for Game {
    fn new() -> Result<Game> {
        Ok(Game { state_option: None })
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        let screen_size = self.screen_size();
        let full_height = screen_size.y;
        let block_size = BLOCK_SIZE_RATIO * full_height;

        let render_info = self.game_state().render_info();

        let scale_transform = Transform::scale((block_size, block_size));
        let position_transform = Transform::translate((
            screen_size.x * 0.5 - (0.5 * block_size * render_info.field.width_blocks() as f32),
            screen_size.y * 0.5 - (0.5 * block_size * render_info.field.height_blocks() as f32),
        )) * scale_transform;

        render_field(
            render_info.field,
            scale_transform,
            position_transform,
            window,
        );
        // let position_transform = Transform::translate((block_size, block_size)) * scale_transform;

        Ok(())
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if let None = self.state_option {
            self.state_option = Some((GameState::new(), window.screen_size()));
        }

        self.game_state().update();
        Ok(())
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        let game_state = self.game_state();
        match event {
            Event::Key(Key::Left, ButtonState::Pressed) => {
                game_state.on_input_shift(ShiftDir::Left)
            }
            Event::Key(Key::Right, ButtonState::Pressed) => {
                game_state.on_input_shift(ShiftDir::Right)
            }
            Event::Key(Key::Down, ButtonState::Pressed) => game_state.on_input_soft_drop(),
            Event::Key(Key::Z, ButtonState::Pressed) => game_state.on_input_rotate(RotateDir::CCW),
            Event::Key(Key::X, ButtonState::Pressed) => game_state.on_input_rotate(RotateDir::CW),
            Event::Key(Key::Space, ButtonState::Pressed) => game_state.on_input_hard_drop(),
            Event::Key(Key::Escape, ButtonState::Pressed) => window.close(),
            _ => (),
        }
        Ok(())
    }
}

fn main() {
    run::<Game>(
        "Draw Geometry",
        Vector::new(800, 600),
        Settings {
            update_rate: 1000.0 / 120.0,
            ..Settings::default()
        },
    );
}
