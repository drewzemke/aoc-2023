use common::puzzle::PuzzlePart;

use crate::System;

pub struct Puzzle19b {}

impl PuzzlePart for Puzzle19b {
    fn description() -> &'static str {
        "Find the total number of parts that will be accepted by a system of workflows."
    }

    fn solve(input: &str) -> String {
        let system = System::from(input.split("\n\n").next().unwrap());

        system.count_accepted_parts().to_string()
    }
}
