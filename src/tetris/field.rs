use super::{tetrium::Tetrium, util::collide};

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

    pub fn apply(&mut self, tetris: &Tetrium, rotation: u8, x: usize) {
        let bounds = tetris.rotate_bounds(rotation);

        let bound_y = bounds.len();
        let bound_x = bounds[0].len();
        let mut max_y = Option::None;
    
        for y in 1..(21 - bound_y) {
            if collide(self, &bounds, x, y) {
                break;
            } else {
                max_y = Option::Some(y);
            }
        }

        if !max_y.is_none() {
            for bx in 0..bound_x {
                for by in 0..bound_y {
                    self.matrix[by + max_y.unwrap()][bx + x] = bounds[by][bx];
                }
            }
        }

        self.clear_line();
    }

    pub fn clear_line(&mut self) -> usize {
        let mut cleared = 0;
        self.matrix.retain(|e| {
            let keep = e.contains(&0);
            if !keep {
                cleared += 1
            }
            keep
        });
        
        if cleared != 0 {
            let mut vec = vec![];
            for _ in 0..cleared {
                vec.push(vec![0; 10]);
            }

            vec.extend(self.matrix.clone());

            self.matrix = vec;

            // let _ = std::mem::replace(&mut self.matrix, vec);
        }

        cleared
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

    pub fn clone(&self) -> Field {
        return Field { matrix: self.matrix.clone() }
    }
}