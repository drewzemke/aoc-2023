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
            .flat_map(|(line_num, line)| line.into_spans().map(move |element| (element, line_num)))
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
                let neighbor_ids: Vec<_> = tagged_nodes
                    .iter()
                    .filter(|(_, other_line_num, other_range)| {
                        if line_num == other_line_num {
                            // check for adjacent nodes on the same line
                            range.start == other_range.end || other_range.start == range.end
                        } else if line_num.abs_diff(*other_line_num) == 1 {
                            // check on lines above/below current line
                            let expanded_range = other_range.start.max(1) - 1..other_range.end + 1;
                            range.start < expanded_range.end && expanded_range.start < range.end
                        } else {
                            false
                        }
                    })
                    .map(|(node, _, _)| node.id)
                    .collect();

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
