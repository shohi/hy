use log::error;
use serde_derive::Deserialize;

mod dictionary;
mod iciba;
mod youdao;

use dictionary::Dictionary;
use iciba::Iciba;
use youdao::YouDao;

pub trait Query {
    fn query(&self, keyword: &str) -> Result<Item, ItemError>;
}

pub trait Parser {
    type Item: serde::de::DeserializeOwned;
    fn parse(&self, d: &Self::Item) -> Result<Item, ItemError>;
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

    match Iciba::new().query(word) {
        Ok(item) => {
            vec.push(item);
        }
        Err(err) => {
            error!("err: {:#?}", err);
        }
    };

    match YouDao::new().query(word) {
        Ok(item) => {
            vec.push(item);
        }
        Err(err) => {
            error!("err: {:#?}", err);
        }
    };

    if let Ok(item) = Dictionary::new().query(word) {
        vec.push(item);
    }

    vec
}

pub fn query_future(word: &str) {
    let iciba_future = ItemFuture {
        querier: Iciba::new(),
        keyword: word,
    };

    let youdao_future = ItemFuture {
        querier: YouDao::new(),
        keyword: word,
    };

    let dict_future = ItemFuture {
        querier: Dictionary::new(),
        keyword: word,
    };
}

use futures::{Async, Future, Poll};
pub struct ItemFuture<'a, T: Query> {
    pub querier: T,
    pub keyword: &'a str,
}

impl<'a, T: Query> Future for ItemFuture<'a, T> {
    type Item = Item;
    type Error = ItemError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let item = self.querier.query(self.keyword)?;
        Ok(Async::Ready(item))
    }
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
