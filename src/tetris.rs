#![allow(non_camel_case_types)]

pub static FILL_WEIGHT: i32 = 1;
pub static WHITESPACE_WEIGHT: i32 = -1;
pub static BUMPINESS_WEIGHT: i32 = -1;

pub struct Field {
    pub matrix: Vec<Vec<u8>>
}

impl Field {
    pub fn simulate_application(&self, bounds: &Vec<Vec<u8>>, init_x: usize, init_y: usize) -> Field {
        let mut field = Field {
            matrix: self.matrix.clone()
        };
        let bound_y = bounds.len();
        let bound_x = bounds[0].len();
    
        for y in init_y..(init_y + bound_y) {
            for x in init_x..(init_x + bound_x) {
                field.matrix[y][x] = 1;
            }
        }
    
        field
    }
}

pub enum Tetrium {
    TETRIS_STAIR,
    TETRIS_STAIR_REVERSE,
    TETRIS_T,
    TETRIS_L,
    TETRIS_L_REVERSE,
    TETRIS_SQUARE,
    TETRIS_LINE
}

impl Tetrium {
    pub fn rotate_bounds(&self, times: u8) -> Vec<Vec<u8>> {
        match self {
            &Tetrium::TETRIS_SQUARE => vec![vec![1, 1], vec![1, 1]],
            &Tetrium::TETRIS_L => match times {
                0 => vec![vec![1, 0], vec![1, 0], vec![1, 1]],
                1 => vec![vec![1, 1, 1], vec![1, 0, 0]],
                2 => vec![vec![1, 1], vec![0, 1], vec![0, 1]],
                3 => vec![vec![0, 0, 1], vec![1, 1, 1]],
                _ => self.rotate_bounds(times % 4)
            },
            &Tetrium::TETRIS_L_REVERSE => match times {
                0 => vec![vec![0, 1], vec![0, 1], vec![1, 1]],
                1 => vec![vec![1, 0, 0], vec![1, 1, 1]],
                2 => vec![vec![1, 1], vec![1, 0], vec![1, 0]],
                3 => vec![vec![1, 1, 1], vec![0, 0, 1]],
                _ => self.rotate_bounds(times % 4)
            },
            &Tetrium::TETRIS_STAIR => match times {
                0 => vec![vec![0, 1, 1], vec![1, 1, 0]],
                1 => vec![vec![1, 0], vec![1, 1], vec![0, 1]],
                _ => self.rotate_bounds(times % 2)
            },
            &Tetrium::TETRIS_STAIR_REVERSE => match times {
                0 => vec![vec![1, 1, 0], vec![0, 1, 1]],
                1 => vec![vec![0, 1], vec![1, 1], vec![1, 0]],
                _ => self.rotate_bounds(times % 2)
            },
            &Tetrium::TETRIS_LINE => match times {
                0 => vec![vec![1, 1, 1, 1]],
                1 => vec![vec![1], vec![1], vec![1], vec![1]],
                _ => self.rotate_bounds(times % 2)
            },
            &Tetrium::TETRIS_T => match times {
                0 => vec![vec![0, 1, 0], vec![1, 1, 1]],
                1 => vec![vec![1, 0], vec![1, 1], vec![1, 0]],
                2 => vec![vec![1, 1, 1], vec![0, 1, 0]],
                3 => vec![vec![0, 1], vec![1, 1], vec![0, 1]],
                _ => self.rotate_bounds(times % 4)
            }
        }
    }
}

// bound[y][x]

pub fn find_best(field: &Field, tetrium: &Tetrium) -> Result<(usize, u8), &'static str> {
    let mut max_score = Option::None;
    let mut best = Option::None;
    for rotation in 0..4 {
        let bounds = tetrium.rotate_bounds(rotation);
        for x in 0..(11 - bounds[0].len()) {
            let score = calculate_score(field, x, &bounds);
            if max_score.is_none() || max_score.unwrap() < score {
                max_score = Option::Some(score);
                best = Option::Some((x, rotation))
            }
        }
    }
    if !best.is_none() {
        Ok(best.unwrap())
    } else {
        Err("Max Score is Null")
    }
}

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

pub fn collide(field: &Field, bounds: &Vec<Vec<u8>>, init_x: usize, amount: usize) -> bool {
    let bound_y = bounds.len();
    let bound_x = bounds[0].len();
    for x in 0..bound_x {
        for y in 0..bound_y {
            if field.matrix[y + amount][init_x + x] == 1 {
                return true;
            }
        }
    }
    return false;
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