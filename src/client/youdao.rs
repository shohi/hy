use super::{Item, ItemError, Query};

use serde_derive::{Deserialize, Serialize};

struct YouDao {
    base_url: &'static str,
    key_from: &'static str,
    key: &'static str,
}

impl YouDao {
    pub fn new() -> YouDao {
        YouDao {
            base_url: "http://fanyi.youdao.com/openapi.do",
            key_from: "node-fanyi",
            key: "110811608",
        }
    }
}

impl Query for YouDao {
    fn query(&self, keyword: &str) -> Result<Item, ItemError> {
        let url = format!(
            "{}?keyfrom={}&key={}&type=data&doctype=json&version=1.1&q={}",
            self.base_url, self.key_from, self.key, keyword
        );
        println!("url: {}", url);
        let body = reqwest::get(&url).unwrap().text().unwrap();
        println!("body = {:?}", body);
        Ok(Item::new())
    }
}

#[cfg(test)]
#[path = "./youdao_test.rs"]
mod youdao_test;
