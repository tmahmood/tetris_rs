pub const FPS: f64 = 10.00;
pub const BOARD_WIDTH: i32 = 10;
pub const BOARD_HEIGHT: i32 = 20;
pub const ACTUAL_SIZE: [u32; 2] = [700, 830];
pub const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const GRAY: [f32; 4] =  [0.20, 0.20, 0.20, 1.0];
pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
// {{ we make this section customizable later
pub const GRD: f64 = 40.0;
pub const G00: f64 = 0.0;
pub const G01: f64 = 1.0;
pub const G02: f64 = 2.0;
pub const G03: f64 = 3.0;
pub const G04: f64 = 4.0;
pub const G05: f64 = 5.0;
pub const G06: f64 = 6.0;
pub const G07: f64 = 7.0;
pub const G08: f64 = 8.0;
pub const G09: f64 = 9.0;
pub const G10: f64 = 10.0;
//
pub const LINE_SHAPE: usize = 0;
pub const Z_SHAPE: usize = 1;
pub const S_SHAPE: usize = 2;
pub const SQUARE_SHAPE: usize = 3;
pub const MIR_L_SHAPE: usize = 4;
pub const L_SHAPE: usize = 5;
pub const TRI_SHAPE: usize = 6;


pub const SHAPES: [[[i32;2]; 4]; 7] = [
    [[0, 0], [1, 0], [2, 0], [3, 0]],  // line shape
    [[0, 0], [1, 0], [1, 1], [2, 1]],  // z shape
    [[0, 1], [1, 1], [1, 0], [2, 0]],  // s shape
    [[0, 0], [0, 1], [1, 1], [1, 0]],  // square shape
    [[1, 0], [1, 1], [1, 2], [0, 2]],  // Mirror L shape
    [[0, 0], [0, 1], [0, 2], [1, 2]],  // L shape
    [[0, 1], [1, 1], [2, 1], [1, 0]],  // tri shape
];

pub const COLORS: [[f32;4]; 7] = [
    [ 00.0/255.0, 170.0/255.0, 255.0/255.0, 1.0], // l
    [255.0/255.0,  78.0/255.0,  41.0/255.0, 1.0], // z
    [ 28.0/255.0, 173.0/255.0, 107.0/255.0, 1.0], // s
    [313.0/255.0,  28.0/255.0, 118.0/255.0, 1.0], // q
    [153.0/255.0, 223.0/255.0,  73.0/255.0, 1.0], // ml
    [ 98.0/255.0,  89.0/255.0, 201.0/255.0, 1.0], // L
    [241.0/255.0,  76.0/255.0, 126.0/255.0, 1.0]  // t
];
