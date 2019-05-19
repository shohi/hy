use super::{Item, ItemError, Phonetic, Query, TranslatePair};
use serde_derive::Deserialize;

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

impl Query for Iciba {
    // TODO: improve error handling
    fn query(&self, keyword: &str) -> Result<Item, ItemError> {
        let url = format!("{}{}", self.base_url, keyword);
        // println!("url: {}", url);

        let val: Dict = reqwest::get(&url)?.json()?;

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
        let symbol = &dict.base.symbols[0];

        Phonetic {
            api: "iciba.com".into(),
            en: format!("英[ {} ]", &symbol.phen),
            us: format!("美[ {} ]", &symbol.phus),
        }
    }

    fn acceptation(&self, dict: &Dict) -> Vec<String> {
        let mut result = Vec::new();

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

#[derive(Debug, Deserialize)]
struct Dict {
    // NOTE: typo in its API
    #[serde(rename = "baesInfo")]
    base: BaseInfo,

    #[serde(rename = "trade_means", default)]
    trades: Vec<TradeMean>,

    #[serde(rename = "sentence")]
    sentences: Vec<Sentence>,
}

#[derive(Debug, Deserialize)]
struct BaseInfo {
    symbols: Vec<Symbol>,
}

#[derive(Debug, Deserialize)]
struct Symbol {
    #[serde(rename = "ph_en")]
    phen: String,

    #[serde(rename = "ph_am")]
    phus: String,

    parts: Vec<Part>,
}

#[derive(Debug, Deserialize)]
struct Part {
    part: String,
    means: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct TradeMean {
    #[serde(rename = "word_trade")]
    word: String,

    #[serde(rename = "word_mean")]
    mean: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Sentence {
    #[serde(rename = "Network_en")]
    en: String,

    #[serde(rename = "Network_cn")]
    cn: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yyy_query() {
        let cb = Iciba::new();
        let val = cb.query("hello");
        println!("result -> {:#?}", &val);
    }
}
