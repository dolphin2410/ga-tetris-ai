#[allow(non_camel_case_types)]

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