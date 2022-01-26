pub mod tetris;

use wasm_bindgen::prelude::*;

use crate::tetris::{field::Field, util::find_best, tetrium::Tetrium};

#[wasm_bindgen]
pub fn tetris() {
    let mut field = Field {
        matrix: vec![vec![0; 10]; 20]
    };

    field.matrix[19][0] = 1;
    field.matrix[19][1] = 1;
    field.matrix[19][2] = 1;
    field.matrix[19][3] = 1;

    let (x, rotation) = find_best(&field, &Tetrium::TETRIS_L_REVERSE).unwrap();

    println!("X: {}, Rotation: {}", x, rotation);
}