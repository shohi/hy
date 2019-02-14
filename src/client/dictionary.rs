use reqwest;

use super::{Item, ItemError, Query};

struct Dictionary {
    base_url: &'static str,
    key: &'static str,
}

impl Dictionary {
    pub fn new() -> Dictionary {
        Dictionary {
            // base_url: "http://www.dictionaryapi.com/api/v1/references/collegiate/xml",
            base_url: "http://www.dictionaryapi.com/api/v3/references/collegiate/json",
            key: "82c5d495-ccf0-4e72-9051-5089e85c2975",
        }
    }
}

impl Query for Dictionary {
    fn query(&self, keyword: &str) -> Result<Item, ItemError> {
        let url = format!("{}/{}?key={}", self.base_url, keyword, self.key);
        println!("url: {}", url);
        let body = reqwest::get(&url).unwrap().text().unwrap();
        println!("body = {:?}", body);

        return Ok(Item::new());
    }
}

#[cfg(test)]
#[path = "./dictionary_test.rs"]
mod dictionary_test;
