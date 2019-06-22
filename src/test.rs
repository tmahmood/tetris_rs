extern crate ordered_float;
use crate::board::{tile::Tile, Board, shape::Shape};
use crate::consts::*;
use graphics::line::Shape::Square;

#[test]
fn drain_vector() {
    let mut k = vec![3, 4, 1, 5, 3, 1];
    k.drain(2..4);
    assert_eq!(vec![3, 4, 3, 1], k);
}

#[test]
fn update_horizontal_times() {
    let mut current_shape = Shape::new(SHAPES[SQUARE_SHAPE], SQUARE_SHAPE);
    let mut shape = Shape::new(SHAPES[SQUARE_SHAPE], SQUARE_SHAPE);
    for _i in 0..4 {
        shape.update_horizontal(-1);
    }
    current_shape.update_horizontal_times(-1, 4);
    assert_eq!(shape, current_shape);
}

#[test]
fn update_times() {
    let mut current_shape = Shape::new(SHAPES[SQUARE_SHAPE], SQUARE_SHAPE);
    let mut shape = Shape::new(SHAPES[SQUARE_SHAPE], SQUARE_SHAPE);
    for _i in 0..4 {
        shape.update();
    }
    current_shape.update_times(4);
    assert_eq!(shape, current_shape);
}

#[test]
fn board_size() {
    let board = Board::new(0.3);
    assert_eq!((BOARD_WIDTH * BOARD_HEIGHT) + 1, board.tiles.len() as i32);
    let tile = Tile::new(BOARD_WIDTH - 1, BOARD_HEIGHT - 1, 1);
    let loc = board.arr_loc(&tile);
    assert_eq!(199, loc);
}

#[test]
fn at_most_bottom_right_pos() {
    let mut board = Board::new(0.3);
    let tile = Tile::new(6, 4, 1);
    let loc = board.arr_loc(&tile);
    board.tiles[loc] = tile;
    assert_ne!(
        Tile::new(9,19, 1),
        board.tiles[199],
    );
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
fn placement_when_tile_at_boundary() {
    let mut board = Board::new(0.3);
    let tile = Tile::new(10, 4, 1);
    let loc = board.arr_loc(&tile);
    board.tiles[loc] = tile;
    assert_eq!(
        Tile::new(10,4, 1),
        board.tiles[50],
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
    board.update_current_shape_horizontal(-1);
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

#[test]
fn moving_down_the_rows() {
    let mut board = Board::new(0.3);
    board.tiles = vec![
        Tile::new(1, 6, 1),
        Tile::new(2, 6, 1),
        Tile::new(3, 6, 1),
        Tile::new(4, 5, 1),
        Tile::new(5, 6, 1),
        Tile::new(6, 5, 1),
        Tile::new(7, 5, 1),
        Tile::new(8, 5, 1),
        Tile::new(9, 6, 1),
        Tile::new(10, 6, 1),
    ];
    board.move_down_rows_above(6);
    let final_tiles = vec![
        Tile::new(1, 6, 99),
        Tile::new(2, 6, 99),
        Tile::new(3, 6, 99),
        Tile::new(4, 6, 1),
        Tile::new(5, 6, 99),
        Tile::new(6, 6, 1),
        Tile::new(7, 6, 1),
        Tile::new(8, 6, 1),
        Tile::new(9, 6, 99),
        Tile::new(10, 6, 99),
    ];
    println!("{:?}", board.tiles);
    assert_eq!(board.tiles.len(), 10);
    assert_eq!(board.tiles, final_tiles);
}

