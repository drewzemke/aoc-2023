use crate::Schematic;
use common::puzzle::PuzzlePart;

pub struct Puzzle12a {}

impl PuzzlePart for Puzzle12a {
    fn description() -> &'static str {
        "Find the numbers of ways to arrange operational and damaged springs subject to a schematic."
    }

    fn solve(input: &str) -> String {
        input
            .lines()
            .map(Schematic::from)
            .map(|schematic| schematic.count_arrangements(&mut vec![]))
            .sum::<u64>()
            .to_string()
    }
}
