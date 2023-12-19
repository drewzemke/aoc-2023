use crate::Universe;
use common::puzzle::PuzzlePart;

pub struct Puzzle11b {}

impl PuzzlePart for Puzzle11b {
    fn description() -> &'static str {
        "Find the pairwise distances between galaxies in a much-more-expanded universe."
    }

    fn solve(input: &str) -> String {
        let universe = Universe::from(input);

        universe
            .galaxy_pairs()
            .iter()
            .map(|(galaxy1, galaxy2)| universe.expanded_distance(galaxy1, galaxy2, 1_000_000))
            .sum::<usize>()
            .to_string()
    }
}
