use serde::Deserialize;

use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Graph {
    pub(crate) name: String,
    pub(crate) entry_point: String,
    pub(crate) nodes: HashMap<String, Node>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub(crate) enum Node {
    Standard {
        calculation: Calculation,
        next: HashMap<String, String>,
    },
    Output {
        output: String,
    },
}

#[derive(Deserialize, Debug)]
#[serde(tag = "operation")]
pub(crate) enum Calculation {
    #[serde(rename = "<")]
    LessThan { field: String, value: i32 },
}
