use crate::{
    Category, ComparisonRule, DirectRule, Operator, PartRating, Rule, State, System, Workflow,
};

impl From<char> for Category {
    fn from(c: char) -> Self {
        match c {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => panic!("unrecognized category character"),
        }
    }
}

impl From<&str> for PartRating {
    fn from(input: &str) -> Self {
        // starts/ends with a '{' and '}'
        // then a comma-separated list that's always in the
        // order x=???, m=???, a=???, s=???
        let mut ratings = input[1..input.len() - 1]
            .split(',')
            .map(|assignment| assignment[2..].parse::<u32>().unwrap());
        Self {
            x: ratings.next().unwrap(),
            m: ratings.next().unwrap(),
            a: ratings.next().unwrap(),
            s: ratings.next().unwrap(),
        }
    }
}

impl<'a> From<&'a str> for Rule<'a> {
    fn from(input: &'a str) -> Self {
        if let Some((comparison, name)) = input.split_once(':') {
            // comparison rule
            let category = comparison.chars().next().unwrap().into();
            let operator = if comparison.chars().nth(1).unwrap() == '>' {
                Operator::GreaterThan
            } else {
                Operator::LessThan
            };
            let value = comparison[2..].parse::<u32>().unwrap();
            let destination = match name {
                "A" => State::Accept,
                "R" => State::Reject,
                name => State::Workflow(name),
            };
            Self::Comparison(ComparisonRule {
                category,
                operator,
                value,
                destination,
            })
        } else {
            // direct rule
            let destination = match input {
                "A" => State::Accept,
                "R" => State::Reject,
                name => State::Workflow(name),
            };
            Self::Direct(DirectRule { destination })
        }
    }
}

impl<'a> From<&'a str> for Workflow<'a> {
    fn from(input: &'a str) -> Self {
        // input looks like `name{rule1,rule2,rule3}`
        let (name, rules) = input.split_once('{').unwrap();
        let rules = rules[..rules.len() - 1]
            .split(',')
            .map(Rule::from)
            .collect();
        Self { name, rules }
    }
}

impl<'a> From<&'a str> for System<'a> {
    fn from(input: &'a str) -> Self {
        Self(
            input
                .lines()
                .map(|line| {
                    let workflow = Workflow::from(line);
                    (workflow.name, workflow)
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_part_rating() {
        let input = "{x=787,m=2655,a=1222,s=2876}";
        let part = PartRating::from(input);

        assert_eq!(
            part,
            PartRating {
                x: 787,
                m: 2655,
                a: 1222,
                s: 2876
            }
        );
    }

    #[test]
    fn test_parse_direct_rule() {
        let input = "A";
        let rule = Rule::from(input);

        assert_eq!(
            rule,
            Rule::Direct(DirectRule {
                destination: State::Accept
            })
        );

        let input = "R";
        let rule = Rule::from(input);

        assert_eq!(
            rule,
            Rule::Direct(DirectRule {
                destination: State::Reject
            })
        );

        let input = "nice";
        let rule = Rule::from(input);

        assert_eq!(
            rule,
            Rule::Direct(DirectRule {
                destination: State::Workflow("nice")
            })
        );
    }

    #[test]
    fn test_parse_comparision_rule() {
        let input = "a<2006:qkq";
        let rule = Rule::from(input);

        assert_eq!(
            rule,
            Rule::Comparison(ComparisonRule {
                category: Category::A,
                operator: Operator::LessThan,
                value: 2006,
                destination: State::Workflow("qkq")
            })
        );

        let input = "x>2440:R";
        let rule = Rule::from(input);

        assert_eq!(
            rule,
            Rule::Comparison(ComparisonRule {
                category: Category::X,
                operator: Operator::GreaterThan,
                value: 2440,
                destination: State::Reject
            })
        );
    }

    #[test]
    fn test_parse_workflow() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}";
        let workflow = Workflow::from(input);

        assert_eq!(
            workflow,
            Workflow {
                name: "px",
                rules: vec![
                    Rule::Comparison(ComparisonRule {
                        category: Category::A,
                        operator: Operator::LessThan,
                        value: 2006,
                        destination: State::Workflow("qkq")
                    }),
                    Rule::Comparison(ComparisonRule {
                        category: Category::M,
                        operator: Operator::GreaterThan,
                        value: 2090,
                        destination: State::Accept
                    }),
                    Rule::Direct(DirectRule {
                        destination: State::Workflow("rfg")
                    })
                ]
            }
        )
    }
}
