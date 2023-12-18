use common::puzzle::PuzzlePart;

use crate::parser::parse_line;

pub struct Puzzle09b {}

impl PuzzlePart for Puzzle09b {
    fn description() -> &'static str {
        "Extrapolate the _previous_ values of a bunch of discrete functions, then add the results."
    }

    fn solve(input: &str) -> String {
        input
            .lines()
            .map(parse_line)
            .map(|func| func.extrapolate_backwards())
            .sum::<i32>()
            .to_string()
    }
}
