use super::{Item, ItemError, Parser, Phonetic, Query, TranslatePair};

use async_trait::async_trait;
use reqwest::Client;
use serde_derive::Deserialize;
use serde_json;
use std::time::Duration;

pub(super) struct Dictionary {
    // TODO: refactor - extrace common fields
    client: Client,
    base_url: &'static str,
    key: &'static str,
    parser: DictParser,
}

impl Dictionary {
    pub fn new(timeout: Duration) -> Dictionary {
        let client = Client::builder().timeout(timeout).build().unwrap();

        Dictionary {
            client,
            // base_url: "http://www.dictionaryapi.com/api/v1/references/collegiate/xml",
            base_url: "http://www.dictionaryapi.com/api/v3/references/collegiate/json",
            key: "82c5d495-ccf0-4e72-9051-5089e85c2975",
            parser: DictParser,
        }
    }
}

#[async_trait]
impl Query for Dictionary {
    async fn query<'a>(&self, keyword: &'a str) -> Result<Item<'a>, ItemError> {
        let url = format!("{}/{}?key={}", self.base_url, keyword, self.key);
        // println!("{}", url);

        let resp: String = self.client.get(&url).send().await?.text().await?;
        let info: Response = serde_json::from_str(&resp)?;

        match info {
            Response::Details(dicts) => {
                if dicts.len() == 0 {
                    return Err(ItemError {
                        message: "empty content".to_string(),
                    });
                }

                let val = &dicts[0];
                let mut item = Item::default();
                item.query = keyword;
                item.phonetic = self.parser.phonetic(val);
                item.acceptations = self.parser.acceptation(val);
                item.sentences = self.parser.sentence(val);

                Ok(item)
            }
            Response::Candicates(lst) => {
                let mut item = Item::default();
                item.query = keyword.into();
                item.acceptations = self.parser.acceptation_from_candidates(lst);

                Ok(item)
            }
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
            api: "dictionaryapi.com",
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
    fn acceptation_from_candidates(&self, candidates: Vec<String>) -> Vec<String> {
        candidates
    }

    fn sentence(&self, _dict: &Dict) -> Vec<TranslatePair> {
        // NOTE: there is no sentence examples
        Vec::new()
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Response {
    Details(Vec<Dict>),
    Candicates(Vec<String>),
}

#[derive(Default, Debug, Deserialize)]
struct Dict {
    #[serde(default, rename = "fl")]
    part: String,

    #[serde(default, rename = "shortdef")]
    means: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_query() {
        let keyword = "querys";
        let p = Dictionary::new(Duration::from_secs(2));
        let result = p.query(keyword).await;
        println!("result -> {:#?}", &result);
    }
}
