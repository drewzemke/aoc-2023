use crate::{Direction, Platform};
use common::puzzle::PuzzlePart;

pub struct Puzzle14b {}

const TOTAL_CYCLES: usize = 1_000_000_000;
const PATTERN_WATCH_START: usize = 1_000;

// NOTE: This wasn't too much a surprise tbh, but it turns out the
//   load values at the end of each cycle stabilizes into a repeating
//   pattern fairly quickly, so we can just inspect that pattern and
//   extrapolate the final value.
// You could also run the whole computation, but that would take _a while_.
impl PuzzlePart for Puzzle14b {
    fn description() -> &'static str {
        "Find the total load on the north edge of a platform of rocks after a billion cycles of tilting."
    }

    fn solve(input: &str) -> String {
        let mut platform = Platform::from(input);

        let mut pattern = vec![];
        for iter in 0..TOTAL_CYCLES {
            if iter >= PATTERN_WATCH_START {
                // record values of the loads until they start repeating
                let load = platform.north_load();
                if pattern.first().is_some_and(|value| *value == load) {
                    // stop processing and try to extrapolate from here
                    break;
                }
                pattern.push(load);
            }

            platform.tilt(Direction::North);
            platform.tilt(Direction::West);
            platform.tilt(Direction::South);
            platform.tilt(Direction::East);
        }

        // assuming the pattern we recorded repeats forever, we can index into it
        // to find what it will be on the last iteration
        let index = (TOTAL_CYCLES - PATTERN_WATCH_START) % pattern.len();
        pattern[index].to_string()
    }
}
