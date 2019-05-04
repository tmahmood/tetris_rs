use crate::board::tile::Tile;
use crate::board::Point;
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

    pub fn get_points(&self, points: &mut Vec<(f64, f64, usize)>) {
        for tile in self.tiles.iter() {
            points.push((
                tile.point.x(),
                tile.point.y(),
                tile.shape_index
            ));
        }
    }

    pub fn update(&mut self) {
        for tile in self.tiles.iter_mut() {
            tile.update();
        }
    }

    pub fn update_horz(&mut self, direction: i32) {
        for tile in self.tiles.iter_mut() {
            tile.update_x(direction);
        }
    }

    pub fn rotate_left(&mut self) {
        let tiles: Vec<Tile> = self.tiles.iter().map(|tile| {
            Tile::new(tile.point.y, -tile.point.x, self.tile_index)
        }).collect();
        self.trans(tiles);
    }

    pub fn rotate_right(&mut self) {
        let tiles: Vec<Tile> = self.tiles.iter().map(|tile| {
            Tile::new(
                -tile.point.y,
                tile.point.x,
                self.tile_index)
        }).collect();
        self.trans(tiles);
    }
    pub fn trans(&mut self, mut tiles: Vec<Tile>) {
        let lowest_new_x = tiles.iter()
            .min_by_key(|p| p.point.x)
            .unwrap().point.x;
        let lowest_new_y = tiles.iter()
            .min_by_key(|p| p.point.y)
            .unwrap().point.y;
        let lowest_x = self.tiles.iter()
            .min_by_key(|p| p.point.x)
            .unwrap().point.x;
        let lowest_y = self.tiles.iter()
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
        self.tiles = tiles;
    }
}

