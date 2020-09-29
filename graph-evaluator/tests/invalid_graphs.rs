use graph_evaluator::{Graph, GraphCreationError};

#[test]
fn evaluate_a_graph_with_a_cycle() {
    let graph = include_str!("data/cycle_graph.json");
    let error = Graph::from(graph).expect_err("Should not have produced a valid graph");
    match error {
        GraphCreationError::Cycle(cycle) => assert_eq!(
            cycle,
            vec![
                String::from("entry"),
                String::from("a_node"),
                String::from("b_node"),
                String::from("c_node"),
                String::from("a_node"),
            ]
        ),
        x => panic!("Expected a cycle, but got {:#?}", x),
    }
}

#[test]
fn evaluate_a_graph_with_an_invalid_entry() {
    let graph = include_str!("data/empty_graph.json");
    let error = Graph::from(graph).expect_err("Should not have produced a valid graph");
    match error {
        GraphCreationError::InvalidRoot(root) => {
            assert_eq!(root, String::from("nonexistent_entry"))
        }
        x => panic!("Expected to have an invalid root node, but got {:#?}", x),
    }
}

#[test]
fn evaluate_a_graph_with_an_invalid_next_node() {
    let graph = include_str!("data/broken_graph.json");
    let error = Graph::from(graph).expect_err("Should not have produced a valid graph");
    match error {
        GraphCreationError::InvalidNode(node) => assert_eq!(node, String::from("nonexistent_node")),
        x => panic!("Expected to have an invalid node, but got {:#?}", x),
    }
}

#[test]
fn evaluate_a_graph_with_empty_edges() {
    let graph = include_str!("data/missing_edges_graph.json");
    let error = Graph::from(graph).expect_err("Should not have produced a valid graph");
    match error {
        GraphCreationError::InvalidEdges(node) => assert_eq!(node, String::from("entry")),
        x => panic!("Expected to be missing edges, but got {:#?}", x),
    }
}
