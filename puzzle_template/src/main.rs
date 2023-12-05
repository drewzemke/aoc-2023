use common::puzzle::Puzzle;
use puzzle00::{puzzle00a::Puzzle00a, puzzle00b::Puzzle00b};

fn main() {
    let input = include_str!("../data/input");
    let example = include_str!("../data/example");

    Puzzle00::run(input, example);
}

struct Puzzle00 {}

impl Puzzle for Puzzle00 {
    type PartA = Puzzle00a;
    type PartB = Puzzle00b;

    fn name() -> &'static str {
        todo!()
    }
}
