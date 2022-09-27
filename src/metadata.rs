use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn default_hashmap() -> HashMap<String, String> {
    HashMap::new()
}

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Metadata {
    name: String,
    #[serde(default = "default_hashmap")]
    labels: HashMap<String, String>,
    #[serde(default = "default_hashmap")]
    annotations: HashMap<String, String>
}

impl Metadata {
    pub fn new(name: &str) -> Metadata {
        Metadata{
            name: name.to_string(),
            labels: Default::default(),
            annotations: Default::default()
        }
    }
}