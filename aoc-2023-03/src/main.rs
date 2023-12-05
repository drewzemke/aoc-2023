use aoc_2023_03::{
    element::Element,
    parser::SchematicLine,
    schematic::{SchematicGraph, SchematicNode},
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let schematic_lines: Vec<_> = reader
        .lines()
        .map(Result::unwrap)
        .map(|line| SchematicLine::parse_from_str(&line))
        .collect();
    let schematic: SchematicGraph = schematic_lines.into();

    // puzzle1
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

    println!("puzzle 1: {sum_of_part_numbers}");

    // puzzle2
    let sums_of_gear_ratios: u32 = schematic
        .nodes()
        .filter(|node| matches!(node.element(), Element::Symbol('*')))
        .filter_map(|node| {
            let mut neighbors = schematic.neighbors_of(node);

            // look for _exactly_ two neighbors
            let Some(Element::Number(num1)) = neighbors.next().map(SchematicNode::element) else {
                return None;
            };
            let Some(Element::Number(num2)) = neighbors.next().map(SchematicNode::element) else {
                return None;
            };
            if neighbors.next().is_none() {
                Some(num1 * num2)
            } else {
                None
            }
        })
        .sum();

    println!("puzzle 2: {sums_of_gear_ratios}");
}
