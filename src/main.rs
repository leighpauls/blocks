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
    geom::Vector,
    graphics::{Font},
    input::{ButtonState, Key},
    lifecycle::{run, Event, Settings, State, Window},
    Error, Future, Result,
};
use render::draw_field;

pub struct Game {
    pub state: GameState,
    pub screen_size: Vector,
    pub score_font: Font,
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

        draw_field(window, &game)
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
