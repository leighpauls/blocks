#![no_std]

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
extern crate getrandom;
extern crate num_traits;
#[macro_use]
extern crate alloc;

mod controlled;
mod field;
mod gamestate;
mod input;
mod keybindings;
mod lockdelay;
mod position;
mod random_bag;
mod render;
mod resources;
mod shapes;
mod tetromino;
mod time;

use alloc::boxed::Box;
use futures::Async;
use gamestate::{GameCondition, GameState};
use quicksilver::{
    geom::Vector,
    input::{ButtonState, Key},
    lifecycle::{run, Event, Settings, State, Window},
    Result,
};
use render::draw_field;
use resources::{ResourceFuture, Resources};
use time::{GameClock, PausedClock};

pub struct Game {
    pub state: GameState,
    pub screen_size: Vector,
    pub resources: Resources,
}

enum GameScreen {
    Loading(Box<ResourceFuture>),
    Playing(Game, GameClock),
    Paused(Game, PausedClock),
    Won(Game),
    Lost(Game),
    Swap,
}

impl GameScreen {
    fn evolve(&mut self, window: &Window) {
        *self = match core::mem::replace(self, GameScreen::Swap) {
            GameScreen::Loading(mut resource_future) => match resource_future.poll() {
                Ok(Async::Ready(resources)) => {
                    let (game_state, clock) = GameState::new();
                    GameScreen::Playing(
                        Game {
                            state: game_state,
                            screen_size: window.screen_size(),
                            resources: resources,
                        },
                        clock,
                    )
                }
                _ => GameScreen::Loading(resource_future),
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
            loading_game: GameScreen::Loading(Box::new(resources::load_resources())),
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

        self.loading_game = match core::mem::replace(&mut self.loading_game, GameScreen::Swap) {
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
                    match core::mem::replace(&mut self.loading_game, GameScreen::Swap) {
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
