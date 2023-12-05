use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone)]
enum PuzzlePartName {
    A,
    B,
}

#[derive(Parser)]
struct PuzzleArgs {
    #[arg(long, short)]
    part: Option<PuzzlePartName>,

    #[arg(long = "example", short = 'e')]
    use_example: bool,
}

pub trait PuzzlePart {
    fn description() -> String;
    fn solve(input: &str) -> String;
}

pub trait Puzzle {
    type PartA: PuzzlePart;
    type PartB: PuzzlePart;

    fn run() {
        let args = PuzzleArgs::parse();

        let input = if args.use_example {
            include_str!("../data/example")
        } else {
            include_str!("../data/input")
        };

        match args.part {
            Some(PuzzlePartName::A) => Self::process::<Self::PartA>("A", input),
            Some(PuzzlePartName::B) => Self::process::<Self::PartB>("B", input),
            None => {
                Self::process::<Self::PartA>("A", input);
                println!("---");
                Self::process::<Self::PartB>("B", input);
            }
        };
    }

    fn process<P: PuzzlePart>(part_name: &str, input: &str) {
        println!("Solving Puzzle {part_name}:");
        println!("\"{}\"", <P as PuzzlePart>::description());
        let res = <P as PuzzlePart>::solve(input);
        println!("Solution: {res}")
    }
}
