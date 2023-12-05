use crate::parser::parse_game;
use common::puzzle::PuzzlePart;

// constraints
const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

pub struct Puzzle02a {}

impl PuzzlePart for Puzzle02a {
    fn description() -> &'static str {
        "Sum the ids of possible games."
    }

    fn solve(input: &str) -> String {
        let mut id_sum: u32 = 0;

        for line in input.lines() {
            let (_, game) = parse_game(line).unwrap();

            // puzzle 1: compute maxes to find which games were possible given the constraints
            let color_maxes = game.color_maxes();

            if color_maxes.0 <= MAX_RED && color_maxes.1 <= MAX_GREEN && color_maxes.2 <= MAX_BLUE {
                id_sum += game.id;
            }
        }

        id_sum.to_string()
    }
}
