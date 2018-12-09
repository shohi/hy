use super::{Item, ItemError, Query};

use serde_derive::{Deserialize, Serialize};

struct YouDao {
    base_url: String,
}

impl YouDao {
    pub fn new() -> YouDao {
        YouDao {
            base_url: String::from("http://fanyi.youdao.com/openapi.do?keyfrom=node-fanyi&key=110811608&type=data&doctype=json&version=1.1&q=%s"),
        }
    }
}

impl Query for YouDao {
    fn query(&self, url: &str) -> Result<Item, ItemError> {
        let body = reqwest::get(url).unwrap().text().unwrap();
        println!("body = {:?}", body);
        Ok(Item::new())
    }
}

#[cfg(test)]
#[path = "./youdao_test.rs"]
mod youdao_test;
