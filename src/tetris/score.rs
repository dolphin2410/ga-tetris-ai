use super::{util::collide, field::Field};

pub static FILL_WEIGHT: i32 = 1;
pub static WHITESPACE_WEIGHT: i32 = -1;
pub static BUMPINESS_WEIGHT: i32 = -1;

pub fn calculate_score(field: &Field, x: usize, bounds: &Vec<Vec<u8>>) -> i32 {
    let bound_y = bounds.len();
    let mut max_y = Option::None;

    for y in 1..(21 - bound_y) {
        if collide(field, bounds, x, y) {
            break;
        } else {
            max_y = Option::Some(y - 1);
        }
    }

    if max_y.is_none() {
        return 0;
    } else if max_y.unwrap() <= 0 { // TODO <= or < ???
        return 0;
    }

    let whitespace = calculate_whitespace(field, bounds, x, max_y.unwrap());
    let bottom_fill = calculate_bottom_fill(bounds, max_y.unwrap());
    let bumpiness = calculate_bumpiness(field, bounds, x, max_y.unwrap());

    return bottom_fill + whitespace + bumpiness;
}

pub fn calculate_whitespace(field: &Field, bounds: &Vec<Vec<u8>>, init_x: usize, init_y: usize) -> i32 {
    let bound_x = bounds[0].len();

    let mut whitespace = 0;

    for y in init_y..20 {
        for x in init_x..(init_x + bound_x) {
            if field.matrix[y][x] == 0 {
                whitespace = whitespace + 1
            }
        }
    }

    whitespace * WHITESPACE_WEIGHT
}

pub fn calculate_bottom_fill(bounds: &Vec<Vec<u8>>, init_y: usize) -> i32 {
    let bound_y = bounds.len();
    let bound_x = bounds[0].len();

    let mut score = 0;

    for y in 0..bound_y {
        for x in 0..bound_x {
            if bounds[y][x] != 0 {
                score += ((y + init_y) as i32) * FILL_WEIGHT
            }
        }
    }
    score
}

pub fn calculate_bumpiness(field: &Field, bounds: &Vec<Vec<u8>>, init_x: usize, init_y: usize) -> i32 {
    let application = field.simulate_application(bounds, init_x, init_y);
    let mut vec = vec![];
    for i in 0..application.matrix.len() {
        if application.matrix[i].contains(&1) {
            vec.push(i as i32);
        }
    }
    (vec.last().unwrap() - vec.first().unwrap()) * BUMPINESS_WEIGHT
}