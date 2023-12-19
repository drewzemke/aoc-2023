use crate::Universe;
use common::puzzle::PuzzlePart;

pub struct Puzzle11a {}

impl PuzzlePart for Puzzle11a {
    fn description() -> &'static str {
        "Find the pairwise distances between galaxies in an expanded universe."
    }

    fn solve(input: &str) -> String {
        let universe = Universe::from(input);

        universe
            .galaxy_pairs()
            .iter()
            .map(|(galaxy1, galaxy2)| universe.expanded_distance(galaxy1, galaxy2, 2))
            .sum::<usize>()
            .to_string()
    }
}
