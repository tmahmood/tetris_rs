
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}


impl Point {
    pub fn x(&self) -> f64 {
        GR * self.x as f64
    }

    pub fn y(&self) -> f64 {
        GR * self.y as f64
    }
}

pub mod tile;
pub mod shape;

use crate::consts::*;
use crate::board::shape::Shape;
use crate::board::tile::Tile;

pub struct Board {
    pub tiles: Vec<Tile>,
    pub current_shape: Shape,
    pub speed_factor: f64,
    pub direction: f64,
    pub time_passed: f64,
}

impl Board {
    pub fn new(speed_factor: f64) -> Board {
        Board {
            tiles: Vec::new(),
            speed_factor,
            current_shape: Shape::choose_random_shape(),
            direction: 0.0,
            time_passed: 0.0
        }
    }

    pub fn get_all_drawable_tiles(&self) -> Vec<(f64, f64)>{
        let mut points = Vec::new();
        for tile in self.tiles.iter() {
            let (x, y) = (tile.point.x(), tile.point.y());
            points.push((x, y));
        }
        self.current_shape.get_points(&mut points);
        points
    }

    pub fn update(&mut self, dt: f64) {
        self.time_passed += dt;
        self.update_vert();
    }

    fn update_vert(&mut self) {
        if self.time_passed >= 0.6 {
            if !(self.update_shape_position()) {
                self.tiles.append(self.current_shape.tiles.as_mut());
                self.current_shape = Shape::choose_random_shape();
            }
            self.time_passed = 0.0;
        }
    }

    pub fn update_shape_position(&mut self) -> bool {
        let mut shape = self.current_shape.clone();
        shape.update();
        if self.validate_shape_position(&shape) {
            self.current_shape = shape;
            return true
        }
        false
    }

    pub fn update_current_shape_horz(&mut self, direction: i32) {
        let mut shape = self.current_shape.clone();
        shape.update_vert(direction);
        if self.validate_shape_position(&shape) {
            self.current_shape = shape;
        }
    }

    fn validate_shape_position(&self, shape: &Shape) -> bool {
        if shape.tiles.iter().all(|tile| self.is_inside_boundary(tile)) {
            if shape.tiles.iter().all(|tile| self.is_valid_move(tile)) {
                return true;
            }
        }
        false
    }
    pub fn is_inside_boundary(&self, tile: &Tile) -> bool {
        tile.point.y < BOARD_HEIGHT &&
            (tile.point.x <= BOARD_WIDTH && tile.point.x >= 0)
    }

    pub fn is_valid_move(&self, tile: &Tile) -> bool {
        self.tiles.iter().all(|_tile| !_tile.collides_with(tile))
    }
}

