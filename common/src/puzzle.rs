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
    /// A description of the value(s) computed for this puzzle,
    /// just for the sake of more meaningful output.
    fn description() -> &'static str;

    /// Do all of the work necessary to transform the input text into
    /// the solution text.
    fn solve(input: &str) -> String;
}

pub trait Puzzle {
    type PartA: PuzzlePart;
    type PartB: PuzzlePart;

    /// The name of the puzzle, usually just the number (eg. "01")
    fn name() -> &'static str;

    /// Based on command line args, this executes the solver for one or both
    /// parts of a day's puzzles, using either the primary input or the example input.
    fn run(input: &str, example: &str) {
        let args = PuzzleArgs::parse();

        let input = if args.use_example { example } else { input };

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
        println!("Solving Puzzle {} Part {part_name}:", Self::name());
        println!("\"{}\"", <P as PuzzlePart>::description());
        let res = <P as PuzzlePart>::solve(input);
        println!("Solution: {res}")
    }
}
