use super::{util::collide, field::Field};

pub static FILL_WEIGHT: f32 = 0.8;
pub static WHITESPACE_WEIGHT: f32 = -0.8;
pub static BUMPINESS_WEIGHT: f32 = -0.8;
pub static CLEARED_WEIGHT: f32 = 0.55;

pub fn calculate_score(field: &mut Field, x: usize, bounds: &Vec<Vec<u8>>) -> f32 {
    let bound_y = bounds.len();
    let mut max_y = Option::None;

    for y in 1..(21 - bound_y) {
        if collide(field, bounds, x, y) {
            break;
        } else {
            max_y = Option::Some(y);
        }
    }

    if max_y.is_none() {
        return 0.0;
    } else if max_y.unwrap() <= 0 {
        return 0.0;
    }

    let whitespace = calculate_whitespace(field, bounds, x, max_y.unwrap());
    let bottom_fill = calculate_bottom_fill(bounds, max_y.unwrap());
    let bumpiness = calculate_bumpiness(field, bounds, x, max_y.unwrap());
    let cleared = field.clear_line() as f32 * CLEARED_WEIGHT;

    return bottom_fill + whitespace + bumpiness + cleared;
}

pub fn calculate_whitespace(field: &Field, bounds: &Vec<Vec<u8>>, init_x: usize, init_y: usize) -> f32 {
    let bound_x = bounds[0].len();

    let mut whitespace = 0;

    for y in init_y..20 {
        for x in init_x..(init_x + bound_x) {
            if field.matrix[y][x] == 0 {
                whitespace = whitespace + 1
            }
        }
    }

    whitespace as f32 * WHITESPACE_WEIGHT
}

pub fn calculate_bottom_fill(bounds: &Vec<Vec<u8>>, init_y: usize) -> f32 {
    let bound_y = bounds.len();
    let bound_x = bounds[0].len();

    let mut score = 0.0;

    for y in 0..bound_y {
        for x in 0..bound_x {
            if bounds[y][x] != 0 {
                score += ((y + init_y) as f32) * FILL_WEIGHT
            }
        }
    }
    score
}

pub fn calculate_bumpiness(field: &Field, bounds: &Vec<Vec<u8>>, init_x: usize, init_y: usize) -> f32 {
    let application = field.simulate_application(bounds, init_x, init_y);
    let mut vec = vec![];
    for i in 0..application.matrix.len() {
        if application.matrix[i].contains(&1) {
            vec.push(i as i32);
        }
    }
    (vec.last().unwrap() - vec.first().unwrap()) as f32 * BUMPINESS_WEIGHT
}