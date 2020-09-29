use std::collections::HashMap;
use std::iter::FromIterator;

use graph_evaluator::Graph;
use graph_evaluator::GraphEvaluationError;

#[test]
fn evaluate_a_graph_with_a_field_that_is_missing() {
    let graph = include_str!("data/example_graph.json");
    let graph = Graph::from(graph).unwrap();

    let model: HashMap<String, i32> =
        HashMap::from_iter(vec![(String::from("number_of_good_things"), 5)]);

    let error = graph
        .evaluate(&model)
        .expect_err("Should not have successfully evaluated");
    match error {
        GraphEvaluationError::InvalidField(field) => {
            assert_eq!(field, String::from("number_of_bad_things"))
        }
        _ => panic!("Expected an invalid field error, but got {:#?}", error),
    }
}

#[test]
fn evaluate_a_graph_with_keys_that_do_not_match_the_operation() {
    let graph = include_str!("data/bad_next_keys_graph.json");
    let graph = Graph::from(graph).unwrap();

    let model: HashMap<String, i32> =
        HashMap::from_iter(vec![(String::from("number_of_bad_things"), 5)]);

    let error = graph
        .evaluate(&model)
        .expect_err("Should not have successfully evaluated");
    match error {
        GraphEvaluationError::MissingNextKey(key) => assert_eq!(key, String::from("true")),
        _ => panic!("Expected an invalid field error, but got {:#?}", error),
    }
}
