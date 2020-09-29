use itertools::Itertools;
use std::collections::HashMap;
use std::collections::VecDeque;
use thiserror::Error;

mod models;

pub use models::Graph;
use models::{Calculation, Node};

impl Graph {
    pub fn from(input: &str) -> Result<Graph, GraphCreationError> {
        let graph: Graph = serde_json::from_str(input)?;

        let graph_root = graph
            .nodes
            .get(&graph.entry_point)
            .ok_or_else(|| GraphCreationError::InvalidRoot(graph.entry_point.clone()))?;
        let mut nodes_to_visit: VecDeque<(&Node, Vec<String>)> = VecDeque::new();
        nodes_to_visit.push_back((graph_root, vec![graph.entry_point.clone()]));
        while let Some((current_node, current_path)) = nodes_to_visit.pop_front() {
            if let Node::Standard { next, .. } = current_node {
                if next.is_empty() {
                    let node = current_path
                        .last()
                        .ok_or_else(|| GraphCreationError::InvalidRoot(graph.entry_point.clone()))?
                        .to_string();
                    return Err(GraphCreationError::InvalidEdges(node));
                }
                for node in next.values() {
                    let next_node = graph
                        .nodes
                        .get(node)
                        .ok_or_else(|| GraphCreationError::InvalidNode(node.clone()))?;

                    let mut next_path = current_path.clone();
                    next_path.push(node.clone());

                    if current_path.contains(node) {
                        return Err(GraphCreationError::Cycle(next_path));
                    }
                    nodes_to_visit.push_back((next_node, next_path));
                }
            }
        }

        Ok(graph)
    }

    pub fn evaluate(&self, values: &HashMap<String, i32>) -> Result<String, GraphEvaluationError> {
        let graph_root = self
            .nodes
            .get(&self.entry_point)
            .ok_or_else(|| unreachable!("Graph must be constructed with a root node"))?;
        self.evaluate_node(graph_root, values)
    }

    fn evaluate_node(
        &self,
        node: &Node,
        values: &HashMap<String, i32>,
    ) -> Result<String, GraphEvaluationError> {
        match node {
            Node::Standard { calculation, next } => match calculation {
                Calculation::LessThan { field, value } => {
                    let actual_value = values
                        .get(field)
                        .ok_or_else(|| GraphEvaluationError::InvalidField(field.clone()))?;
                    let next_key = if actual_value < value {
                        "true"
                    } else {
                        "false"
                    };
                    let next_node_key = next.get(next_key).ok_or_else(|| {
                        GraphEvaluationError::MissingNextKey(next_key.to_string())
                    })?;
                    let next_node = self
                        .nodes
                        .get(next_node_key)
                        .expect("All destinations must be valid");
                    self.evaluate_node(next_node, values)
                }
            },
            Node::Output { output } => Ok(output.clone()),
        }
    }
}

#[derive(Error, Debug)]
pub enum GraphCreationError {
    #[error("Could not parse the JSON as a graph")]
    InvalidJson(#[from] serde_json::Error),
    #[error("Graph contained a cycle {}", .0.iter().cloned().intersperse(String::from(" -> ")).collect::<String>())]
    Cycle(Vec<String>),
    #[error("Graph does not contain root node of {0}")]
    InvalidRoot(String),
    #[error("Graph points to a nonexistent node of {0}")]
    InvalidNode(String),
    #[error("Graph node {0} does not have any next edges")]
    InvalidEdges(String),
}

#[derive(Error, Debug)]
pub enum GraphEvaluationError {
    #[error("No value provided for {0}")]
    InvalidField(String),
    #[error("Expected node to contain key {0}")]
    MissingNextKey(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_json() {
        let error = Graph::from("{}");
        match error {
            Err(GraphCreationError::InvalidJson(x)) => {
                eprintln!("{}", x);
            }
            x => panic!("Expected an InvalidJson error, but got {:#?}", x),
        };
    }
}
