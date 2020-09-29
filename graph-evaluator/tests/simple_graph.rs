use std::collections::HashMap;
use std::iter::FromIterator;

use graph_evaluator::Graph;

#[test]
fn evaluate_a_simple_graph() {
    let graph = include_str!("data/example_graph.json");
    let graph = Graph::from(graph).unwrap();

    let model: HashMap<String, i32> =
        HashMap::from_iter(vec![(String::from("number_of_bad_things"), 5)]);

    let output = graph.evaluate(&model).unwrap();

    assert_eq!(
        output,
        String::from("The bad things were below the acceptable threshold.")
    );
}
