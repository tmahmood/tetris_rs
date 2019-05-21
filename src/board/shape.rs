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
                    Tile::new(tile[0] + 4, tile[1], num)
                ).collect(),
            tile_index: num
        }
    }

    pub fn new(tiles: [[i32; 2]; 4], tile_index: usize) -> Shape {
        Shape {
            tiles: tiles.iter()
                .map(|tile|
                    Tile::new(tile[0], tile[1], tile_index)
                ).collect(),
            tile_index
        }
    }

    pub fn update(&mut self) {
        for tile in self.tiles.iter_mut() {
            tile.update();
        }
    }

    pub fn update_times(&mut self, times: i32) {
        for _time in 0..times {
            self.update();
        }
    }


    pub fn update_horizontal(&mut self, direction: i32) {
        for tile in self.tiles.iter_mut() {
            tile.update_x(direction);
        }
    }

    pub fn update_horizontal_times(&mut self, direction: i32, times: i32) {
        for _time in 0..times {
            self.update_horizontal(direction);
        }
    }
}

