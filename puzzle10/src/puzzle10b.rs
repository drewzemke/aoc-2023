use common::puzzle::PuzzlePart;

use crate::PipeGrid;

pub struct Puzzle10b {}

impl PuzzlePart for Puzzle10b {
    fn description() -> &'static str {
        "Find the number of tiles enclosed by a loop of pipes."
    }

    fn solve(input: &str) -> String {
        let mut grid = PipeGrid::from(input);
        let pipe_loop = grid.find_loop();

        grid.count_in_loop(&pipe_loop).to_string()
    }
}
