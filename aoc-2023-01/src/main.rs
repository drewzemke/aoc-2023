use aoc_2023_01::{common::Puzzle, part_a::Puzzle01a, part_b::Puzzle01b};

fn main() {
    Puzzle01::run();
}

struct Puzzle01 {}

impl Puzzle for Puzzle01 {
    type PartA = Puzzle01a;
    type PartB = Puzzle01b;
}
