use common::puzzle::PuzzlePart;

pub struct Puzzle01a {}

impl PuzzlePart for Puzzle01a {
    fn description() -> &'static str {
        "Sum the two digit numbers formed from the first and last numerical digits in each line."
    }

    fn solve(input: &str) -> String {
        let mut sum: u32 = 0;

        for line in input.lines() {
            let first_digit = line.chars().find_map(|c| c.to_digit(10)).unwrap();
            let last_digit = line
                .chars()
                .rfind(|c| c.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap();

            let line_value = first_digit * 10 + last_digit;
            sum += line_value;
        }

        sum.to_string()
    }
}
