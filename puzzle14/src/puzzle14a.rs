use crate::{Direction, Platform};
use common::puzzle::PuzzlePart;

pub struct Puzzle14a {}

impl PuzzlePart for Puzzle14a {
    fn description() -> &'static str {
        "Find the total load on the north edge of a platform of rocks after tilting it to the north."
    }

    fn solve(input: &str) -> String {
        let mut platform = Platform::from(input);

        platform.tilt(Direction::North);

        platform.north_load().to_string()
    }
}
