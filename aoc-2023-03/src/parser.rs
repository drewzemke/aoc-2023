use crate::element::{Element, ElementSpan};
use nom::{
    branch::alt,
    character::complete::{char, digit1},
    combinator::{map, map_res, value},
    multi::many1,
    IResult,
};

#[derive(Debug)]
pub struct SchematicLine(Vec<ElementSpan>);

impl SchematicLine {
    pub fn parse_from_str(input: &str) -> SchematicLine {
        let mut input = input;
        let mut offset = 0;
        let mut elements: Vec<ElementSpan> = vec![];

        while let Ok((remainder, element)) = parse_element(input) {
            let parsed_length = input.len() - remainder.len();
            let range = offset..(offset + parsed_length);

            if let Some(element) = element {
                elements.push(ElementSpan { element, range });
            }

            input = remainder;
            offset += parsed_length;
        }

        SchematicLine(elements)
    }

    pub fn into_spans(self) -> impl Iterator<Item = ElementSpan> {
        self.0.into_iter()
    }
}

fn parse_element(input: &str) -> IResult<&str, Option<Element>> {
    alt((
        map(parse_symbol, |c| Some(Element::Symbol(c))),
        map(parse_num, |n| Some(Element::Number(n))),
        map(parse_dots, |_| None),
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
