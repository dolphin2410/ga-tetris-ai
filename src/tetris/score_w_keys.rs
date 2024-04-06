use crate::ga::model::KeyData;

use super::field::Field;

pub fn calculate_cleared_score_w_key(cleared: i32, highest: i32, height: i32, key: &KeyData) -> f32 {
    let min = key.completed_lines_min;
    let max = key.completed_lines_max;
    let weight = min + ((highest as f32 / height as f32) * (max - min));
    return cleared as f32 * weight;
}

pub fn calculate_score_w_key(field: &Field, x: usize, bounds: &Vec<Vec<u8>>, key: &KeyData) -> Result<(Field, f32, i32, i32), &'static str> {
    if let Ok(simulated) = field.simulate_application(bounds, x) {
        let whitespace = calculate_whitespace(&simulated);
        let bumpiness = calculate_bumpiness_score_w_key(&simulated, key);
        let cleared = calculate_cleared_lines(&simulated);
        let hole_rows = rows_with_holes(&simulated);


        let len = simulated.matrix.len();
        let highest = simulated.highest();

        Ok((simulated, calculate_cleared_score_w_key(cleared, highest, len as i32, key) - (whitespace as f32 * key.whitespace_weight) - bumpiness - (hole_rows as f32 * key.hole_rows_weight), whitespace, hole_rows))
    } else {
        Err("Simon says, 'sugo'")
    }
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

pub fn rows_with_holes(simulated: &Field) -> i32 {
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
