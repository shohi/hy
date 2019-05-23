use futures::try_ready;
use futures::{Async, Future, Poll};
use log::error;
use reqwest::r#async::Response;
use serde_derive::Deserialize;

pub mod dictionary;
pub mod iciba;
pub mod youdao;

use dictionary::Dictionary;
use iciba::Iciba;
use youdao::YouDao;

pub trait Query {
    fn query(&self, keyword: &str) -> Result<Item, ItemError>;
}

pub trait AsyncQuery {
    fn query_async(&self, keyword: &str) -> Box<Future<Item = Item, Error = ItemError> + Send>;
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

struct ItemFuture<T, U>
where
    U: Parser,
{
    pub response: T,
    pub parser: U,
    pub keyword: String,
}

impl<T, U> Future for ItemFuture<T, U>
where
    T: Future<Item = Response, Error = reqwest::Error>,
    U: Parser,
{
    type Item = Item;
    type Error = ItemError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let mut resp = try_ready!(self.response.poll());
        let mut json_future = resp.json::<U::Item>();
        let d = try_ready!(json_future.poll());

        match self.parser.parse(&d) {
            Ok(mut item) => {
                // TODO: avoid copy
                item.query = self.keyword.clone();
                Ok(Async::Ready(item))
            }
            Err(err) => Err(err),
        }
    }
}

// TODO
pub fn query_future(word: &str) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_join() {
        let vec = query_all("hello");
        println!("{:#?}", vec);
    }
}
