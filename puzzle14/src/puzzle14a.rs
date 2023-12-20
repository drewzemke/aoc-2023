use common::puzzle::PuzzlePart;

use crate::{PlatformColumns, PlatformRows};

pub struct Puzzle14a {}

impl PuzzlePart for Puzzle14a {
    fn description() -> &'static str {
        "Find the total load on the north edge of a platform of rocks after tilting it to the north."
    }

    fn solve(input: &str) -> String {
        let rows = PlatformRows::from(input);
        let mut cols: PlatformColumns = rows.into();

        cols.tilt_north();

        let rows: PlatformRows = cols.into();
        rows.north_load().to_string()
    }
}
