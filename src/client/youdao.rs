use super::{Item, ItemError, Phonetic, Query, TranslatePair};

use async_trait::async_trait;
use reqwest::Client;
use serde_derive::Deserialize;
use serde_json;
use std::time::Duration;

pub(super) struct YouDao {
    client: Client,
    base_url: &'static str,
    key_from: &'static str,
    key: &'static str,
}

impl YouDao {
    pub fn new(timeout: Duration) -> YouDao {
        let client = Client::builder().timeout(timeout).build().unwrap();

        YouDao {
            client,
            base_url: "http://fanyi.youdao.com/openapi.do",
            key_from: "node-fanyi",
            key: "110811608",
        }
    }
}
#[async_trait]
impl Query for YouDao {
    async fn query<'a>(&self, keyword: &'a str) -> Result<Item<'a>, ItemError> {
        let url = format!(
            "{}?keyfrom={}&key={}&type=data&doctype=json&version=1.1&q={}",
            self.base_url, self.key_from, self.key, keyword
        );
        // println!("url: {}", url);

        let resp: String = self.client.get(&url).send().await?.text().await?;
        let dict: Dict = serde_json::from_str(&resp).unwrap();

        let mut item = Item::default();
        item.query = keyword;
        item.phonetic = self.phonetic(&dict);
        item.acceptations = self.acceptation(&dict);
        item.sentences = self.sentence(&dict);

        Ok(item)
    }
}

impl YouDao {
    fn phonetic(&self, dict: &Dict) -> Phonetic {
        let basic = &dict.basic;

        Phonetic {
            api: "fanyi.youdao.com",
            en: format!("英[ {} ]", &basic.phen),
            us: format!("美[ {} ]", &basic.phus),
        }
    }

    fn acceptation(&self, dict: &Dict) -> Vec<String> {
        dict.basic.means.clone()
    }

    fn sentence(&self, dict: &Dict) -> Vec<TranslatePair> {
        let sents = &dict.sentences;
        sents
            .iter()
            .map(|s| TranslatePair {
                from: s.key.clone(),
                to: s.values.join(","),
            })
            .collect()
    }
}

#[derive(Debug, Deserialize)]
struct Dict {
    #[serde(default)]
    basic: Basic,

    #[serde(default, rename = "web")]
    sentences: Vec<Sentence>,
}

#[derive(Default, Debug, Deserialize)]
struct Basic {
    #[serde(default, rename = "uk-phonetic")]
    phen: String,

    #[serde(default, rename = "us-phonetic")]
    phus: String,

    #[serde(default, rename = "explains")]
    means: Vec<String>,
}

#[derive(Default, Debug, Deserialize)]
struct Sentence {
    #[serde(default)]
    key: String,

    #[serde(default, rename = "value")]
    values: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_query() {
        let yd = YouDao::new(Duration::from_secs(2));
        let result = yd.query("hello").await;
        println!("result -> {:#?}", &result);
    }
}
