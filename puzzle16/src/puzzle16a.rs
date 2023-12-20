use crate::TileGrid;
use common::puzzle::PuzzlePart;

pub struct Puzzle16a {}

impl PuzzlePart for Puzzle16a {
    fn description() -> &'static str {
        "Count how many tiles are energized by light bouncing around a grid."
    }

    fn solve(input: &str) -> String {
        let mut grid = TileGrid::from(input);
        grid.trace_beams((0, 0), crate::Direction::Right);
        grid.energized_tiles().to_string()
    }
}
