use crate::console_log;

use super::{tetrium::Tetrium, util::highest_of};

pub struct Field {
    pub matrix: Vec<Vec<u8>>
}

impl Field {
    pub fn simulate_application(&self, bounds: &Vec<Vec<u8>>, init_x: usize) -> Result<Field, &'static str> {
        let bound_y = bounds.len();
        let bound_x = bounds[0].len();

        let mut field = Field {
            matrix: self.matrix.clone()
        };

        if let Ok(init_y) = get_y_level(&field, bounds, init_x) {
            for y in 0..bound_y {
                for x in 0..bound_x {
                    if bounds[y][x] != 0 {
                        field.matrix[init_y + y][x + init_x] = bounds[y][x];
                    }
                }
            }
            return Ok(field);
        }
        Err("Nope")
    }

    pub fn apply(&mut self, tetris: &Tetrium, rotation: u8, x: usize) {
        let bounds = tetris.rotate_bounds(rotation);

        let bound_y = bounds.len();
        let bound_x = bounds[0].len();
    
        let init_y = highest_of((0..bounds[0].len())
            .map(|bound_x| {
                self.highest_at(x + bound_x - 1)
            })
            .collect()) + bound_y;

        let height = self.matrix.len();

        for y in 0..bound_y {
            for x in 0..bound_x {
                self.matrix[height - init_y + y][x + x] = bounds[y][x];
            }
        }
    }

    pub fn debug(&self) {
        println!("--start--");
        for row in self.matrix.iter() {
            for column in row {
                print!("{}", column);
            }
            println!("")
        }
        println!("---end---");
    }

    pub fn debug_console(&self) {
        console_log!("--start--");
        let mut string = String::new();
        for row in self.matrix.iter() {
            for column in row {
                string.push_str(&column.to_string())
            }
            string.push('\n');
        }
        console_log!("{}", string);
        console_log!("---end---");
    }

    pub fn clone(&self) -> Field {
        return Field { matrix: self.matrix.clone() }
    }

    pub fn highest_at(&self, x: usize) -> i32 {
        let mut height = 0;
        for y in 0..self.matrix.len() {
            let full = !self.matrix[y].contains(&0);
            if self.matrix[y][x] != 0 && height == 0 && !full {
                height = self.matrix.len() - y;
            }
    
            if height > 0 && full {
                height -= 1;
            }
        }
    
        return height as i32;
    }
    
    pub fn highest(&self) -> i32 {
        for (y, row) in self.matrix.iter().enumerate() {
            if row.contains(&0) {
                return (self.matrix.len() - y) as i32;
            }
        }
    
        return 0;
    }
}

pub fn get_y_level(field: &Field, bounds: &Vec<Vec<u8>>, init_x: usize) -> Result<usize, &'static str> {
    let mut init_y = 0;
    while !collided(field, bounds, init_x, init_y) {
        init_y += 1;
    }
    return Ok(init_y - 1);
}

pub fn collided(field: &Field, bounds: &Vec<Vec<u8>>, init_x: usize, init_y: usize) -> bool {
    for y in 0..bounds.len() {
        for x in 0..bounds[0].len() {
            if y + init_y >= 20 {
                return true;
            }
            if field.matrix[y + init_y][x + init_x] != 0 && bounds[y][x] != 0 {
                return true;
            }
        }
    }
    return false;
}