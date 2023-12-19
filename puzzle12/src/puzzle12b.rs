use crate::Schematic;
use common::puzzle::PuzzlePart;

pub struct Puzzle12b {}

impl PuzzlePart for Puzzle12b {
    fn description() -> &'static str {
        "Find the numbers of ways to arrange springs subject to an expanded schematic."
    }

    fn solve(input: &str) -> String {
        let mut memory = vec![];
        input
            .lines()
            .map(Schematic::from)
            .map(|schematic| schematic.unfold())
            .map(|schematic| schematic.count_arrangements(&mut memory))
            .sum::<u64>()
            .to_string()
    }
}
