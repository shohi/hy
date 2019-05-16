use super::{Item, ItemError, Query};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

struct Iciba {
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
    fn query(&self, keyword: &str) -> Result<Item, ItemError> {
        let url = format!("{}{}", self.base_url, keyword);
        println!("url: {}", url);

        let mut resp = reqwest::get(&url)?;
        let body = resp.text()?;
        let val: Dict = serde_json::from_str(&body)?;

        println!("hello, {:#?}", &val);
        let mut item = Item::new();
        item.query = keyword.into();

        Ok(item)
    }
}

#[derive(Debug, Deserialize)]
struct Dict {
    // NOTE: typo in its API
    #[serde(rename = "baesInfo")]
    base: BaseInfo,

    #[serde(rename = "trade_means")]
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
    pham: String,

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
