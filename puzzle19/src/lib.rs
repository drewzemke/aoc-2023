use std::collections::HashMap;

pub mod parser;
pub mod puzzle19a;
pub mod puzzle19b;

#[derive(Debug, PartialEq, Eq)]
pub struct PartRating {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl PartRating {
    pub fn total(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }

    fn rating(&self, category: &Category) -> u32 {
        match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    GreaterThan,
    LessThan,
}

#[derive(Debug, PartialEq, Eq)]
pub enum State<'a> {
    Accept,
    Reject,
    Workflow(&'a str),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ComparisonRule<'a> {
    category: Category,
    operator: Operator,
    value: u32,
    destination: State<'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DirectRule<'a> {
    destination: State<'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Rule<'a> {
    Comparison(ComparisonRule<'a>),
    Direct(DirectRule<'a>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct System<'a>(HashMap<&'a str, Workflow<'a>>);

impl<'a> System<'a> {
    fn accepts(&self, part: &PartRating) -> bool {
        let mut state = &State::Workflow("in");
        loop {
            match state {
                State::Accept => return true,
                State::Reject => return false,
                State::Workflow(name) => {
                    let workflow = self.0.get(name).unwrap();
                    for rule in &workflow.rules {
                        match rule {
                            Rule::Comparison(ComparisonRule {
                                category,
                                operator,
                                value,
                                destination,
                            }) => {
                                let rating = part.rating(category);
                                let matches = match operator {
                                    Operator::GreaterThan => rating > *value,
                                    Operator::LessThan => rating < *value,
                                };
                                if matches {
                                    state = destination;
                                    break;
                                }
                            }
                            Rule::Direct(DirectRule { ref destination }) => {
                                state = destination;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}

// for each "Accept" in the system
