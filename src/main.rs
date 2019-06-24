// Draw some multi-colored geometry to the screen
extern crate quicksilver;

use quicksilver::{
    geom::{Rectangle, Transform, Vector},
    graphics::{Background::Img, Color, Image},
    input::{ButtonState, Key},
    lifecycle::{run, Asset, Event, Settings, State, Window},
    Result,
};

struct Screen {
    position: Vector,
    image: Asset<Image>,
    is_first_loop: bool,
}

impl State for Screen {
    fn new() -> Result<Screen> {
        Ok(Screen {
            position: Vector::new(50, 50),
            image: Asset::new(Image::load("image.png")),
            is_first_loop: true,
        })
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        let pos = self.position;
        self.image.execute(|loaded_image| {
            window.draw_ex(
                &Rectangle::new(pos, (100, 200)),
                Img(loaded_image),
                Transform::translate((0, 200)) * Transform::rotate(45),
                0,
            );
            Ok(())
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if self.is_first_loop {
            println!(
                "Focused! update rate: {} draw rate: {} max updates: {}",
                window.update_rate(),
                window.draw_rate(),
                window.max_updates()
            );
            self.is_first_loop = false;
        }
        Ok(())
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        if let Event::Key(key, ButtonState::Pressed) = event {
            self.position.x += match key {
                Key::Left => -50.0,
                Key::Right => 50.0,
                _ => 0.0,
            }
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
