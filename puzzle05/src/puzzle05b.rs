use crate::{parser::SeedParseStrategy, MapSet};
use common::puzzle::PuzzlePart;

pub struct Puzzle05b {}

impl PuzzlePart for Puzzle05b {
    fn description() -> &'static str {
        "Find the smallest location that can be obtained by passing a set of seeds through a series of maps, but with many more seeds."
    }

    fn solve(input: &str) -> String {
        let map_set = MapSet::parse_from_str(input, SeedParseStrategy::PairedRanges);
        map_set.smallest_output().to_string()
    }
}
