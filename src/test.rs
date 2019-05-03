extern crate ordered_float;
use crate::board::{tile::Tile, Board, Point, shape::Shape};
use crate::consts::*;

#[test]
fn if_tile_can_move() {
    let mut board = Board::new(0.3);
    let tile = Tile::new(1, 8);
    assert!(board.is_valid_move(&tile));
    let tile = Tile::new(8, 8);
    assert!(board.is_valid_move(&tile));
}

#[test]
fn should_fail_to_move_when_near_border() {
    let square = Shape::new(SHAPES[3],3);
    let mut board = Board::new(0.3);
    board.current_shape = square;
    board.update_current_shape_horz(-1);
    assert_eq!(1, board.current_shape.tiles[0].point.x);
}

#[test]
fn rotate_shape() {
    let mut def_shape = Shape::new(SHAPES[1], 1);
    let rot_left_shape = Shape::new(
        [[2, 1], [2, 2], [1, 2], [1, 3]],  // z shape
        1
    );
    def_shape.rotate_left();
    assert!(
        def_shape.tiles.iter()
            .all(|tile| rot_left_shape.tiles.contains(tile)));
}

#[test]
fn norm_shape() {
    let rot_left_shape = Shape::new(
        [[2, -3], [2, -2], [1, -2], [1, 1]],  // z shape
        1
    );
    let expected = Shape::new(
        [[2, 1], [2, 2], [1, 2], [1, 3]],  // z shape
        1
    );
    assert_eq!(expected, rot_left_shape.norm());
}

#[test]
fn find_min_by_key() {
    let tiles = vec![
        Tile::new(2, -3),
        Tile::new(2, -2),
        Tile::new(1, -2),
        Tile::new(1, 1)
    ];
    let lowest = tiles.iter().min_by_key(|p| p.point.y);
    assert_eq!(*lowest.unwrap(), Tile::new(2, -3));
}

