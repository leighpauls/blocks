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
    graphics::{Font, Image},
    input::{ButtonState, Key},
    lifecycle::{run, Event, Settings, State, Window},
    Error, Future, Result,
};
use render::{draw_field, Images};
use time::{GameClock, PausedClock};

pub struct Game {
    pub state: GameState,
    pub screen_size: Vector,
    pub score_font: Font,
    pub images: Images,
}

type FontFuture = Box<Future<Item = Font, Error = Error>>;
type ImageFuture = Box<Future<Item = Image, Error = Error>>;

enum GameScreen {
    Loading(FontFuture, ImageFuture),
    Playing(Game, GameClock),
    Paused(Game, PausedClock),
    Won(Game),
    Lost(Game),
    Swap,
}

impl GameScreen {
    fn evolve(&mut self, window: &Window) {
        *self = match std::mem::replace(self, GameScreen::Swap) {
            GameScreen::Loading(mut font_future, mut empty_future) => {
                match (font_future.poll(), empty_future.poll()) {
                    (Ok(Async::Ready(font)), Ok(Async::Ready(empty_mino))) => {
                        let (game_state, clock) = GameState::new();
                        GameScreen::Playing(
                            Game {
                                state: game_state,
                                screen_size: window.screen_size(),
                                score_font: font,
                                images: Images {
                                    empty_mino: empty_mino,
                                },
                            },
                            clock,
                        )
                    }
                    _ => GameScreen::Loading(font_future, empty_future),
                }
            }
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
            loading_game: GameScreen::Loading(
                Box::new(Font::load("Roboto-Medium.ttf")),
                Box::new(Image::load("empty_mino.png")),
            ),
        })
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        match &self.loading_game {
            GameScreen::Playing(g, _)
            | GameScreen::Paused(g, _)
            | GameScreen::Won(g)
            | GameScreen::Lost(g) => draw_field(window, g),
            _ => Ok(()),
        }
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.loading_game.evolve(window);

        self.loading_game = match std::mem::replace(&mut self.loading_game, GameScreen::Swap) {
            GameScreen::Playing(mut game, clock) => {
                match game.state.update(window.keyboard(), clock.now()) {
                    GameCondition::Won => GameScreen::Won(game),
                    GameCondition::Lost => GameScreen::Lost(game),
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
