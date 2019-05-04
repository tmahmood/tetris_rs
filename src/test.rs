extern crate ordered_float;
use crate::board::{tile::Tile, Board, shape::Shape};
use crate::consts::*;

#[test]
fn if_tile_can_move() {
    let board = Board::new(0.3);
    let tile = Tile::new(1, 8, 1);
    assert!(board.is_valid_move(&tile));
    let tile = Tile::new(8, 8, 1);
    assert!(board.is_valid_move(&tile));
}

#[test]
fn should_fail_to_move_when_passes_boundary() {
    let square = Shape::new(SHAPES[3],3);
    let mut board = Board::new(0.3);
    board.current_shape = square;
    board.update_current_shape_horz(-1);
    assert_eq!(0, board.current_shape.tiles[0].point.x);
}

#[test]
fn rotate_shape() {
    let mut board = Board::new(0.3);

    board.current_shape = Shape::new([[4, 6], [5, 6], [5, 7], [6, 7]], 1);
    let rot_left_shape = Shape::new(
        [[5,  6], [5,  7], [4,  7], [4,  8]],  // z shape rl
        1
    );
    board.rotate_left();
    let mut e = rot_left_shape.tiles.clone();
    let mut a = board.current_shape.tiles.clone();
    e.sort(); a.sort();
    assert_eq!(e, a);
}

