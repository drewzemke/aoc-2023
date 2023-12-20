use crate::{Direction, TileGrid};
use common::puzzle::PuzzlePart;

pub struct Puzzle16b {}

impl PuzzlePart for Puzzle16b {
    fn description() -> &'static str {
        "Find the maximum number of tiles that are energized by a beam of light with arbitrary starting location."
    }

    fn solve(input: &str) -> String {
        let mut grid = TileGrid::from(input);
        let mut energized_tiles = vec![];

        for row_idx in 0..grid.height() {
            grid.trace_beams((row_idx, 0), Direction::Right);
            energized_tiles.push(grid.energized_tiles());
            grid.reset();

            grid.trace_beams((row_idx, grid.width()), Direction::Left);
            energized_tiles.push(grid.energized_tiles());
            grid.reset();
        }

        for col_idx in 0..grid.width() {
            grid.trace_beams((0, col_idx), Direction::Down);
            energized_tiles.push(grid.energized_tiles());
            grid.reset();

            grid.trace_beams((grid.height(), col_idx), Direction::Up);
            energized_tiles.push(grid.energized_tiles());
            grid.reset();
        }

        energized_tiles.iter().max().unwrap().to_string()
    }
}
