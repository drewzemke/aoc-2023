use common::puzzle::PuzzlePart;

use crate::Path;

pub struct Puzzle24a {}

// const MIN_COORD: f64 = 7.0;
// const MAX_COORD: f64 = 27.0;
const MIN_COORD: f64 = 200000000000000.0;
const MAX_COORD: f64 = 400000000000000.0;

impl PuzzlePart for Puzzle24a {
    fn description() -> &'static str {
        "Count the number of pairs of paths that whose projections to the xy-plane intersect in a certain area."
    }

    fn solve(input: &str) -> String {
        let paths: Vec<_> = input.lines().map(Path::from).collect();

        paths
            .iter()
            .enumerate()
            .flat_map(|(idx, path1)| {
                paths[idx + 1..].iter().filter(|path2| {
                    path1.intersection_times(path2).is_some_and(|(t1, t2)| {
                        t1 > 0.0
                            && t2 > 0.0
                            && path1.at(t1).has_xy_in(MIN_COORD, MAX_COORD)
                            && path2.at(t2).has_xy_in(MIN_COORD, MAX_COORD)
                    })
                })
            })
            .count()
            .to_string()
    }
}
