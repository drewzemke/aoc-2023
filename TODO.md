## what do I want to be able to do?

```shell
# both parts of puzzle 02 with primary input
cargo run --bin puzzle02
# just part a of day 01 with example input
cargo run --bin puzzle01 --part a --example
cargo run --bin puzzle01 -p A -e
```

`Puzzle`
puzzles take input (a big-ass string) and return output (impl Display?)
- a day number
- solver for each part
- input and example text

`PuzzlePart`
each puzzle part has
- whether it's the first or second (a/b)
- a function to compute it
- a label for the answer (what's being computed)


