use crate::Pattern;
use common::puzzle::PuzzlePart;

pub struct Puzzle13b {}

impl PuzzlePart for Puzzle13b {
    fn description() -> &'static str {
        "Sum numbers obtained by figuring out symmetries of patterns of rock and ash, subject to a single smudge."
    }

    fn solve(input: &str) -> String {
        input
            .split("\n\n")
            .map(Pattern::from)
            .map(|pattern| {
                for idx in 0..pattern.width() - 1 {
                    if pattern.is_symmetric_across_vert(idx, 1) {
                        return idx + 1;
                    }
                }
                for idx in 0..pattern.height() - 1 {
                    if pattern.is_symmetric_across_horiz(idx, 1) {
                        return 100 * (idx + 1);
                    }
                }

                panic!("didn't find any symmetry :(")
            })
            .sum::<usize>()
            .to_string()
    }
}
