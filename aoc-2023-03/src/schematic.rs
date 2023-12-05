use std::ops::Range;

use crate::{element::Element, parser::SchematicLine};

type NodeId = usize;

#[derive(Debug)]
pub struct SchematicNode {
    id: usize,
    element: Element,
    neighbor_ids: Vec<NodeId>,
}

impl SchematicNode {
    pub fn new(id: NodeId, element: Element) -> Self {
        Self {
            id,
            element,
            neighbor_ids: vec![],
        }
    }

    pub fn element(&self) -> &Element {
        &self.element
    }
}

#[derive(Debug)]
pub struct SchematicGraph(Vec<SchematicNode>);

impl SchematicGraph {
    pub fn nodes(&self) -> impl Iterator<Item = &SchematicNode> {
        self.0.iter()
    }

    pub fn neighbors_of<'a>(
        &'a self,
        node: &'a SchematicNode,
    ) -> impl Iterator<Item = &SchematicNode> + 'a {
        node.neighbor_ids.iter().map(|nbr_idx| &self.0[*nbr_idx])
    }
}

impl From<Vec<SchematicLine>> for SchematicGraph {
    fn from(lines: Vec<SchematicLine>) -> Self {
        // reorganize into tupeles of the form (data, line_number, span)
        let tagged_nodes: Vec<(SchematicNode, usize, Range<usize>)> = lines
            .into_iter()
            .enumerate()
            .flat_map(|(line_num, line)| line.0.into_iter().map(move |element| (element, line_num)))
            .enumerate()
            .map(|(index, (span, line_num))| {
                (
                    SchematicNode::new(index, span.element),
                    line_num,
                    span.range,
                )
            })
            .collect();

        // compute adjacencies
        let nodes: Vec<SchematicNode> = tagged_nodes
            .iter()
            .map(|(node, line_num, range)| {
                let same_line_nodes_ids: Vec<_> = tagged_nodes
                    .iter()
                    .filter(|(_, other_line_num, _)| line_num == other_line_num)
                    .filter(|(_, _, other_range)| {
                        range.start == other_range.end || other_range.start == range.end
                    })
                    .map(|(node, _, _)| node.id)
                    .collect();

                let mut above_or_below_nodes: Vec<_> = tagged_nodes
                    .iter()
                    .filter(|(_, other_line_num, _)| line_num.abs_diff(*other_line_num) == 1)
                    .filter(|(_, _, other_range)| {
                        let expanded_range = other_range.start.max(1) - 1..other_range.end + 1;
                        range.start < expanded_range.end && expanded_range.start < range.end
                    })
                    .map(|(node, _, _)| node.id)
                    .collect();

                let mut neighbor_ids = same_line_nodes_ids;
                neighbor_ids.append(&mut above_or_below_nodes);

                SchematicNode {
                    id: node.id,
                    element: node.element.clone(),
                    neighbor_ids,
                }
            })
            .collect();

        Self(nodes)
    }
}
