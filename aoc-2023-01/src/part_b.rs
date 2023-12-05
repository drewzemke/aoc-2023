use crate::common::PuzzlePart;
use crate::find_digits;

pub struct Puzzle01b {}

impl PuzzlePart for Puzzle01b {
    fn description() -> String {
        "Sum the two digit numbers formed from the first and last numerical _or word_ digits in each line."
            .into()
    }

    fn solve(input: &str) -> String {
        let mut sum: u32 = 0;

        for line in input.lines() {
            let digits = find_digits(line);
            let first_digit = digits.first().unwrap();
            let last_digit = digits.last().unwrap();

            let line_value = first_digit * 10 + last_digit;
            sum += line_value;
        }

        sum.to_string()
    }
}
