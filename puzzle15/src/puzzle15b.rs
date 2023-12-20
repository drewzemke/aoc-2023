use common::puzzle::PuzzlePart;

use crate::{InitializationStep, Lens, LensArray};

pub struct Puzzle15b {}

impl PuzzlePart for Puzzle15b {
    fn description() -> &'static str {
        "Find the `focusing power` of the final configuration of a hashmap-esque array of boxes."
    }

    fn solve(input: &str) -> String {
        let steps: Vec<_> = input
            .trim()
            .split(',')
            .map(InitializationStep::from)
            .collect();

        let mut array = LensArray::new();
        for step in steps {
            match step {
                InitializationStep::Set { label, value } => {
                    array.add(Lens { label, value });
                }
                InitializationStep::Unset { label } => array.remove(label),
            }
        }

        array.focusing_power().to_string()
    }
}
