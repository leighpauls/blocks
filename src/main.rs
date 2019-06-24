// Draw some multi-colored geometry to the screen
extern crate quicksilver;

use quicksilver::{
    geom::{Rectangle, Transform, Vector},
    graphics::{Background::Img, Color, Image},
    input::{ButtonState, Key},
    lifecycle::{run, Asset, Event, Settings, State, Window},
    Result,
};

const FIELD_WIDTH: usize = 10;
const FIELD_HEIGHT: usize = 22;

const BLOCK_SIZE: f32 = 20.0;

#[derive(Copy, Clone)]
enum FieldBlock {
    Empty,
    Occupied,
}

type Field = [[FieldBlock; FIELD_HEIGHT]; FIELD_WIDTH];

struct Screen {
    is_first_loop: bool,
    field: Field,
}

impl State for Screen {
    fn new() -> Result<Screen> {
        Ok(Screen {
            is_first_loop: true,
            field: [[FieldBlock::Empty; FIELD_HEIGHT]; FIELD_WIDTH],
        })
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        for y in 0..FIELD_HEIGHT {
            for x in 0..FIELD_WIDTH {
                window.draw(
                    &Rectangle::new(
                        (BLOCK_SIZE * x as f32, BLOCK_SIZE * y as f32),
                        (BLOCK_SIZE, BLOCK_SIZE),
                    ),
                    match self.field[x][y] {
                        FieldBlock::Empty => Color::BLUE,
                        FieldBlock::Occupied => Color::RED,
                    },
                );
            }
        }
        Ok(())
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if self.is_first_loop {
            self.is_first_loop = false;
            println!(
                "Focused! update rate: {} draw rate: {} max updates: {}",
                window.update_rate(),
                window.draw_rate(),
                window.max_updates(),
            );

            self.field[3][4] = FieldBlock::Occupied;
        }
        Ok(())
    }

    fn event(&mut self, event: &Event, _window: &mut Window) -> Result<()> {
        if let Event::Key(key, ButtonState::Pressed) = event {
            let target = &mut self.field[5][6];
            *target = match key {
                Key::Left => FieldBlock::Occupied,
                Key::Right => FieldBlock::Empty,
                _ => *target,
            };
        }
        Ok(())
    }
}

fn main() {
    run::<Screen>(
        "Draw Geometry",
        Vector::new(800, 600),
        Settings {
            update_rate: 1000.0 / 120.0,
            ..Settings::default()
        },
    );
}
