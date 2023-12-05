use crate::{
    element::Element,
    parser::SchematicLine,
    schematic::{SchematicGraph, SchematicNode},
};
use common::puzzle::PuzzlePart;

pub struct Puzzle03b {}

impl PuzzlePart for Puzzle03b {
    fn description() -> &'static str {
        "Sum the 'gear ratios' of particular parts in the schematic."
    }

    fn solve(input: &str) -> String {
        let schematic: SchematicGraph = input
            .lines()
            .map(SchematicLine::parse_from_str)
            .collect::<Vec<_>>()
            .into();

        let sum_of_gear_ratios: u32 = schematic
            .nodes()
            .filter(|node| matches!(node.element(), Element::Symbol('*')))
            .filter_map(|node| {
                let mut neighbors = schematic.neighbors_of(node);

                // look for _exactly_ two neighbors
                let Some(Element::Number(num1)) = neighbors.next().map(SchematicNode::element)
                else {
                    return None;
                };
                let Some(Element::Number(num2)) = neighbors.next().map(SchematicNode::element)
                else {
                    return None;
                };
                if neighbors.next().is_none() {
                    Some(num1 * num2)
                } else {
                    None
                }
            })
            .sum();

        sum_of_gear_ratios.to_string()
    }
}
