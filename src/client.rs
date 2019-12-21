use async_trait::async_trait;
use serde_derive::Deserialize;
use std::time::Duration;

pub mod dictionary;
pub mod iciba;
pub mod youdao;

#[async_trait]
pub trait Query {
    async fn query<'a>(&self, keyword: &'a str) -> Result<Item<'a>, ItemError>;
}

pub trait Parser {
    type Item: serde::de::DeserializeOwned;
    fn parse(&self, d: &Self::Item) -> Result<Item, ItemError>;
}

#[derive(Debug, Default)]
pub struct Item<'a> {
    // TODO: use `&str` instead of String
    pub query: &'a str,
    pub phonetic: Phonetic,
    pub acceptations: Vec<String>,
    pub sentences: Vec<TranslatePair>,
}

// TODO: implement format trait
#[derive(Debug, Deserialize, Default)]
pub struct Phonetic {
    pub api: &'static str,
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
    // TODO: use &str instead string
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

use crate::history;
use crate::render;
use crate::say;

use dictionary::Dictionary;
use iciba::Iciba;
use youdao::YouDao;

use futures::{pin_mut, select, FutureExt};
use log::error;

fn render_result(res: Result<Item, ItemError>) {
    match res {
        Ok(item) => render::render_item(&item),
        Err(err) => {
            error!("err: {:#?}", err);
        }
    };
}

// TODO: refactor
pub async fn translate(word: &str, timeout: Duration) {
    history::record_search(word);

    let ic_client = Iciba::new(timeout);
    let yd_client = YouDao::new(timeout);
    let dc_client = Dictionary::new(timeout);

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
        translate("hello", Duration::from_secs(2)).await;
    }
}
