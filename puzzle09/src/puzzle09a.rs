use crate::parser::parse_line;
use common::puzzle::PuzzlePart;

pub struct Puzzle09a {}

impl PuzzlePart for Puzzle09a {
    fn description() -> &'static str {
        "Extrapolate the next values of a bunch of discrete functions, then add the results."
    }

    fn solve(input: &str) -> String {
        input
            .lines()
            .map(parse_line)
            .map(|func| func.extrapolate())
            .sum::<i32>()
            .to_string()
    }
}
