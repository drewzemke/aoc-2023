use crate::MapSet;
use common::puzzle::PuzzlePart;

pub struct Puzzle05a {}

impl PuzzlePart for Puzzle05a {
    fn description() -> &'static str {
        "Find the smallest location that can be obtained by passing a set of seeds through a series of maps."
    }

    fn solve(input: &str) -> String {
        let map_set = MapSet::parse_from_str(input);

        map_set.seed_outputs().iter().min().unwrap().to_string()
    }
}
