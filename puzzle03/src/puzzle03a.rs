use crate::{element::Element, parser::SchematicLine, schematic::SchematicGraph};
use common::puzzle::PuzzlePart;

pub struct Puzzle03a {}

impl PuzzlePart for Puzzle03a {
    fn description() -> &'static str {
        "Sum the numbers of 'parts' in the schematic."
    }

    fn solve(input: &str) -> String {
        let schematic: SchematicGraph = input
            .lines()
            .map(SchematicLine::parse_from_str)
            .collect::<Vec<_>>()
            .into();

        let sum_of_part_numbers: u32 = schematic
            .nodes()
            .filter(|node| {
                schematic
                    .neighbors_of(node)
                    .any(|node| matches!(node.element(), Element::Symbol(..)))
            })
            .filter_map(|node| {
                if let Element::Number(n) = node.element() {
                    Some(n)
                } else {
                    None
                }
            })
            .sum();

        sum_of_part_numbers.to_string()
    }
}
