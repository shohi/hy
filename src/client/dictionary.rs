use super::{Item, ItemError};
use super::{Parser, Query};
use super::{Phonetic, TranslatePair};

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

use futures::try_ready;
use futures::{Async, Future, Poll};

struct XxxFuture<T, U>
where
    U: Parser,
{
    pub response: T,
    pub parser: U,
    pub keyword: String,
}

impl<T, U> Future for XxxFuture<T, U>
where
    T: Future<Item = Response, Error = reqwest::Error>,
    U: Parser,
{
    type Item = Item;
    type Error = ItemError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        // TODO
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

impl Dictionary {
    // FIXME: not work
    fn query2(&self, keyword: &str) -> impl Future<Item = Item, Error = ItemError> {
        let url = format!("{}/{}?key={}", self.base_url, keyword, self.key);

        let f = self.async_client.get(&url).send();

        XxxFuture {
            response: f,
            keyword: keyword.into(),
            parser: DictParser,
        }
        /*
            .and_then(|resp| resp.json::<Vec<Dict>>())
            .map(|d| {
                if d.len() == 0 {
                    return Err(ItemError {
                        message: "empty content".to_string(),
                    });
                }

                let val = &d[0];
                let mut item = Item::default();
                item.query = keyword.into();
                item.phonetic = self.phonetic(val);
                item.acceptations = self.acceptation(val);
                item.sentences = self.sentence(val);

                Ok(item)
            })
            .map_err(|err| Err(err.into()));

        f
        */
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
    fn test_xxx_query() {
        let keyword = "hello";
        let p = Dictionary::new();
        let result = p.query(keyword);
        println!("result -> {:#?}", &result);
    }
}
