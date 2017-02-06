extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use piston::input::Button::Keyboard;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

mod model;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {}
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [200, 200])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App { gl: GlGraphics::new(opengl) };

    let mut events = Events::new(EventSettings::new());

    let mut game_model = model::Model::new(50);

    while let Some(event) = events.next(&mut window) {
        match event {
            Input::Render(_) => {
                // Nothing yet, because that's the hard part and i'm avoiding it ;x
            },
            Input::Press(Keyboard(Key::W)) => {
                game_model.change_snake_direction(model::Direction::Up)
            },
            Input::Press(Keyboard(Key::A)) => {
                game_model.change_snake_direction(model::Direction::Left)
            },
            Input::Press(Keyboard(Key::S)) => {
                game_model.change_snake_direction(model::Direction::Right)
            },
            Input::Press(Keyboard(Key::D)) => {
                game_model.change_snake_direction(model::Direction::Down)
            },
            _ => {
            }
        }
    }
}
