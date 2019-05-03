extern crate rand;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::{GlutinWindow as Window};
use opengl_graphics::{OpenGL};

pub trait Game {
    fn keyboard(&mut self, button_args: &ButtonArgs);
    fn render(&mut self, render_args: &RenderArgs);
    fn update(&mut self, update_args: &UpdateArgs);
}

pub fn game_loop<T: Game>(app: &mut T, window: &mut Window) {
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(window) {
        if let Some(k) = e.button_args() {
            app.keyboard(&k);
        }
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}

pub fn get_window(
    opengl: &OpenGL,
    size:[u32;2],
    title: &'static str
) -> Window {
    WindowSettings::new(
        title,
        size)
        .opengl(*opengl).exit_on_esc(true)
        .build().unwrap()
}

pub mod consts;
pub mod board;
pub mod app;

#[cfg(test)]
pub mod test;

