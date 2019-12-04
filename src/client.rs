use log::error;
use serde_derive::Deserialize;
use async_trait::async_trait;

pub mod dictionary;
pub mod iciba;
pub mod youdao;

use dictionary::Dictionary;
use iciba::Iciba;
use youdao::YouDao;

#[async_trait]
pub trait Query {
    async fn query(&self, keyword: &str) -> Result<Item, ItemError>;
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
pub async fn query_all(word: &str) -> Vec<Item> {
    let mut vec = Vec::new();

    match Iciba::new().query(word).await {
        Ok(item) => {
            vec.push(item);
        }
        Err(err) => {
            error!("err: {:#?}", err);
        }
    };

    match YouDao::new().query(word).await{
        Ok(item) => {
            vec.push(item);
        }
        Err(err) => {
            error!("err: {:#?}", err);
        }
    };

    if let Ok(item) = Dictionary::new().query(word).await {
        vec.push(item);
    }

    vec
}


#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_string_join() {
        let vec = query_all("hello").await;
        println!("{:#?}", vec);
    }
}
