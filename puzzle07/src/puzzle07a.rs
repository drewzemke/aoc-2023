use common::puzzle::PuzzlePart;

use crate::{parser::parse_line, Hand};

pub struct Puzzle07a {}

impl PuzzlePart for Puzzle07a {
    fn description() -> &'static str {
        "Sum the 'ranks' of a set of poker hands."
    }

    fn solve(input: &str) -> String {
        let mut hands: Vec<(Hand, u32)> = input.lines().map(parse_line).collect();
        hands.sort_by(|(hand1, _), (hand2, _)| hand2.cmp(hand1));

        hands
            .iter()
            .enumerate()
            .map(|(rank, (_, num))| (rank as u32 + 1) * *num)
            .sum::<u32>()
            .to_string()
    }
}
