use crate::board::tile::Tile;
use crate::consts::*;
use rand::Rng;


#[derive(Clone, PartialOrd, PartialEq, Debug)]
pub struct Shape {
    pub tiles: Vec<Tile>,
    pub tile_index: usize
}

impl Shape {

    pub fn choose_random_shape() -> Shape {
        let mut rng = rand::thread_rng();
        let num = rng.gen_range(0, SHAPES.len());
        Shape {
            tiles: SHAPES[num].iter()
                .map(|tile|
                    Tile::new(tile[0] + 4, tile[1])
                ).collect(),
            tile_index: num
        }
    }

    pub fn new(tiles: [[i32; 2]; 4], tile_index: usize) -> Shape {
        Shape {
            tiles: tiles.iter()
                .map(|tile|
                    Tile::new(tile[0], tile[1])
                ).collect(),
            tile_index
        }
    }

    pub fn get_points(&self, points: &mut Vec<(f64, f64)>) {
        for tile in self.tiles.iter() {
            let (x, y) = (tile.point.x(), tile.point.y());
            points.push((x, y));
        }
    }

    pub fn update(&mut self) {
        for tile in self.tiles.iter_mut() {
            tile.update();
        }
    }

    pub fn update_vert(&mut self, direction: i32) {
        for tile in self.tiles.iter_mut() {
            tile.update_y(direction);
        }
    }

    pub fn rotate_left(&mut self) {
        self.tiles = self.tiles.iter_mut().map(|tile| {
            Tile::new(tile.point.y, -tile.point.x)
        }).collect();
    }

    pub fn norm(&self) -> Shape {
        let mut shape = self.clone();
        let lowest = shape.tiles.iter().min_by_key(|p| p.point.y);
        shape
    }

}

