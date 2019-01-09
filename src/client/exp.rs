// experiments

use serde_derive::{Deserialize, Serialize};
use serde_xml_rs as sd_xml;

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    name: String,
    source: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Value {
    name: String,
    source: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Project {
    pub name: String,

    #[serde(rename = "Item", default)]
    pub items: Vec<Item>,

    #[serde(rename = "Value", default)]
    pub values: Vec<Value>,
}

#[cfg(test)]
#[path = "./exp_test.rs"]
mod exp_test;
