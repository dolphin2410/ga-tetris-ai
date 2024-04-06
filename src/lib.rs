pub mod tetris;
pub mod console;
pub mod ga;

// use wasm_bindgen::prelude::*;

use crate::tetris::{field::Field, util::find_best, tetrium::Tetrium};

// #[wasm_bindgen]
// pub fn tetris(str_field: &str, tetris: &str) -> String {
//     let matrix = serde_json::from_str::<Vec<Vec<u8>>>(str_field).unwrap();
//     let tetrium = &Tetrium::from_str(tetris).unwrap();
//     let (json, field) = tetris_internal(matrix, tetrium);
//     field.debug_console();
//     return json;
// }

// pub fn tetris_internal(matrix: Vec<Vec<u8>>, tetris: &Tetrium) -> (String, Field) {
//     let field = Field { matrix };

//     let (x, rotation, sim, score, _, _) = find_best(&field, tetris).unwrap();

//     let bounds = serde_json::to_string(&tetris.rotate_bounds(rotation)).unwrap();

//     (format!(r#"{{"x": {}, "bounds": {}}}"#, x, bounds), sim)
// }

#[test]
fn test() {
    let field = Field { matrix: vec![vec![0; 10]; 20] };
    // field.simulate_application(&Tetrium::TETRIS_L.rotate_bounds(1), 0).unwrap().debug();
    // field.simulate_application(&Tetrium::TETRIS_L.rotate_bounds(2), 0).unwrap().debug();
    // field.simulate_application(&Tetrium::TETRIS_L.rotate_bounds(3), 0).unwrap().debug();
    // let new = field.simulate_application(&Tetrium::TETRIS_L.rotate_bounds(0), 0).unwrap();
    // new.simulate_application(&Tetrium::TETRIS_L.rotate_bounds(2), 0).unwrap().debug();
    // let (x, rot, mut sim, _, hole, hole_row) = find_best(&field, &Tetrium::TETRIS_L).unwrap();
    // sim.clear_line(&vec![0; 10]);
    // sim.debug();
    // println!("{}--------------------{}", x, rot);
    // let (x, rot, mut sim, _, hole, hole_row) = find_best(&sim, &Tetrium::TETRIS_T).unwrap();
    // sim.clear_line(&vec![0; 10]);
    // sim.debug();
    // println!("{}--------------------{}", x, rot);
    // let (x, rot, mut sim, _, hole, hole_row) = find_best(&sim, &Tetrium::TETRIS_STAIR).unwrap();
    // sim.clear_line(&vec![0; 10]);
    // sim.debug();
    // println!("{}--------------------{}", x, rot);
    // let (x, rot, mut sim, _, hole, hole_row) = find_best(&sim, &Tetrium::TETRIS_L).unwrap();
    // sim.clear_line(&vec![0; 10]);
    // sim.debug();
    // println!("{}--------------------{}", x, rot);
}