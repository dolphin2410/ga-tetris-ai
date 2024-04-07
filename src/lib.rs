pub mod tetris;
pub mod console;
pub mod ga;

use wasm_bindgen::prelude::*;

use ga::model::KeyData;
use tetris::simulation::{MovementData, SimulationResults};

use crate::tetris::{field::Field, simulation::find_best_move, tetrium::Tetrium};

pub const KEY: KeyData = KeyData { whitespace_weight: 0.7910332, bumpiness_weight: 0.39972275, completed_lines_min: 0.05920756, completed_lines_max: 0.78152, hole_rows_weight: 0.9194734 };

#[wasm_bindgen]
pub fn tetris(str_field: &str, tetris: &str) -> String {
    let matrix = serde_json::from_str::<Vec<Vec<u8>>>(str_field).unwrap();
    let tetrium = &Tetrium::from_str(tetris).unwrap();
    let (json, field) = tetris_internal(matrix, tetrium);
    field.debug_console();
    return json;
}

pub fn tetris_internal(matrix: Vec<Vec<u8>>, tetris: &Tetrium) -> (String, Field) {
    let field = Field { matrix };

    let MovementData { x, rotation, sim_res: SimulationResults { simulated_field, .. }, .. } = find_best_move(&field, tetris, &KEY).unwrap();

    let tetrium_bounds = serde_json::to_string(&tetris.rotate_bounds(rotation)).unwrap();

    return (format!(r#"{{"x": {}, "bounds": {}}}"#, x, tetrium_bounds), simulated_field);
}

#[cfg(test)]
mod tests {
    use crate::{ga::model::{train, visual_simulation, KeyData}, KEY};

    #[test]
    fn simulate_tetris_game() {
        visual_simulation(&KEY);
    }

    #[test]
    fn train_ga_agent() {
        train(vec![
            KeyData { whitespace_weight: 0.8208, bumpiness_weight: 0.3924, completed_lines_min: 0.5500, completed_lines_max: 0.7806, hole_rows_weight: 0.8810 },
            KeyData { whitespace_weight: 0.12526667, bumpiness_weight: 0.18833774, completed_lines_min: 0.13196576, completed_lines_max: 0.90341705, hole_rows_weight: 0.83196425 },
            KeyData { whitespace_weight: 0.9675431, bumpiness_weight: 0.3924, completed_lines_min: 0.41572422, completed_lines_max: 0.7806, hole_rows_weight: 0.881 },
            KeyData { whitespace_weight: 0.66450435, bumpiness_weight: 0.2782724, completed_lines_min: 0.12451875, completed_lines_max: 0.5316432, hole_rows_weight: 0.881 },
        ]);
        
        // simulation(&KeyData { whitespace_weight: 0.66450435, bumpiness_weight: 0.2782724, completed_lines_min: 0.12451875, completed_lines_max: 0.5316432, hole_rows_weight: 0.881 });
        // simulation(&KeyData { whitespace_weight: 0.809155, bumpiness_weight: 0.24494582, completed_lines_min: 0.5324551, completed_lines_max: 0.34481657, hole_rows_weight: 0.4931103 });
        // simulation(&KeyData { whitespace_weight: 0.9675431, bumpiness_weight: 0.3924, completed_lines_min: 0.41572422, completed_lines_max: 0.7806, hole_rows_weight: 0.881 });
        // simulation(&KeyData { whitespace_weight: 0.12526667, bumpiness_weight: 0.18833774, completed_lines_min: 0.13196576, completed_lines_max: 0.90341705, hole_rows_weight: 0.83196425 });
        // simulation(&KeyData { whitespace_weight: 0.7506602, bumpiness_weight: 0.41453475, completed_lines_min: 0.114563525, completed_lines_max: 0.27861118, hole_rows_weight: 0.697925 });
        // simulation(&KeyData { whitespace_weight: 0.8208, bumpiness_weight: 0.3924, completed_lines_min: 0.5500, completed_lines_max: 0.7806, hole_rows_weight: 0.8810 });
        // simulation(&KeyData { whitespace_weight: 0.12526667, bumpiness_weight: 0.18833774, completed_lines_min: 0.13196576, completed_lines_max: 0.90341705, hole_rows_weight: 0.83196425 });
    }
}