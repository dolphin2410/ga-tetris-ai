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