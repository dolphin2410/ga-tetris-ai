use tetris_ai::ga::model::{simulation, train, KeyData};

fn main() {
    train(vec![
        KeyData { whitespace_weight: 0.8208, bumpiness_weight: 0.3924, completed_lines_min: 0.5500, completed_lines_max: 0.7806, hole_rows_weight: 0.8810 },
        KeyData { whitespace_weight: 0.12526667, bumpiness_weight: 0.18833774, completed_lines_min: 0.13196576, completed_lines_max: 0.90341705, hole_rows_weight: 0.83196425 },
        KeyData { whitespace_weight: 0.9675431, bumpiness_weight: 0.3924, completed_lines_min: 0.41572422, completed_lines_max: 0.7806, hole_rows_weight: 0.881 },
        KeyData { whitespace_weight: 0.66450435, bumpiness_weight: 0.2782724, completed_lines_min: 0.12451875, completed_lines_max: 0.5316432, hole_rows_weight: 0.881 },
    ]);

    // simulation(&KeyData { whitespace_weight: 0.66450435, bumpiness_weight: 0.2782724, completed_lines_min: 0.12451875, completed_lines_max: 0.5316432, hole_rows_weight: 0.881 });

    // simulation(&KeyData { whitespace_weight: 0.809155, bumpiness_weight: 0.24494582, completed_lines_min: 0.5324551, completed_lines_max: 0.34481657, hole_rows_weight: 0.4931103 });

    // simulation(&KeyData { whitespace_weight: 0.9675431, bumpiness_weight: 0.3924, completed_lines_min: 0.41572422, completed_lines_max: 0.7806, hole_rows_weight: 0.881 });

    // simulation(&KeyData { whitespace_weight: 0.12526667, bumpiness_weight: 0.18833774, completed_lines_min: 0.13196576, completed_lines_max: 0.90341705, hole_rows_weight: 0.83196425 });
    // simulation(&KeyData { whitespace_weight: 0.7506602, bumpiness_weight: 0.41453475, completed_lines_min: 0.114563525, completed_lines_max: 0.27861118, hole_rows_weight: 0.697925 });
    // simulation(&KeyData { whitespace_weight: 0.8208, bumpiness_weight: 0.3924, completed_lines_min: 0.5500, completed_lines_max: 0.7806, hole_rows_weight: 0.8810 });
    // simulation(&KeyData { whitespace_weight: 0.12526667, bumpiness_weight: 0.18833774, completed_lines_min: 0.13196576, completed_lines_max: 0.90341705, hole_rows_weight: 0.83196425 });
}