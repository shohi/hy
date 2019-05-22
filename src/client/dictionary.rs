use super::{Item, ItemError, Query};
use super::{ItemFuture, Parser};
use super::{Phonetic, TranslatePair};

use futures::try_ready;
use futures::{Async, Future, Poll};
use reqwest::{self, r#async::Client as AsyncClient, r#async::Response, Client};
use serde_derive::Deserialize;
use std::time::Duration;

pub(super) struct Dictionary {
    client: Client,
    async_client: AsyncClient,
    base_url: &'static str,
    key: &'static str,
    parser: DictParser,
}

impl Dictionary {
    pub fn new() -> Dictionary {
        let client = Client::builder()
            .timeout(Duration::from_secs(30)) // FIXME: configurable?
            .build()
            .unwrap();
        let async_client = AsyncClient::builder()
            .timeout(Duration::from_secs(30)) // FIXME: configurable?
            .build()
            .unwrap();

        Dictionary {
            client: client,
            async_client: async_client,
            // base_url: "http://www.dictionaryapi.com/api/v1/references/collegiate/xml",
            base_url: "http://www.dictionaryapi.com/api/v3/references/collegiate/json",
            key: "82c5d495-ccf0-4e72-9051-5089e85c2975",
            parser: DictParser,
        }
    }
}

impl Query for Dictionary {
    fn query(&self, keyword: &str) -> Result<Item, ItemError> {
        let url = format!("{}/{}?key={}", self.base_url, keyword, self.key);
        // println!("{}", url);

        let dicts: Vec<Dict> = self.client.get(&url).send()?.json()?;
        if dicts.len() == 0 {
            return Err(ItemError {
                message: "empty content".to_string(),
            });
        }

        let val = &dicts[0];
        let mut item = Item::default();
        item.query = keyword.into();
        item.phonetic = self.parser.phonetic(val);
        item.acceptations = self.parser.acceptation(val);
        item.sentences = self.parser.sentence(val);

        Ok(item)
    }
}
impl Dictionary {
    // FIXME: not work
    fn query_async(&self, keyword: &str) -> impl Future<Item = Item, Error = ItemError> {
        let url = format!("{}/{}?key={}", self.base_url, keyword, self.key);

        let f = self.async_client.get(&url).send();

        ItemFuture {
            response: f,
            keyword: keyword.into(),
            parser: DictParser,
        }
    }
}

struct DictParser;

impl Parser for DictParser {
    type Item = Vec<Dict>;

    fn parse(&self, d: &Self::Item) -> Result<Item, ItemError> {
        if d.len() == 0 {
            return Err(ItemError {
                message: "empty content".to_string(),
            });
        }

        let val = &d[0];
        let mut item = Item::default();
        item.phonetic = self.phonetic(val);
        item.acceptations = self.acceptation(val);
        item.sentences = self.sentence(val);

        return Ok(item);
    }
}

impl DictParser {
    fn phonetic(&self, _dict: &Dict) -> Phonetic {
        // NOTE: there is no phonetics
        Phonetic {
            api: "dictionaryapi.com".into(),
            en: "".into(),
            us: "".into(),
        }
    }

    fn acceptation(&self, dict: &Dict) -> Vec<String> {
        let means = &dict.means;

        means
            .iter()
            .map(|s| format!("{}. {}", &dict.part, s))
            .collect()
    }

    fn sentence(&self, _dict: &Dict) -> Vec<TranslatePair> {
        // NOTE: there is no sentence examples
        Vec::new()
    }
}

struct Display<T>(T);
impl<T> Future for Display<T>
where
    T: Future,
    T::Item: std::fmt::Debug,
    T::Error: std::fmt::Debug,
{
    type Item = ();
    type Error = ();
    fn poll(&mut self) -> Poll<(), ()> {
        match self.0.poll() {
            Ok(Async::Ready(value)) => {
                println!("value: {:#?}", value);
            }
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Err(err) => {
                println!("err: {:#?}", err);
            }
        }

        Ok(().into())
    }
}

#[derive(Debug, Deserialize)]
struct Dict {
    #[serde(default, rename = "fl")]
    part: String,

    #[serde(rename = "shortdef")]
    means: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_query() {
        let keyword = "hello";
        let p = Dictionary::new();
        let result = p.query(keyword);
        println!("result -> {:#?}", &result);
    }

    #[test]
    // FIXME: not work
    fn test_async_query() {
        let keyword = "hello";
        let p = Dictionary::new();

        let i = p.query_async(keyword);
        let f = Display(i);
        tokio::run(f);
    }
}
