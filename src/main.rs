// Draw some multi-colored geometry to the screen
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
    screen_size_option: Option<Vector>,
    is_first_loop: bool,
    field: Field,
    controlled_blocks: ControlledBlocks,
}

impl Game {
    fn handle_drop(&mut self, drop_result: DropResult) {
        if let DropResult::Continue = drop_result {
            // These blocks are still dropping
            return;
        }
        for pos in self.controlled_blocks.positions().iter() {
            self.field.set(*pos, FieldBlock::Occupied);
        }

        // Replace the stopped blocks with new ones
        self.controlled_blocks = ControlledBlocks::new();
        self.controlled_blocks.start();
    }
}

impl State for Game {
    fn new() -> Result<Game> {
        Ok(Game {
            screen_size_option: None,
            is_first_loop: true,
            field: Field::new(),
            controlled_blocks: ControlledBlocks::new(),
        })
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        let screen_size = self
            .screen_size_option
            .expect("drawing before first update");
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
                    match self.field.at(Pos::new(x, y)) {
                        FieldBlock::Empty => {
                            if self.controlled_blocks.is_controlled(Pos::new(x, y)) {
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
        if self.is_first_loop {
            self.is_first_loop = false;
            println!(
                "update rate: {} draw rate: {} max updates: {} resize: {:?}",
                window.update_rate(),
                window.draw_rate(),
                window.max_updates(),
                window.resize_strategy()
            );

            self.field.set(Pos::new(3, 4), FieldBlock::Occupied);
            self.screen_size_option = Some(window.screen_size());
            self.controlled_blocks.start();
        }

        let drop_result = self.controlled_blocks.maybe_periodic_drop(&self.field);
        self.handle_drop(drop_result);

        Ok(())
    }

    fn event(&mut self, event: &Event, _window: &mut Window) -> Result<()> {
        match event {
            Event::Key(Key::Left, ButtonState::Pressed) => {
                self.controlled_blocks.shift(&self.field, ShiftDir::Left)
            }
            Event::Key(Key::Right, ButtonState::Pressed) => {
                self.controlled_blocks.shift(&self.field, ShiftDir::Right)
            }
            Event::Key(Key::Down, ButtonState::Pressed) => {
                let drop_result = self.controlled_blocks.manual_soft_drop(&self.field);
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
