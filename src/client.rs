use reqwest;
use serde_derive::{Deserialize, Serialize};
use termion::{color, style};

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
    api: String,
    en: String,
    us: String,
}

// TODO: refactor
impl Phonetic {
    pub fn dump(&self) -> String {
        format!(
            "{}{}  {}{}  ~  {}{}",
            color::Fg(color::Magenta),
            &self.en,
            self.us,
            color::Fg(color::LightBlack),
            self.api,
            color::Fg(color::Reset),
        )
    }
}

impl Item {
    pub fn dump(&self) {
        // TODO
        let joined = self.acceptations.join("\n");
        println!("{}", joined);
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct TranslatePair {
    pub from: String,
    pub to: String,
}

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

    vec.push(Iciba::new().query(word).unwrap());
    vec.push(YouDao::new().query(word).unwrap());
    vec.push(Dictionary::new().query(word).unwrap());

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
