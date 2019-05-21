use crate::board::Point;
use crate::consts::*;

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug, Ord, Eq)]
pub struct Tile {
    pub point: Point,
    pub shape_index: usize,
}

impl Default for Tile {
    fn default () -> Tile {
        Tile {
            point: Point{x: 0, y: 0},
            shape_index: 99
        }
    }
}

impl Tile {
    pub fn new(x: i32, y: i32, shape_index: usize) -> Tile {
        Tile {
            point: Point {x, y},
            shape_index
        }
    }

    pub fn collides_with(&self, other: &Tile) -> bool {
        let rect1 = self.point.clone();
        let rect2 = other.point.clone();
        rect1.x() < rect2.x() + GRD && rect1.x() + GRD > rect2.x()
            && rect1.y() < rect2.y() + GRD && rect1.y() + GRD > rect2.y()
    }

    pub fn update(&mut self){
        // 1 tiles par sec
        //
        self.point.y += 1;
    }

    pub fn update_x(&mut self, direction: i32) {
        self.point.x += direction * 1;
    }

}
