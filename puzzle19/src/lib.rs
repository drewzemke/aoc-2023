use part_set::PartSet;
use std::collections::HashMap;

pub mod parser;
pub mod part_set;
pub mod puzzle19a;
pub mod puzzle19b;

#[derive(Debug, PartialEq, Eq)]
pub struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    pub fn total(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }

    fn rating(&self, category: &Category) -> u64 {
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
    value: u64,
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
    fn accepts(&self, part: &Part) -> bool {
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

    fn count_accepted_parts(&self) -> u64 {
        let start_state = State::Workflow("in");
        let start_set = PartSet {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        };

        let accepted_sets = self.find_accepted_sets(&start_set, &start_state);

        // now refine these sets into disjoint sets
        let mut refined_sets = vec![];
        for (refining_index, refining_set) in accepted_sets.iter().enumerate() {
            let mut working_refinement = vec![refining_set.clone()];
            for inner_set in &accepted_sets[refining_index + 1..] {
                working_refinement = working_refinement
                    .into_iter()
                    .flat_map(|set| inner_set.refine(&set))
                    .collect()
            }
            refined_sets.append(&mut working_refinement);
        }

        refined_sets.iter().map(|set| set.size()).sum::<u64>()
    }

    fn find_accepted_sets(&self, set: &PartSet, state: &State) -> Vec<PartSet> {
        match state {
            State::Accept => vec![set.clone()],
            State::Reject => vec![],
            State::Workflow(name) => {
                let workflow = self.0.get(name).unwrap();
                let mut working_set = set.clone();
                let mut new_sets = vec![];

                for rule in &workflow.rules {
                    match rule {
                        Rule::Comparison(ComparisonRule {
                            category,
                            operator,
                            value,
                            destination,
                        }) => {
                            let (matching, rest) = working_set.split(category, operator, *value);
                            if let Some(matching) = matching {
                                new_sets
                                    .append(&mut self.find_accepted_sets(&matching, destination));
                            }
                            if let Some(rest) = rest {
                                working_set = rest;
                            } else {
                                break;
                            }
                        }
                        Rule::Direct(DirectRule { destination }) => {
                            new_sets.append(&mut self.find_accepted_sets(&working_set, destination))
                        }
                    }
                }

                new_sets
            }
        }
    }
}

// for each "Accept" in the system
