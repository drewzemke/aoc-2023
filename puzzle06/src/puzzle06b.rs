use common::puzzle::PuzzlePart;

use crate::{math_things::number_of_ways_to_win, parser::parse_ignore_spaces};

pub struct Puzzle06b {}

impl PuzzlePart for Puzzle06b {
    fn description() -> &'static str {
        "Compute the number of ways to win just one big boat race."
    }

    fn solve(input: &str) -> String {
        let (time, distance) = parse_ignore_spaces(input);
        number_of_ways_to_win(time, distance).to_string()
    }
}
