use serde_derive::Deserialize;
use async_trait::async_trait;

pub mod dictionary;
pub mod iciba;
pub mod youdao;

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
    // TODO: more efficient type instead of String?
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

use crate::render;
use crate::say;
use dictionary::Dictionary;
use iciba::Iciba;
use youdao::YouDao;

use log::error;
use futures::{
    FutureExt,
    pin_mut,
    select,
};

fn render_result(res: Result<Item, ItemError>) {
    match res{
        Ok(item) => {
            render::render_item(&item)
        }
        Err(err) => {
            error!("err: {:#?}", err);
        }
    };
}

// TODO: refactor
pub async fn translate(word: &str) {
    let ic_client = Iciba::new();
    let yd_client = YouDao::new();
    let dc_client = Dictionary::new();

    let s = say::say(word).fuse();

    let ic = ic_client.query(word).fuse();
    let yd = yd_client.query(word).fuse();
    let dc = dc_client.query(word).fuse();

    pin_mut!(s, ic, yd, dc);

    loop {
        select! {
            _ = s => {},
            item = ic => render_result(item),
            item = yd => render_result(item),
            item = dc => render_result(item),
            complete => break,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_translate() {
        translate("hello").await;
    }
}
