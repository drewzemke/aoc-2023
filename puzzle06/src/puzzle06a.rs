use crate::parser::parse;
use common::puzzle::PuzzlePart;

pub struct Puzzle06a {}

impl PuzzlePart for Puzzle06a {
    fn description() -> &'static str {
        "Compute the product of numbers of ways to win a boat race."
    }

    fn solve(input: &str) -> String {
        parse(input)
            .into_iter()
            .map(|(time, distance)| number_of_ways_to_win(time, distance))
            .product::<u32>()
            .to_string()
    }
}

// If the race is T seconds long, and you charge it for x seconds, then
// it will travel at x mm/s for (T-x) seconds, meaning it travels
// a total of x * (T-x) mm.
// We want to find the number of integer values of x for which the
// distance traveled is greater than some distance D:
//     x * (T-x) > D
// This is a quadratic equation:
//     x^2 - Tx + D < 0
// Its solutions are the integer values for which:
//     (T - sqrt(T^2 - 4*D)) / 2  <  x  < (T + sqrt(T^2 - 4*D)) / 2
// If those two numbers are not integers themselves, the number of integers
// between them is the difference of their floors.
// If they are integers, the number of integers between them is one less
// than their differences.
// Sooo let's just do all that!
fn number_of_ways_to_win(time: u32, distance: u32) -> u32 {
    let disc = (time * time - 4 * distance) as f32;
    let disc_sqrt = disc.powf(0.5);
    let left = (time as f32 - disc_sqrt) / 2.0;
    let right = (time as f32 + disc_sqrt) / 2.0;

    // if disc_sqrt is an integer...
    if disc_sqrt.floor() == disc_sqrt {
        (right - left) as u32 - 1
    } else {
        (right.floor() as u32) - (left.floor() as u32)
    }
}

#[test]
fn test_number_of_way_to_win() {
    assert_eq!(number_of_ways_to_win(7, 9), 4);
    assert_eq!(number_of_ways_to_win(15, 40), 8);
    assert_eq!(number_of_ways_to_win(30, 200), 9);
}
