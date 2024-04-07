use std::{thread, time::{Duration, SystemTime, UNIX_EPOCH}};

use rand::Rng;

use crate::tetris::{field::Field, simulation::{find_best_move, MovementData}, tetrium::Tetrium};
use rand::seq::SliceRandom;

#[derive(Clone, Copy, Debug)]
pub struct KeyData {
    pub whitespace_weight: f32,
    pub bumpiness_weight: f32,
    pub completed_lines_min: f32,
    pub completed_lines_max: f32,
    pub hole_rows_weight: f32
}

// 
pub static POPULATION: u32 = 200;
pub static GIFTED_POPULATION: usize = 50;

// 돌연변이 빈도
pub static MUT_RATE: f32 = 0.1;

pub fn visual_simulation(keys: &KeyData) {
    let mut rng = rand::thread_rng();

    let mut field = Field { matrix: vec![vec![0; 10]; 20] };

    loop {
        let tetris: Tetrium = rng.gen();

        if let Some(MovementData { sim_res, .. }) = find_best_move(&field, &tetris, keys) {
            field = sim_res.simulated_field;
            field.clear_line(&vec![0; 10]);
            field.debug();
            thread::sleep(Duration::from_millis(300));
        } else {
            break;
        }
    }
}

pub fn fitness_single(keys: &KeyData) -> u32 {
    let mut rng = rand::thread_rng();

    let mut field = Field { matrix: vec![vec![0; 10]; 20] };

    let mut score = 0;

    loop {
        let tetris: Tetrium = rng.gen();

        if let Some(MovementData { sim_res, .. }) = find_best_move(&field, &tetris, keys) {

            field = sim_res.simulated_field;
            score += field.clear_line(&vec![0; 10]);
        } else {
            break;
        }
    }

    return score;
}

pub fn fitness(keys: &KeyData) -> u32 {
    let i = fitness_single(keys);
    let ii = fitness_single(keys);
    let iii = fitness_single(keys);
    let iv = fitness_single(keys);
    let v = fitness_single(keys);

    return i + ii + iii + iv + v;
} 

// sort by fitness
pub fn sort(population: &mut Vec<(u32, KeyData)>) {
    population.sort_by(|a, b| a.0.cmp(&b.0));
    population.reverse();
}

// 돌연변이
pub fn mutation(key: &mut KeyData) {
    let mut rng = rand::thread_rng();

    if rng.gen::<f32>() < MUT_RATE {
        key.whitespace_weight = rng.gen::<f32>();
    }
    if rng.gen::<f32>() < MUT_RATE {
        key.bumpiness_weight = rng.gen::<f32>();
    }
    if rng.gen::<f32>() < MUT_RATE {
        key.completed_lines_max = rng.gen::<f32>();
    }
    if rng.gen::<f32>() < MUT_RATE {
        key.completed_lines_min = rng.gen::<f32>();
    }
    if rng.gen::<f32>() < MUT_RATE {
        key.hole_rows_weight = rng.gen::<f32>();
    }
}

// 교차를 실행
pub fn crossover_and_mutation(selected: &Vec<(u32, KeyData)>, population: &Vec<(u32, KeyData)>) -> Vec<(u32, KeyData)> {
    let mut rng = rand::thread_rng();

    let mut crossovered = vec![];

    // Crossover for 0 .. POPULATION
    for _ in 0..POPULATION {
        let rand_selected = selected.choose(&mut rand::thread_rng()).unwrap();
        let rand_population = population.choose(&mut rand::thread_rng()).unwrap();

        let new_whitespace = if rng.gen_range(0..2) == 0 {
            rand_population.1.whitespace_weight
        } else {
            rand_selected.1.whitespace_weight
        };

        let new_bumpiness = if rng.gen_range(0..2) == 0 {
            rand_population.1.bumpiness_weight
        } else {
            rand_selected.1.bumpiness_weight
        };

        let new_l_min = if rng.gen_range(0..2) == 0 {
            rand_population.1.completed_lines_min
        } else {
            rand_selected.1.completed_lines_min
        };

        let new_l_max = if rng.gen_range(0..2) == 0 {
            rand_population.1.completed_lines_max
        } else {
            rand_selected.1.completed_lines_max
        };

        let new_hole_rows = if rng.gen_range(0..2) == 0 {
            rand_population.1.hole_rows_weight
        } else {
            rand_selected.1.hole_rows_weight
        };
        
        let mut new = KeyData {
            whitespace_weight: new_whitespace,
            bumpiness_weight: new_bumpiness,
            completed_lines_min: new_l_min,
            completed_lines_max: new_l_max,
            hole_rows_weight: new_hole_rows,
        };

        mutation(&mut new);

        crossovered.push(new);
    }

    crossovered.iter().map(|x| {
        (fitness(x), x.clone())
    }).collect()
}

// Compare previous generation with the new generation
pub fn replace(new_gen: Vec<(u32, KeyData)>, old_gen: &mut Vec<(u32, KeyData)>) {
    let mut key = 0;
    for i in old_gen.iter_mut() {
        if i.0 < new_gen[key].0 {
            *i = new_gen[key]
        }

        key += 1;
    }
}

pub fn print_timestamp(header: &str) {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH).expect("Time went backwards");

    println!("{}: Timestamp: {:?}", header, since_the_epoch);
}

pub fn train(intial_population: Vec<KeyData>) {
    let mut rng = rand::thread_rng();
    let mut population = vec![];

    println!("Training Started");

    for key in intial_population.iter() {
        let new_key = key.clone();
        let fitness = fitness(&new_key);
        println!("Current Fitness: {}", fitness);
        population.push((fitness, new_key));
    }

    for _ in 0..(POPULATION as usize - intial_population.len()) {

        // Generate Initial Population
        let keys = KeyData {
            whitespace_weight: rng.gen::<f32>(),
            bumpiness_weight: rng.gen::<f32>(),
            completed_lines_max: rng.gen::<f32>(),
            completed_lines_min: rng.gen::<f32>(),
            hole_rows_weight: rng.gen::<f32>()
        };

        population.push((fitness(&keys), keys))
    }

    println!("Population Generated");

    for _ in 0..100 {
        print_timestamp("proc_enter");

        sort(&mut population);

        // Find the superior gene of current step
        println!("Current Superior: {:?}", population[0].1.clone());
        
        // The ones adapted to the environment
        let selected = population[0..GIFTED_POPULATION].to_vec();

        // crossover the chromosome
        let crossovered = crossover_and_mutation(&selected, &population);

        // compare the prior generation with the new generation
        replace(crossovered, &mut population);

        print_timestamp("proc_exit");
    }


    // Finalize the step
    sort(&mut population);

    println!("{:?}", population[0].1.clone());
}