use aoc_2023_01::{part_a::Puzzle01a, part_b::Puzzle01b};
use aoc_common::puzzle::Puzzle;

fn main() {
    let input = include_str!("../data/input");
    let example = include_str!("../data/example");

    Puzzle01::run(input, example);
}

struct Puzzle01 {}

impl Puzzle for Puzzle01 {
    type PartA = Puzzle01a;
    type PartB = Puzzle01b;
}
