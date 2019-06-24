// Draw some multi-colored geometry to the screen
extern crate quicksilver;

use quicksilver::{
    geom::{Rectangle, Transform, Vector},
    graphics::Color,
    input::{ButtonState, Key},
    lifecycle::{run, Event, Settings, State, Window},
    Result,
};

const FIELD_WIDTH: usize = 10;
const FIELD_HEIGHT: usize = 22;

const BLOCK_SIZE_RATIO: f32 = 0.04;

#[derive(Copy, Clone)]
enum FieldBlock {
    Empty,
    Occupied,
}

type Field = [[FieldBlock; FIELD_HEIGHT]; FIELD_WIDTH];

struct Screen {
    screen_size: Option<Vector>,
    is_first_loop: bool,
    field: Field,
}

impl State for Screen {
    fn new() -> Result<Screen> {
        Ok(Screen {
            screen_size: None,
            is_first_loop: true,
            field: [[FieldBlock::Empty; FIELD_HEIGHT]; FIELD_WIDTH],
        })
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        let screen_size = self.screen_size.expect("drawing before first update");
        let full_height = screen_size.y;
        let block_size = BLOCK_SIZE_RATIO * full_height;

        let field_transform = Transform::translate((
            screen_size.x * 0.5 - (0.5 * block_size * FIELD_WIDTH as f32),
            screen_size.y * 0.5 - (0.5 * block_size * FIELD_HEIGHT as f32),
        ));
        for y in 0..FIELD_HEIGHT {
            for x in 0..FIELD_WIDTH {
                window.draw_ex(
                    &Rectangle::new(
                        (block_size * x as f32, block_size * y as f32),
                        (block_size, block_size),
                    ),
                    match self.field[x][y] {
                        FieldBlock::Empty => Color::BLUE,
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

            self.field[3][4] = FieldBlock::Occupied;
            self.screen_size = Some(window.screen_size());
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
