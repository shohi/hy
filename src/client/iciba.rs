extern crate reqwest;
extern crate serde_xml_rs;

use super::{Item, ItemError};
use serde_xml_rs::deserialize;

// TODO: implement deserialize
#[derive(Deserialize, Debug)]
struct Dict {
    key: String,

    #[serde(flatten)]
    prons: Vec<Pronounciation>,

    #[serde(flatten)]
    accepts: Vec<Acceptation>,

    #[serde(flatten)]
    sents: Vec<Sentence>,
}

#[derive(Deserialize, Debug)]
struct Pronounciation {
    ps: String,
    pron: String,
}

#[derive(Deserialize, Debug)]
struct Acceptation {
    pos: String,
    acceptation: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "sent")]
struct Sentence {
    orig: String,
    trans: String,
}

pub fn get_data(url: &str) -> Result<Item, ItemError> {
    let body = reqwest::get(url).unwrap().text().unwrap();

    println!("body = {:?}", body);

    let dict: Dict = deserialize(body.as_bytes()).unwrap();
    println!("dict = {:?}", dict);

    Ok(Item::new())
}

#[cfg(test)]
#[path = "./iciba_test.rs"]
mod iciba_test;
