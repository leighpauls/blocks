#[cfg(test)]
#[macro_use]
extern crate double;

extern crate quicksilver;

mod controlled;
mod field;
mod position;

use controlled::{ControlledBlocks, DropResult};
use field::{Field, FieldBlock};
use position::{Pos, ShiftDir};
use quicksilver::{
    geom::{Rectangle, Transform, Vector},
    graphics::Color,
    input::{ButtonState, Key},
    lifecycle::{run, Event, Settings, State, Window},
    Result,
};

const BLOCK_SIZE_RATIO: f32 = 0.04;

struct Game {
    // Initialzied on the first loop
    game_state_option: Option<GameState>,
}

struct GameState {
    screen_size: Vector,
    field: Field,
    controlled_blocks: ControlledBlocks,
}

impl Game {
    fn handle_drop(&mut self, drop_result: DropResult) {
        if let DropResult::Continue = drop_result {
            // These blocks are still dropping
            return;
        }
        for pos in self.game_state().controlled_blocks.positions().iter() {
            self.game_state().field.set(*pos, FieldBlock::Occupied);
        }

        // Replace the stopped blocks with new ones
        self.game_state().controlled_blocks = ControlledBlocks::new();
    }

    fn game_state(&mut self) -> &mut GameState {
        self.game_state_option
            .as_mut()
            .expect("Getting game state beore first loop")
    }
}

impl State for Game {
    fn new() -> Result<Game> {
        Ok(Game {
            game_state_option: None,
        })
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        let screen_size = self.game_state().screen_size;
        let full_height = screen_size.y;
        let block_size = BLOCK_SIZE_RATIO * full_height;

        let field_transform = Transform::translate((
            screen_size.x * 0.5 - (0.5 * block_size * field::WIDTH as f32),
            screen_size.y * 0.5 - (0.5 * block_size * field::HEIGHT as f32),
        ));
        for y in 0..field::HEIGHT {
            for x in 0..field::WIDTH {
                window.draw_ex(
                    &Rectangle::new(
                        (block_size * x as f32, block_size * y as f32),
                        (block_size, block_size),
                    ),
                    match self.game_state().field.at(Pos::new(x, y)) {
                        FieldBlock::Empty => {
                            if self
                                .game_state()
                                .controlled_blocks
                                .is_controlled(Pos::new(x, y))
                            {
                                Color::GREEN
                            } else {
                                Color::BLUE
                            }
                        }
                        FieldBlock::Occupied => Color::RED,
                    },
                    field_transform,
                    0,
                );
            }
        }
        Ok(())
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if let None = self.game_state_option {
            self.game_state_option = Some(GameState {
                screen_size: window.screen_size(),
                field: Field::new(),
                controlled_blocks: ControlledBlocks::new(),
            });
        }

        let game_state = self.game_state();
        let drop_result = game_state
            .controlled_blocks
            .maybe_periodic_drop(&game_state.field);
        self.handle_drop(drop_result);

        Ok(())
    }

    fn event(&mut self, event: &Event, _window: &mut Window) -> Result<()> {
        let game_state = self.game_state();
        match event {
            Event::Key(Key::Left, ButtonState::Pressed) => game_state
                .controlled_blocks
                .shift(&game_state.field, ShiftDir::Left),
            Event::Key(Key::Right, ButtonState::Pressed) => game_state
                .controlled_blocks
                .shift(&game_state.field, ShiftDir::Right),
            Event::Key(Key::Down, ButtonState::Pressed) => {
                let drop_result = game_state
                    .controlled_blocks
                    .manual_soft_drop(&game_state.field);
                self.handle_drop(drop_result)
            }
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
