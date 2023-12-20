use crate::Hash;
use common::puzzle::PuzzlePart;

pub struct Puzzle15a {}

impl PuzzlePart for Puzzle15a {
    fn description() -> &'static str {
        "Find the sum of hash values of a bunch of strings."
    }

    fn solve(input: &str) -> String {
        input
            .trim()
            .split(',')
            .map(Hash::hash)
            .sum::<u32>()
            .to_string()
    }
}
