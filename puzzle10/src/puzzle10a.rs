use common::puzzle::PuzzlePart;

use crate::PipeGrid;

pub struct Puzzle10a {}

impl PuzzlePart for Puzzle10a {
    fn description() -> &'static str {
        "Find the distance between the start and furthest point along a pipe loop."
    }

    fn solve(input: &str) -> String {
        let mut grid = PipeGrid::from(input);
        let pipe_loop = grid.find_loop();

        // the loop always has even length, and the point furthest from the
        // start will be half the length of the pipe away
        (pipe_loop.len() / 2).to_string()
    }
}
