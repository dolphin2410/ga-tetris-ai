pub mod tetris;

use tetris::order::Order;
use wasm_bindgen::prelude::*;

use crate::tetris::{field::Field, util::find_best, tetrium::Tetrium};

#[wasm_bindgen]
pub fn tetris(str_field: &str, tetris: &str) -> Order {
    let matrix = serde_json::from_str::<Vec<Vec<u8>>>(str_field).unwrap();

    let mut field = Field { matrix };

    let (x, rotation) = find_best(&mut field, &Tetrium::from_str(tetris).unwrap()).unwrap();

    Order {
        x, rotation
    }
}