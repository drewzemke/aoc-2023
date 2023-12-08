use parser::ParsedNode;

pub mod puzzle08a;
pub mod puzzle08b;

pub mod parser;

#[derive(Debug, PartialEq, Eq)]
pub enum Step {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Node<'a> {
    name: &'a str,
    left_index: usize,
    right_index: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Graph<'a>(Vec<Node<'a>>);

impl<'a> From<Vec<ParsedNode<'a>>> for Graph<'a> {
    fn from(parsed_nodes: Vec<ParsedNode<'a>>) -> Self {
        let mut nodes: Vec<Node> = vec![];

        for ParsedNode { name, left, right } in &parsed_nodes {
            let left_index = parsed_nodes
                .iter()
                .position(|node| &node.name == left)
                .unwrap();
            let right_index = parsed_nodes
                .iter()
                .position(|node| &node.name == right)
                .unwrap();
            nodes.push(Node {
                name,
                left_index,
                right_index,
            });
        }

        Graph(nodes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ParsedNode;

    #[test]
    fn test_create_graph() {
        let nodes = vec![
            ParsedNode {
                name: "AAA",
                left: "BBB",
                right: "BBB",
            },
            ParsedNode {
                name: "BBB",
                left: "AAA",
                right: "ZZZ",
            },
            ParsedNode {
                name: "ZZZ",
                left: "ZZZ",
                right: "ZZZ",
            },
        ];
        let graph = Graph::from(nodes);

        assert_eq!(
            graph,
            Graph(vec![
                Node {
                    name: "AAA",
                    left_index: 1,
                    right_index: 1
                },
                Node {
                    name: "BBB",
                    left_index: 0,
                    right_index: 2
                },
                Node {
                    name: "ZZZ",
                    left_index: 2,
                    right_index: 2
                },
            ])
        );
    }
}
