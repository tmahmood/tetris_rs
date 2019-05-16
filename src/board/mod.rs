
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug, Eq, Ord, Default)]
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
    pub next_shape: Shape,
    pub speed_factor: f64,
    pub direction: f64,
    pub time_passed: f64,
    pub drop_fast: bool,
    pub lines: Vec<i32>,
    pub score: i32,
}

impl Board {
    pub fn new(speed_factor: f64) -> Board {
        Board {
            tiles: vec![Tile::default(); (BOARD_HEIGHT * BOARD_WIDTH + 1) as usize],
            speed_factor,
            current_shape: Shape::choose_random_shape(),
            next_shape: Shape::choose_random_shape(),
            direction: 0.0,
            time_passed: 0.0,
            drop_fast: false,
            score: 0,
            lines: vec![0;20]
        }
    }

    pub fn arr_loc(&self, tile: &Tile) -> usize {
        ((tile.point.y * BOARD_WIDTH) + tile.point.x) as usize
    }

    pub fn get_all_drawable_tiles(&self) -> Vec<(f64, f64, usize)>{
        let mut points = Vec::new();
        let push = |tile: &Tile| (
            tile.point.x(), tile.point.y(), tile.shape_index
        );
        points.extend(self.tiles.iter().filter(|tile| tile.shape_index < 99).map(push));
        points.extend(self.current_shape.tiles.iter().map(push));
        points.extend(self.next_shape.tiles.iter().map(|tile| (
            tile.point.x() + 7.0 * GR, tile.point.y(), tile.shape_index,
        )));
        points
    }

    pub fn update(&mut self, dt: f64) {
        self.time_passed += dt;
        self.update_vert();
    }

    pub fn update_vert(&mut self) {
        if self.time_passed < 0.6 && !self.drop_fast{
            return;
        }
        if !(self.update_shape_position()) {
            self.place_shape()
        }
        self.time_passed = 0.0;
    }

    pub fn place_shape(&mut self) {
        self.drop_fast = false;
        for tile in self.current_shape.tiles.iter() {
            self.lines[tile.point.y as usize] += 1;
            let loc = self.arr_loc(tile);
            self.tiles[loc] = *tile;
        }
        self.current_shape = self.next_shape.clone();
        self.next_shape = Shape::choose_random_shape();
    }

    pub fn remove_completed_lines(&mut self) {
        for i in self.lines.len() - 1..0 {
            let line = self.lines[i];
            if line < 10 {
                continue;
            }
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
        shape.update_horz(direction);
        if self.validate_shape_position(&shape) {
            self.current_shape = shape;
        }
    }

    fn validate_shape_position(&self, shape: &Shape) -> bool {
        if !shape.tiles.iter().all(|tile| self.is_inside_boundary(tile)) {
            return false;
        }
        shape.tiles.iter().all(|tile| self.is_valid_move(tile))
    }
    pub fn is_inside_boundary(&self, tile: &Tile) -> bool {
        tile.point.y < BOARD_HEIGHT &&
            (tile.point.x <= BOARD_WIDTH && tile.point.x >= 0)
    }

    pub fn is_valid_move(&self, tile: &Tile) -> bool {
        self.tiles.iter().all(|_tile| !_tile.collides_with(tile))
    }

    pub fn rotate_left(&mut self) {
        let mut shape = self.current_shape.clone();
        let tiles: Vec<Tile> = shape.tiles.iter().map(|tile| {
            Tile::new( tile.point.y, -tile.point.x,
                shape.tile_index)
        }).collect();
        if self.trans(tiles, &mut shape) {
            self.current_shape = shape;
        }
    }

    pub fn rotate_right(&mut self) {
        let mut shape = self.current_shape.clone();
        let tiles: Vec<Tile> = shape.tiles.iter().map(|tile| {
            Tile::new( -tile.point.y, tile.point.x,
                shape.tile_index)
        }).collect();
        if self.trans(tiles, &mut shape) {
            self.current_shape = shape;
        }
    }
    pub fn trans(&mut self, mut tiles: Vec<Tile>, shape: &mut Shape) -> bool {
        let lowest_new_x = tiles.iter()
            .min_by_key(|p| p.point.x)
            .unwrap().point.x;
        let lowest_new_y = tiles.iter()
            .min_by_key(|p| p.point.y)
            .unwrap().point.y;
        let lowest_x = self.current_shape.tiles.iter()
            .min_by_key(|p| p.point.x)
            .unwrap().point.x;
        let lowest_y = self.current_shape.tiles.iter()
            .min_by_key(|p| p.point.y)
            .unwrap().point.y;
        let move_vec = Point {
            x: lowest_x - lowest_new_x,
            y: lowest_y - lowest_new_y
        };
        tiles = tiles.iter().map(|tile|{
            Tile::new(
                move_vec.x + tile.point.x,
                move_vec.y + tile.point.y,
                tile.shape_index
            )
        }).collect();
        shape.tiles = tiles;
        self.validate_shape_position(shape)
    }
}

