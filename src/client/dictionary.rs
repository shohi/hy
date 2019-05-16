use super::{Item, ItemError, Query};
use serde_json::{Result as JsonResult, Value};
use std::str::FromStr;

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
        println!("{}", url);

        let mut resp = reqwest::get(&url)?;
        let body = resp.text()?;
        let val: Value = serde_json::from_str(&body)?;

        let mut item = Item::new();
        item.query = keyword.to_string();

        return Ok(item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xxx_query() {
        let keyword = "hello";
        let p = Dictionary::new();
        p.query(keyword);
    }
}
