use reqwest;

use super::{Item, ItemError, Query};

struct Proxy();

impl Query for Proxy {
    fn query(&self, keyword: &str) -> Result<Item, ItemError> {
        let url = format!("http://www.dictionaryapi.com/api/v1/references/collegiate/xml/{}?key=82c5d495-ccf0-4e72-9051-5089e85c2975", keyword);
        let body = reqwest::get(&url).unwrap().text().unwrap();
        println!("body = {:?}", body);

        return Ok(Item::new());
    }
}

#[cfg(test)]
#[path = "./dictionary_test.rs"]
mod dictionary_test;
