use aoc_2023_02::parser::parse_game;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// puzzle 1: constraints
const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut id_sum: u32 = 0;
    let mut power_sum: u32 = 0;

    for line in reader.lines().map(Result::unwrap) {
        let (_, game) = parse_game(&line).unwrap();

        // puzzle 1: compute maxes to find which games were possible given the constraints
        // puzzle 2: compute maxes to find the minimal number of cubes necessary to make each game possible
        let color_maxes = game.color_maxes();

        if color_maxes.0 <= MAX_RED && color_maxes.1 <= MAX_GREEN && color_maxes.2 <= MAX_BLUE {
            id_sum += game.id;
        }

        // puzzle 2: the "power" of a set of cubes is the product of the number of each cube
        let game_power = color_maxes.0 * color_maxes.1 * color_maxes.2;
        power_sum += game_power;
    }

    println!("puzzle 1 (sum of ids of possible games): {id_sum}");
    println!("puzzle 2 (sum of powers of minimal sets): {power_sum}");
}
