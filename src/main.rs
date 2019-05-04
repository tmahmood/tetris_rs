extern crate tetris_rs;
extern crate rand;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use tetris_rs::app::App;
use tetris_rs::board::Board;
use tetris_rs::{get_window, game_loop};
use tetris_rs::consts::ACTUAL_SIZE;

fn main() {

    let opengl = OpenGL::V4_5;
    let mut window: Window = get_window(&opengl, ACTUAL_SIZE, "Tetris");
    let mut app: App = App{
        gl: GlGraphics::new(opengl),
        board: Board::new(5.0),
    };
    game_loop(&mut app, &mut window);
}

