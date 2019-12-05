use super::{Item, ItemError, Query, TranslatePair, Phonetic};

use serde_derive::Deserialize;
use async_trait::async_trait;

pub(super) struct Iciba {
    base_url: &'static str,
    // key: &'static str,
}
impl Iciba {
    pub fn new() -> Iciba {
        Iciba {
            // base_url: "http://dict-co.iciba.com/api/dictionary.php",
            // key: "D191EBD014295E913574E1EAF8E06666",
            base_url: "http://www.iciba.com/index.php?a=getWordMean&c=search&list=1,8&word=",
        }
    }
}

#[async_trait]
impl Query for Iciba {
    // TODO: improve error handling
    async fn query(&self, keyword: &str) -> Result<Item, ItemError> {
        let url = format!("{}{}", self.base_url, keyword);
        // println!("url: {}", url);

        let resp: String = reqwest::get(&url).await?.text().await?;
        let val: Dict = serde_json::from_str(&resp).unwrap();

        let mut item = Item::default();
        item.query = keyword.into();
        item.phonetic = self.phonetic(&val);
        item.acceptations = self.acceptation(&val);
        item.sentences = self.sentence(&val);

        Ok(item)
    }
}

impl Iciba {
    fn phonetic(&self, dict: &Dict) -> Phonetic {
        // FIXME: check length
        if dict.base.symbols.len() == 0 {
            let mut p = Phonetic::default();
            p.api = "iciba.com".into();
            return p
        }

        let symbol = &dict.base.symbols[0];

        Phonetic {
            api: "iciba.com".into(),
            en: format!("英[ {} ]", &symbol.phen),
            us: format!("美[ {} ]", &symbol.phus),
        }
    }

    fn acceptation(&self, dict: &Dict) -> Vec<String> {
        let mut result = Vec::new();

        if dict.base.symbols.len() == 0 {
            return result
        }

        let parts = &dict.base.symbols[0].parts;

        for p in parts {
            let means = p
                .means
                .iter()
                .fold("".into(), |acc, x| format!("{}{}{}", acc, x, ";")); // TODO: refactor

            result.push(format!("{} {}", &p.part, means));
        }

        result
    }

    fn sentence(&self, dict: &Dict) -> Vec<TranslatePair> {
        let sents = &dict.sentences;

        /* TODO
         * Why not work? dict is borrowed?
         sents
         .into_iter()
         .map(|s| TranslatePair {
         from: s.en,
         to: s.cn,
         })
         .collect()
         */
        sents
            .iter()
            .map(|s| TranslatePair {
                from: s.en.clone(),
                to: s.cn.clone(),
            })
        .collect()
    }
}

// TODO: default for all fields?
#[derive(Default, Debug, Deserialize)]
struct Dict {
    // NOTE: typo in its API
    #[serde(default, rename = "baesInfo")]
    base: BaseInfo,

    #[serde(default, rename = "trade_means")]
    trades: Vec<TradeMean>,

    #[serde(default, rename = "sentence")]
    sentences: Vec<Sentence>,
}

#[derive(Default, Debug, Deserialize)]
struct BaseInfo {
    #[serde(default)]
    symbols: Vec<Symbol>,
}

#[derive(Default, Debug, Deserialize)]
struct Symbol {
    #[serde(default, rename = "ph_en")]
    phen: String,

    #[serde(default, rename = "ph_am")]
    phus: String,

    #[serde(default)]
    parts: Vec<Part>,
}

#[derive(Default, Debug, Deserialize)]
struct Part {
    #[serde(default)]
    part: String,

    #[serde(default)]
    means: Vec<String>,
}

#[derive(Default, Debug, Deserialize)]
struct TradeMean {
    #[serde(default, rename = "word_trade")]
    word: String,

    #[serde(default, rename = "word_mean")]
    mean: Vec<String>,
}

#[derive(Default, Debug, Deserialize)]
struct Sentence {
    #[serde(default, rename = "Network_en")]
    en: String,

    #[serde(default, rename = "Network_cn")]
    cn: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_yyy_query() {
        let cb = Iciba::new();
        let val = cb.query("hello").await;
        println!("result -> {:#?}", &val);
    }
}
