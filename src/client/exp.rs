// experiments
extern crate serde_xml_rs;

use serde_xml_rs::{deserialize, serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Item {
    name: String,
    source: String,
}

fn marshall() {
    let src = r#"<Item><name>Banana</name><source>Store</source></Item>"#;
    let should_be = Item {
        name: "Banana".to_string(),
        source: "Store".to_string(),
    };

    // deserialize
    let item: Item = deserialize(src.as_bytes()).unwrap();

    println!("origin: {:?}", should_be);
    println!("data: {:?}", item);

    // serialize
    let mut buffer = Vec::new();
    serialize(&item, &mut buffer).unwrap();

    let ser_str = String::from_utf8(buffer).unwrap();
    println!("serialize: {:?}", ser_str);
}

#[cfg(test)]
#[path = "./exp_test.rs"]
mod exp_test;
