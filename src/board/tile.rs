use crate::board::Point;
use crate::consts::*;

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub struct Tile {
    pub point: Point,
}

impl Tile {
    pub fn new(x: i32, y: i32) -> Tile {
        Tile { point: Point {x, y}, }
    }

    pub fn collides_with(&self, other: &Tile) -> bool {
        let rect1 = self.point.clone();
        let rect2 = other.point.clone();
        rect1.x() < rect2.x() + GR && rect1.x() + GR > rect2.x()
            && rect1.y() < rect2.y() + GR && rect1.y() + GR > rect2.y()
    }

    pub fn update(&mut self){
        // 1 tiles par sec
        //
        self.point.y += 1;
    }

    pub fn update_y(&mut self, direction: i32) {
        self.point.x += direction * 1;
    }

}
