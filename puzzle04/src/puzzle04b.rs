use crate::ScratchCard;
use common::puzzle::PuzzlePart;

pub struct Puzzle04b {}

impl PuzzlePart for Puzzle04b {
    fn description() -> &'static str {
        "Count the total number of cards obtained by a process where winning cards grants you more cards."
    }

    fn solve(input: &str) -> String {
        let matches: Vec<usize> = input
            .lines()
            .map(ScratchCard::parse_from_str)
            .map(|card| card.matches())
            .collect();

        (0..matches.len())
            .map(|index| recursive_count(&matches[index..]))
            .sum::<usize>()
            .to_string()
    }
}

fn recursive_count(list: &[usize]) -> usize {
    if list.is_empty() {
        return 0;
    }

    // one to count the current card
    // then, for each match in the current card, call this function with a slice of the same list
    //   that's offset by incremental amounts
    1 + (0..list[0])
        .map(|start| recursive_count(&list[start + 1..]))
        .sum::<usize>()
}

#[test]
fn test_recursive_count() {
    let list = vec![2, 1, 0];
    assert_eq!(recursive_count(&list), 4);
}
