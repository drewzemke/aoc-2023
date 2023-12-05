use crate::parser::parse_game;
use common::puzzle::PuzzlePart;

pub struct Puzzle02b {}

impl PuzzlePart for Puzzle02b {
    fn description() -> &'static str {
        "Sum the 'powers' of minimal sets within each game."
    }

    fn solve(input: &str) -> String {
        let mut power_sum: u32 = 0;

        for line in input.lines() {
            let (_, game) = parse_game(line).unwrap();

            // puzzle 2: compute maxes to find the minimal number of cubes necessary to make each game possible
            let color_maxes = game.color_maxes();

            // puzzle 2: the "power" of a set of cubes is the product of the number of each cube
            let game_power = color_maxes.0 * color_maxes.1 * color_maxes.2;
            power_sum += game_power;
        }

        power_sum.to_string()
    }
}
