use nom::{
    branch::alt,
    character::complete::{char, digit1},
    combinator::{map, map_res, value},
    multi::many1,
    IResult,
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Range,
};

#[derive(Debug)]
struct Schematic(Vec<SchematicLine>);

impl Schematic {
    fn part_numbers(&self) -> Vec<u32> {
        self.0
            .windows(3)
            .flat_map(|window| {
                if let [prev, curr, next] = window {
                    curr.0
                        .iter()
                        .filter_map(|ElementSpan { element, range }| {
                            if let Element::Number(number) = element {
                                let left_boundary =
                                    if range.start == 0 { 0 } else { range.start - 1 };
                                if curr.has_symbol_in_range(left_boundary..range.start)
                                    || curr.has_symbol_in_range(range.end..range.end + 1)
                                    || prev.has_symbol_in_range(left_boundary..range.end + 1)
                                    || next.has_symbol_in_range(left_boundary..range.end + 1)
                                {
                                    Some(*number)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<u32>>()
                } else {
                    unreachable!()
                }
            })
            .collect()
    }

    fn gear_ratios(&self) -> Vec<u32> {
        vec![]
    }
}

#[derive(Debug)]
struct SchematicLine(Vec<ElementSpan>);

impl SchematicLine {
    fn has_symbol_in_range(&self, range: Range<usize>) -> bool {
        self.0.iter().any(
            |ElementSpan {
                 element,
                 range: element_range,
             }| {
                matches!(element, Element::Symbol(..)) && range.contains(&element_range.start)
            },
        )
    }
}

#[derive(Debug)]
enum Element {
    Number(u32),
    Symbol(char),
    Dots,
}

#[derive(Debug)]
struct ElementSpan {
    element: Element,
    range: Range<usize>,
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let schematic = Schematic(
        reader
            .lines()
            .map(Result::unwrap)
            .map(|line| parse_schematic_line(&line))
            .collect(),
    );

    let sum_of_parts: u32 = schematic.part_numbers().iter().sum();
    let sum_of_gear_ratios: u32 = schematic.gear_ratios().iter().sum();

    println!("puzzle 1: {sum_of_parts}");
    println!("puzzle 2: {sum_of_gear_ratios}");
}

fn parse_schematic_line(input: &str) -> SchematicLine {
    let mut input = input;
    let mut offset = 0;
    let mut elements: Vec<ElementSpan> = vec![];

    while let Ok((remainder, element)) = parse_element(input) {
        let parsed_length = input.len() - remainder.len();
        let range = offset..(offset + parsed_length);

        elements.push(ElementSpan { element, range });

        input = remainder;
        offset += parsed_length;
    }

    SchematicLine(elements)
}

fn parse_element(input: &str) -> IResult<&str, Element> {
    alt((
        map(parse_symbol, Element::Symbol),
        map(parse_num, Element::Number),
        map(parse_dots, |_| Element::Dots),
    ))(input)
}

fn parse_symbol(input: &str) -> IResult<&str, char> {
    alt((
        char('#'),
        char('$'),
        char('-'),
        char('*'),
        char('+'),
        char('&'),
        char('/'),
        char('@'),
        char('%'),
        char('='),
    ))(input)
}

fn parse_num(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_dots(input: &str) -> IResult<&str, ()> {
    value((), many1(char('.')))(input)
}
