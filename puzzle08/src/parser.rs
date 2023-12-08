use crate::Step;

pub fn parse_step_line(input: &str) -> Vec<Step> {
    input
        .chars()
        .map(|c| match c {
            'L' => Step::Left,
            'R' => Step::Right,
            _ => panic!("Unrecognized step character"),
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsedNode<'a> {
    pub name: &'a str,
    pub left: &'a str,
    pub right: &'a str,
}

// nodes look like:
// HGK = (LRV, NBJ)
// they always have three-letter names, so we can just slice
// into the input
pub fn parse_node(input: &str) -> ParsedNode {
    ParsedNode {
        name: &input[0..3],
        left: &input[7..10],
        right: &input[12..15],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Step;

    #[test]
    fn test_parse_step_line() {
        assert_eq!(
            parse_step_line("LLR"),
            vec![Step::Left, Step::Left, Step::Right,]
        )
    }

    #[test]
    fn test_parse_node() {
        assert_eq!(
            parse_node("HGK = (LRV, NBJ)"),
            ParsedNode {
                name: "HGK",
                left: "LRV",
                right: "NBJ",
            }
        )
    }
}
