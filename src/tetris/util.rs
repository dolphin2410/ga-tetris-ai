use super::{score::*, field::Field, tetrium::Tetrium};

pub fn find_best(field: &mut Field, tetrium: &Tetrium) -> Result<(usize, u8), &'static str> {
    let mut max_score = Option::None;
    let mut best = Option::None;
    for rotation in 0..4 {
        let bounds = tetrium.rotate_bounds(rotation);
        for x in 0..(11 - bounds[0].len()) {
            let score = calculate_score(field, x, &bounds);
            if max_score.is_none() || max_score.unwrap() < score {
                max_score = Option::Some(score);
                best = Option::Some((x, rotation));
            }
        }
    }
    if !best.is_none() {
        Ok(best.unwrap())
    } else {
        Err("Max Score is Null")
    }
}

pub fn collide(field: &Field, bounds: &Vec<Vec<u8>>, init_x: usize, amount: usize) -> bool {
    let bound_y = bounds.len();
    let bound_x = bounds[0].len();
    for x in 0..bound_x {
        for y in 0..bound_y {
            if y + amount >= 20 {
                return true;
            }

            if field.matrix[y + amount][init_x + x] != 0 {
                return true;
            }
        }
    }
    return false;
}