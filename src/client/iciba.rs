use super::{Item, ItemError, Query};

use serde;
use serde_derive::{Deserialize, Serialize};
use serde_xml_rs::deserialize;

struct Iciba {
    base_url: &'static str,
    key: &'static str,
}

impl Iciba {
    pub fn new() -> Iciba {
        Iciba {
            base_url: "http://dict-co.iciba.com/api/dictionary.php",
            key: "D191EBD014295E913574E1EAF8E06666",
        }
    }

    fn get_data(&self, url: &str) -> Result<Item, ItemError> {
        let body = reqwest::get(url).unwrap().text().unwrap();
        println!("body = {:?}", body);
        let dict: Dict = deserialize(body.as_bytes()).unwrap();
        println!("dict = {:?}", dict);
        Ok(Item::new())
    }
}

impl Query for Iciba {
    fn query(&self, keyword: &str) -> Result<Item, ItemError> {
        let url = format!("{}?key={}&w={}", self.base_url, self.key, keyword);
        let body = reqwest::get(&url).unwrap().text().unwrap();
        println!("body = {:?}", body);
        Ok(Item::new())
    }
}

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

#[cfg(test)]
#[path = "./iciba_test.rs"]
mod iciba_test;
