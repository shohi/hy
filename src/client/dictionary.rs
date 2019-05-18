use super::{Item, ItemError, Phonetic, Query, TranslatePair};
use reqwest::{self, Client};
use serde_derive::Deserialize;
use std::time::Duration;

pub(super) struct Dictionary {
    client: Client,
    base_url: &'static str,
    key: &'static str,
}

impl Dictionary {
    pub fn new() -> Dictionary {
        let client = Client::builder()
            .timeout(Duration::from_secs(30)) // FIXME: configurable?
            .build()
            .unwrap();

        Dictionary {
            client: client,
            // base_url: "http://www.dictionaryapi.com/api/v1/references/collegiate/xml",
            base_url: "http://www.dictionaryapi.com/api/v3/references/collegiate/json",
            key: "82c5d495-ccf0-4e72-9051-5089e85c2975",
        }
    }
}

impl Query for Dictionary {
    fn query(&self, keyword: &str) -> Result<Item, ItemError> {
        let url = format!("{}/{}?key={}", self.base_url, keyword, self.key);
        // println!("{}", url);

        // TODO: check boundary
        let dicts: Vec<Dict> = self.client.get(&url).send()?.json()?;
        if dicts.len() == 0 {
            return Err(ItemError {
                message: "empty content".to_string(),
            });
        }

        let val = &dicts[0];

        let mut item = Item::default();
        item.query = keyword.into();
        item.phonetic = self.phonetic(val);
        item.acceptations = self.acceptation(val);
        item.sentences = self.sentence(val);

        Ok(item)
    }
}

impl Dictionary {
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
