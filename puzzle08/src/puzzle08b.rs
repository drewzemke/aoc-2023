use common::puzzle::PuzzlePart;

use crate::{
    parser::{parse_node, parse_step_line, ParsedNode},
    Graph, Step,
};

pub struct Puzzle08b {}

impl PuzzlePart for Puzzle08b {
    fn description() -> &'static str {
        "Find the number of steps needed to do multiple simultaneous traversals of a graph."
    }

    // NOTE: This solution relies on a (fortunately correct) assumption that the correct path uses the
    // whole set of left/right steps a whole number of times. So we can compute the result applying all 281 steps
    // to get a `map` that takes each node (referred to by its index in the graph) to the node you end up
    // after the full set of steps. We then iteratively apply that map until it takes the
    // starting node set ("__A") into the ending node set ("__Z").
    //
    // I don't love this solution, but it works so at least there's that. It took a hot 25 minutes
    // to run, meaning without the map optimization it would take something like five days to run. Yikes.
    fn solve(input: &str) -> String {
        let steps = parse_step_line(input.lines().next().unwrap());
        let parsed_nodes: Vec<ParsedNode> = input.lines().skip(2).map(|s| parse_node(s)).collect();
        let graph = Graph::from(parsed_nodes);
        let map = graph.build_map(&steps);

        let step_count = graph.map_traverse(&map);

        (step_count as usize * steps.len()).to_string()
    }
}

impl<'a> Graph<'a> {
    // returns the list of resulting nodes (their indices, actually) obtained by
    // following the full set of steps
    pub fn build_map(&'a self, steps: &[Step]) -> Vec<usize> {
        let mut indices: Vec<_> = (0..self.0.len()).collect();

        for step in steps {
            indices.iter_mut().for_each(|index| {
                let node = &self.0[*index];
                *index = match step {
                    Step::Left => node.left_index,
                    Step::Right => node.right_index,
                }
            });
        }

        indices
    }

    pub fn map_traverse(&'a self, map: &[usize]) -> u64 {
        let mut indices: Vec<_> = self
            .0
            .iter()
            .enumerate()
            .filter_map(|(index, node)| {
                if node.name.ends_with('A') {
                    Some(index)
                } else {
                    None
                }
            })
            .collect();

        let end_indices: Vec<_> = self
            .0
            .iter()
            .enumerate()
            .filter_map(|(index, node)| {
                if node.name.ends_with('Z') {
                    Some(index)
                } else {
                    None
                }
            })
            .collect();

        let mut step_count = 0;

        loop {
            let n = indices
                .iter()
                .filter(|index| end_indices.contains(index))
                .count();
            // if n > 4 {
            //     println!("got {n} matches at step : {step_count}")
            // }
            if n == indices.len() {
                break;
            }

            indices.iter_mut().for_each(|index| *index = map[*index]);
            step_count += 1;
        }

        step_count
    }
}
