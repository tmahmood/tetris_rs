use piston::input::*;
use opengl_graphics::{ GlGraphics};
use crate::consts::*;
use graphics::color::BLACK;
use crate::Game;
use crate::board::{Board};
use crate::board::shape::Shape;


pub struct App {
    pub gl: GlGraphics, // OpenGL drawing backend.
    pub board: Board,
}

impl Game for App {
    fn keyboard(&mut self, args: &ButtonArgs) {
        if args.state == ButtonState::Release {
            match args.button {
                Button::Keyboard(k) => {
                    match k {
                        Key::R => self.board.current_shape = Shape::choose_random_shape(),
                        Key::A => self.board.speed_factor += 1.0,
                        Key::S => self.board.speed_factor -= 1.0,
                        Key::Up => self.board.current_shape.rotate_left(),
                        Key::Down => self.board.current_shape.rotate_right(),
                        Key::Left => self.board.update_current_shape_horz(-1),
                        Key::Right => self.board.update_current_shape_horz(1),
                        Key::Space => self.board.drop_fast = false,
                        _ => (),
                    };
                },
                _ => ()
            };
        } else if args.state == ButtonState::Press {
            match args.button {
                Button::Keyboard(k) => {
                    match k {
                        Key::Space => self.board.drop_fast = true,
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        let square = rectangle::square(0.0, 0.0, GR);
        let points = self.board.get_all_drawable_tiles();
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);
            points.iter().for_each(|point| {
                let transform = c.transform.trans(point.0, point.1);
                rectangle(COLORS[point.2], square, transform, gl);
            });
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.board.update(args.dt);
    }
}
