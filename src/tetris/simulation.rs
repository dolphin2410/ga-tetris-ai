use crate::ga::model::KeyData;

use super::{field::Field, tetrium::Tetrium};

pub struct MovementData {
    pub x: usize,
    pub rotation: u8,
    pub sim_res: SimulationResults,
    pub max_score: f32
}

pub fn find_best_move(field: &Field, tetrium: &Tetrium, key: &KeyData) -> Option<MovementData> {
    let mut max_score = -1000.0;
    let mut best = Option::None;

    for rotation in 0..4 {
        let tetrium_bounds = tetrium.rotate_bounds(rotation);

        for x in 0..(field.matrix[0].len() - tetrium_bounds[0].len() + 1) {
            if let Ok(sim_res) = calculate_score(field, x, &tetrium_bounds, key) {

                if  max_score < sim_res.score {
                    max_score = sim_res.score;

                    best = Option::Some(MovementData { x, rotation, sim_res, max_score });
                }
            }
        }
    }

    best
}

pub struct SimulationResults {
    pub score: f32,

    pub num_whitespace: i32,
    pub bumpiness: f32,
    pub cleared_lines: i32,
    pub num_rows_with_holes: i32,
    pub cleared_score: f32,

    pub simulated_field: Field
}

pub fn calculate_score(field: &Field, x: usize, bounds: &Vec<Vec<u8>>, key: &KeyData) -> Result<SimulationResults, &'static str> {
    if let Ok(simulated_field) = field.simulate_application(bounds, x) {
        let len = simulated_field.matrix.len();
        let highest = simulated_field.highest();

        let num_whitespace = calculate_whitespace(&simulated_field);
        let bumpiness = calculate_bumpiness_score_w_key(&simulated_field, key);
        let cleared_lines = calculate_cleared_lines(&simulated_field);
        let num_rows_with_holes = calculate_rows_with_holes(&simulated_field);
        let cleared_score = calculate_cleared_score(cleared_lines, highest, len as i32, key);

        let score = cleared_score - (num_whitespace as f32 * key.whitespace_weight) - bumpiness - (num_rows_with_holes as f32 * key.hole_rows_weight);

        let sim_res = SimulationResults {
            score,
            num_whitespace,
            bumpiness,
            cleared_lines,
            num_rows_with_holes,
            cleared_score,
            simulated_field
        };

        Ok(sim_res)
    } else {
        Err("Simulation Failed")
    }
}

pub fn calculate_cleared_score(cleared: i32, highest: i32, height: i32, key: &KeyData) -> f32 {
    let min = key.completed_lines_min;
    let max = key.completed_lines_max;
    let weight = min + ((highest as f32 / height as f32) * (max - min));
    return cleared as f32 * weight;
}

pub fn calculate_cleared_lines(simulated: &Field) -> i32 {
    let mut filled = 0;
    for y in simulated.matrix.iter() {
        if !y.contains(&0) {
            filled += 1;
        }
    }
    filled
}

pub fn calculate_whitespace(simulated: &Field) -> i32 {
    let mut whitespace = 0;
    for x in 0..simulated.matrix[0].len() {
        let mut has_block = false;
        for (y, row) in simulated.matrix.iter().enumerate() {
            if !row.contains(&0) {
                continue;
            }
            if simulated.matrix[y][x] == 1 {
                has_block = true; // Found the top block of the column
            } else {
                if has_block && simulated.matrix[y][x] == 0 {
                    // This block is not the top of the column, but is empty
                    whitespace += 1; // This block is a whitespace
                }
            }
        }
    }

    whitespace
}

// pub fn boonsan(data: Vec<i32>) -> f32 {
//     let mut total = 0.0;

//     for i in data.iter() {
//         total += *i as f32;
//     }

//     let average = total / data.len() as f32;

//     let mut sum = 0.0;

//     for i in data.iter() {
//         sum += (*i as f32 - average).powi(2);
//     }

//     sum / data.len() as f32
// }

pub fn calculate_bumpiness_score_w_key(simulated: &Field, key: &KeyData) -> f32 {
    let mut bumpiness = 0;
    let mut prev_height: i32 = -1;

    for x in 0..simulated.matrix[0].len() {
        let height: i32 = simulated.highest_at(x);

        if prev_height != -1 {
            bumpiness += (height - prev_height).abs();
        }

        prev_height = height;
    }

    bumpiness as f32 * key.bumpiness_weight
}

pub fn calculate_rows_with_holes(simulated: &Field) -> i32 {
    let mut rows = vec![];

    for x in 0..simulated.matrix[0].len() {
        let mut has_block = false;
        for (y, row) in simulated.matrix.iter().enumerate() {
            if !row.contains(&0) {
                continue;
            }
            if simulated.matrix[y][x] == 1 {
                has_block = true;
            }
            if has_block && simulated.matrix[y][x] == 0 && !rows.contains(&y) {
                rows.push(y);
            }
        }
    }
    return rows.len() as i32;
}