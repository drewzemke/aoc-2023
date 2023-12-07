use crate::{math_things::number_of_ways_to_win, parser::parse};
use common::puzzle::PuzzlePart;

pub struct Puzzle06a {}

impl PuzzlePart for Puzzle06a {
    fn description() -> &'static str {
        "Compute the product of numbers of ways to win a boat race."
    }

    fn solve(input: &str) -> String {
        parse(input)
            .into_iter()
            .map(|(time, distance)| number_of_ways_to_win(time, distance))
            .product::<u64>()
            .to_string()
    }
}
