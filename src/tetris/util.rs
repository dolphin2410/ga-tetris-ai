use crate::ga::model::KeyData;

use super::{field::Field, score::*, score_w_keys::calculate_score_w_key, tetrium::Tetrium};

pub fn find_best(field: &Field, tetrium: &Tetrium) -> Option<(usize, u8, Field, f32, i32, i32)> {
    let mut max_score = Option::None;
    let mut best = Option::None;
    for rotation in 0..4 {
        let bounds = tetrium.rotate_bounds(rotation);
        for x in 0..(field.matrix[0].len() - bounds[0].len() + 1) {
            if let Ok((simulated, score, hole, hole_row)) = calculate_score(field, x, &bounds) {
                if  max_score.is_none() || max_score.unwrap() < score {
                    max_score = Some(score);
                    best = Option::Some((x, rotation, simulated, max_score.unwrap(), hole, hole_row));
                }
            }
        }
    }
    best
}

pub fn find_best_w_key(field: &Field, tetrium: &Tetrium, key: &KeyData) -> Option<(usize, u8, Field, f32, i32, i32)> {
    let mut max_score = Option::None;
    let mut best = Option::None;
    for rotation in 0..4 {
        let bounds = tetrium.rotate_bounds(rotation);
        for x in 0..(field.matrix[0].len() - bounds[0].len() + 1) {
            if let Ok((simulated, score, hole, hole_row)) = calculate_score_w_key(field, x, &bounds, key) {
                if  max_score.is_none() || max_score.unwrap() < score {
                    max_score = Some(score);
                    
                    best = Option::Some((x, rotation, simulated, max_score.unwrap(), hole, hole_row));
                }
            }
        }
    }
    best
}