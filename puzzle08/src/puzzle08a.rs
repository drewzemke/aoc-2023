use crate::{
    parser::{parse_node, parse_step_line, ParsedNode},
    Graph, Step,
};
use common::puzzle::PuzzlePart;

pub struct Puzzle08a {}

impl PuzzlePart for Puzzle08a {
    fn description() -> &'static str {
        "Find the number of steps needed to traverse a graph using a certain step pattern."
    }

    fn solve(input: &str) -> String {
        let steps = parse_step_line(input.lines().next().unwrap());
        let parsed_nodes: Vec<ParsedNode> = input.lines().skip(2).map(|s| parse_node(s)).collect();
        let graph = Graph::from(parsed_nodes);
        let step_count = graph.traverse(steps);

        step_count.to_string()
    }
}

impl<'a> Graph<'a> {
    pub fn traverse(&'a self, steps: Vec<Step>) -> u32 {
        let mut index = self.0.iter().position(|node| node.name == "AAA").unwrap();
        let end_index = self.0.iter().position(|node| node.name == "ZZZ").unwrap();

        let mut step_count = 0;
        let mut step_index = 0;

        while index != end_index {
            let node = &self.0[index];
            index = match steps[step_index] {
                Step::Left => node.left_index,
                Step::Right => node.right_index,
            };

            step_count += 1;
            step_index = (step_index + 1) % steps.len();
        }

        step_count
    }
}
