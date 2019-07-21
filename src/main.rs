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
use gamestate::{GameCondition, GameState};
use quicksilver::{
    geom::Vector,
    graphics::Font,
    input::{ButtonState, Key},
    lifecycle::{run, Event, Settings, State, Window},
    Error, Future, Result,
};
use render::draw_field;
use time::{GameClock, PausedClock};

pub struct Game {
    pub state: GameState,
    pub screen_size: Vector,
    pub score_font: Font,
}

type FontFuture = Box<Future<Item = Font, Error = Error>>;

enum GameScreen {
    Loading(FontFuture),
    Playing(Game, GameClock),
    Paused(Game, PausedClock),
    Won,
    Lost,
    Swap,
}

impl GameScreen {
    fn evolve(&mut self, window: &Window) {
        *self = match std::mem::replace(self, GameScreen::Swap) {
            GameScreen::Loading(mut font_future) => match font_future.poll() {
                Ok(Async::Ready(font)) => {
                    let (game_state, clock) = GameState::new();
                    GameScreen::Playing(
                        Game {
                            state: game_state,
                            screen_size: window.screen_size(),
                            score_font: font,
                        },
                        clock,
                    )
                }
                _ => GameScreen::Loading(font_future),
            },
            other => other,
        };
    }
}

struct GameWrapper {
    // Initialzied on the first loop
    loading_game: GameScreen,
}

impl State for GameWrapper {
    fn new() -> Result<GameWrapper> {
        Ok(GameWrapper {
            loading_game: GameScreen::Loading(Box::new(Font::load("Roboto-Medium.ttf"))),
        })
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        match &self.loading_game {
            GameScreen::Playing(g, _) | GameScreen::Paused(g, _) => draw_field(window, g),
            _ => Ok(()),
        }
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.loading_game.evolve(window);

        self.loading_game = match std::mem::replace(&mut self.loading_game, GameScreen::Swap) {
            GameScreen::Playing(mut game, clock) => {
                match game.state.update(window.keyboard(), clock.now()) {
                    GameCondition::Won => GameScreen::Won,
                    GameCondition::Lost => GameScreen::Lost,
                    GameCondition::Playing => GameScreen::Playing(game, clock),
                }
            }
            other => other,
        };

        Ok(())
    }

    fn event(&mut self, event: &Event, _window: &mut Window) -> Result<()> {
        match event {
            Event::Key(Key::Escape, ButtonState::Pressed) => {
                self.loading_game =
                    match std::mem::replace(&mut self.loading_game, GameScreen::Swap) {
                        GameScreen::Playing(g, c) => GameScreen::Paused(g, c.pause()),
                        GameScreen::Paused(g, c) => GameScreen::Playing(g, c.resume()),
                        other => other,
                    };
            }
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
