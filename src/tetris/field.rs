// use crate::console_log;

use super::tetrium::Tetrium;

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
                    if bounds[y][x] == 1 {
                        field.matrix[init_y + y][x + init_x] = bounds[y][x];
                    }
                }
            }
            return Ok(field);
        }
        Err("Nope")
    }

    pub fn clear_line(&mut self, empty_line: &Vec<u8>) -> u32 {
        // index of zeros or ones list ex) [0, 0, 0, ..., 0]
        let mut to_remove_total = vec![];

        // index of ones list ex) [1, 1, 1, ..., 1]
        let mut to_remove_ones = vec![];
        
        let mut gasanjum = 0;

        let mut index = 0;

        for line in self.matrix.iter_mut() {

            if !line.iter().any(|x| x.to_owned() == 0) { // a line that consists of only ones
                to_remove_ones.push(index);
                to_remove_total.push(index);
            }

            if !line.iter().any(|x| x.to_owned() == 1) { // a line that consists of only zeros
                to_remove_total.push(index);
            }

            if index != 0 && to_remove_ones.contains(&(index - 1)) {
                gasanjum += 1;    
            }

            index += 1;
        }

        let num_remove: u32 = to_remove_total.len().try_into().unwrap(); // number to refill
        let num_remove_ones: u32 = to_remove_ones.len().try_into().unwrap(); // number to count the scores

        for i in to_remove_total.into_iter().rev() { // reverse the index of to_remove_total and remove from the bottom
            self.matrix.remove(i);
        }

        for _ in 0..num_remove { // refill
            self.matrix.insert(0, empty_line.to_vec());
        }

        return num_remove_ones + gasanjum;
    }

    // wtf is this code for?
    pub fn apply(&mut self, tetris: &Tetrium, rotation: u8, x: usize) {
        let bounds = tetris.rotate_bounds(rotation);

        let bound_y_size = bounds.len();
        let bound_x_size = bounds[0].len();
        let map_height = self.matrix.len();

        let map: Vec<i32> = (0..bound_x_size).map(|bound_x| {
            self.highest_at(x + bound_x - 1)
        }).collect();
    
        let init_y = map.iter().max().unwrap().to_owned() as usize + bound_y_size;


        for y in 0..bound_y_size {
            for x in 0..bound_x_size {
                self.matrix[map_height - init_y + y][x + x] = bounds[y][x];
            }
        }
    }

    pub fn debug(&self) {
        println!("--start---");
        for row in self.matrix.iter() {
            for column in row {
                let to_print = if *column == 1 {
                    "\u{25A0}"
                } else {
                    " "
                };
                print!("{}", to_print);
            }
            println!("")
        }
        println!("---end----");
    }

    pub fn debug_console(&self) {
        // console_log!("--start--");
        let mut string = String::new();
        for row in self.matrix.iter() {
            for column in row {
                string.push_str(&column.to_string())
            }
            string.push('\n');
        }
        // console_log!("{}", string);
        // console_log!("---end---");
    }

    pub fn clone(&self) -> Field {
        return Field { matrix: self.matrix.clone() }
    }

    pub fn highest_at(&self, x: usize) -> i32 {
        let mut height = 0;

        for y in 0..self.matrix.len() {
            // let is_full_line = !self.matrix[y].contains(&0);

            // if self.matrix[y][x] != 0 && height == 0 && !is_full_line {
            if self.matrix[y][x] != 0 && height == 0 {
                height = self.matrix.len() - y; // height is minimum 1 to maximum
            }
    
            // if height > 0 && is_full_line {
            //     height -= 1;
            // }
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

    if init_y == 0 {
        return Err("invalid spot")
    }

    return Ok(init_y - 1);
}

pub fn collided(field: &Field, bounds: &Vec<Vec<u8>>, init_x: usize, init_y: usize) -> bool {
    for y in 0..bounds.len() {
        for x in 0..bounds[0].len() {
            // todo! remove hardcoding
            if y + init_y >= 20 { // out of map 
                return true;
            }

            // collides with prior block placed
            if field.matrix[init_y + y][init_x + x] == 1 && bounds[y][x] == 1 {
                return true;
            }
        }
    }

    return false;
}