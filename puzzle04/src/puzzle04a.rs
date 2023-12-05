use crate::ScratchCard;
use common::puzzle::PuzzlePart;

pub struct Puzzle04a {}

impl PuzzlePart for Puzzle04a {
    fn description() -> &'static str {
        "Sum the scores of scratchy cards."
    }

    fn solve(input: &str) -> String {
        input
            .lines()
            .map(ScratchCard::parse_from_str)
            .map(|card| card.score())
            .sum::<u32>()
            .to_string()
    }
}
