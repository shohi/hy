use serde_derive::Deserialize;

mod dictionary;
mod iciba;
mod youdao;

use dictionary::Dictionary;
use iciba::Iciba;
use youdao::YouDao;

trait Query {
    fn query(&self, keyword: &str) -> Result<Item, ItemError>;
}

#[derive(Debug, Deserialize, Default)]
pub struct Item {
    pub query: String,
    pub phonetic: Phonetic,
    pub acceptations: Vec<String>,
    pub sentences: Vec<TranslatePair>,
}

// TODO: implement format trait
#[derive(Debug, Deserialize, Default)]
pub struct Phonetic {
    pub api: String,
    pub en: String,
    pub us: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct TranslatePair {
    pub from: String,
    pub to: String,
}

// TODO: use `failure` to handle error
#[derive(Debug, Clone)]
pub struct ItemError {
    pub message: String,
}

impl From<reqwest::Error> for ItemError {
    fn from(err: reqwest::Error) -> ItemError {
        ItemError {
            message: format!("{:?}", err),
        }
    }
}

impl From<serde_json::Error> for ItemError {
    fn from(err: serde_json::Error) -> ItemError {
        ItemError {
            message: format!("{:?}", err),
        }
    }
}

// TODO: refactor using generics
pub fn query_all(word: &str) -> Vec<Item> {
    let mut vec = Vec::new();

    if let Ok(item) = Iciba::new().query(word) {
        vec.push(item);
    }

    if let Ok(item) = YouDao::new().query(word) {
        vec.push(item);
    }

    if let Ok(item) = Dictionary::new().query(word) {
        vec.push(item);
    }

    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_join() {
        let vec = query_all("hello");
        println!("{:#?}", vec);
    }
}
