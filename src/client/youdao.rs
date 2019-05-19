use super::{Item, ItemError, Phonetic, Query, TranslatePair};
use serde_derive::Deserialize;

pub(super) struct YouDao {
    base_url: &'static str,
    key_from: &'static str,
    key: &'static str,
}

impl YouDao {
    pub fn new() -> YouDao {
        YouDao {
            base_url: "http://fanyi.youdao.com/openapi.do",
            key_from: "node-fanyi",
            key: "110811608",
        }
    }
}

impl Query for YouDao {
    // TODO: add timeout for http request
    fn query(&self, keyword: &str) -> Result<Item, ItemError> {
        let url = format!(
            "{}?keyfrom={}&key={}&type=data&doctype=json&version=1.1&q={}",
            self.base_url, self.key_from, self.key, keyword
        );
        // println!("url: {}", url);

        let dict: Dict = reqwest::get(&url)?.json()?;

        let mut item = Item::default();
        item.query = keyword.into();
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
            api: "fanyi.youdao.com".into(),
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
    basic: Basic,

    #[serde(rename = "web")]
    sentences: Vec<Sentence>,
}

#[derive(Debug, Deserialize)]
struct Basic {
    #[serde(rename = "uk-phonetic", default)]
    phen: String,

    #[serde(rename = "us-phonetic", default)]
    phus: String,

    #[serde(rename = "explains", default)]
    means: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Sentence {
    key: String,

    #[serde(rename = "value")]
    values: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query() {
        let yd = YouDao::new();
        let result = yd.query("hello");
        println!("result -> {:#?}", &result);
    }
}
