use common::puzzle::PuzzlePart;

use crate::{PartRating, System};

pub struct Puzzle19a {}

impl PuzzlePart for Puzzle19a {
    fn description() -> &'static str {
        "Find the total ratings of the parts that accepted by a system of workflows."
    }

    fn solve(input: &str) -> String {
        let (system, parts) = input.split_once("\n\n").unwrap();
        let system = System::from(system);
        let parts: Vec<_> = parts.lines().map(PartRating::from).collect();

        parts
            .iter()
            .filter(|part| system.accepts(part))
            .map(|part| part.total())
            .sum::<u32>()
            .to_string()
    }
}
