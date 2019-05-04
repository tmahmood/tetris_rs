pub const FPS: f64 = 10.00;
pub const BOARD_WIDTH: i32 = 10;
pub const BOARD_HEIGHT: i32 = 20;
pub const ACTUAL_SIZE: [u32; 2] = [700, 830];
pub const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const GRAY: [f32; 4] =  [0.20, 0.20, 0.20, 1.0];
pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
// {{ we make this section customizable later
pub const ZR: f64 = 0.0;
pub const GR: f64 = 40.0;
pub const G0: f64 = 1.0;
pub const G1: f64 = 2.0;
pub const G2: f64 = 3.0;
pub const G3: f64 = 4.0;
pub const G4: f64 = 5.0;
pub const G5: f64 = 6.0;
pub const G6: f64 = 7.0;
pub const G7: f64 = 8.0;
pub const G8: f64 = 9.0;
pub const G9: f64 = 10.0;
//
pub const EMPTY_SHAPE: [[i32;2]; 4] = [[0, 0], [0, 0], [0, 0], [0, 0]];
pub const SHAPES: [[[i32;2]; 4]; 7] = [
    [[1, 1], [2, 1], [3, 1], [4, 1]],  // line shape
    [[1, 1], [2, 1], [2, 2], [3, 2]],  // z shape
    [[1, 2], [2, 2], [2, 1], [3, 1]],  // s shape
    [[1, 1], [1, 2], [2, 2], [2, 1]],  // square shape
    [[2, 1], [2, 2], [2, 3], [1, 3]],  // Mirror L shape
    [[1, 1], [1, 2], [1, 3], [2, 3]],  // L shape
    [[1, 2], [2, 2], [3, 2], [2, 1]],  // tri shape
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
