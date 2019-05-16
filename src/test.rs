extern crate ordered_float;
use crate::board::{tile::Tile, Board, shape::Shape};
use crate::consts::*;

#[test]
fn board_size() {
    let board = Board::new(0.3);
    assert_eq!(BOARD_WIDTH * BOARD_HEIGHT, board.tiles.len());
}

#[test]
fn set_tile_at_position() {
    let mut board = Board::new(0.3);
    let tile = Tile::new(6, 4, 1);
    let loc = board.arr_loc(&tile);
    board.tiles[loc] = tile;
    assert_eq!(
        Tile::new(6,4, 1),
        board.tiles[46],
    );
    let tile = Tile::new(7, 12, 1);
    let loc = board.arr_loc(&tile);
    board.tiles[loc] = tile;
    assert_eq!(
        Tile::new(7,12, 1),
        board.tiles[127]
    );
}

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

#[test]
fn keep_track_of_lines() {
    let mut board = Board::new(0.3);
    board.current_shape = Shape::new([[4, 6], [5, 6], [5, 7], [6, 7]], 1);
    board.place_shape();
    assert_eq!(board.lines[6], 2);
    assert_eq!(board.lines[7], 2);
}


#[test]
fn line_count_after_placing() {
    let mut board = Board::new(0.3);
    board.current_shape = Shape::new([ [1, 6], [2, 6], [3, 6], [4, 6]], 0 );
    board.place_shape();
    assert_eq!(4, board.lines[6]);
}

#[test]
fn when_all_lines_are_filled() {
    let mut board = Board::new(0.3);
    board.tiles = vec![
        Tile::new(1, 6, 1),
        Tile::new(2, 6, 1),
        Tile::new(3, 6, 1),
        Tile::new(4, 6, 1),
        Tile::new(5, 6, 1),
        Tile::new(6, 6, 1),
        Tile::new(7, 5, 1),
        Tile::new(7, 6, 1),
        Tile::new(8, 6, 1),
        Tile::new(9, 6, 1),
        Tile::new(10, 6, 1),
    ];
    board.lines[6] = 10;
    board.remove_completed_lines();
    assert_eq!(1, board.tiles.len());
    assert_eq!(1, board.lines[6]);
}

