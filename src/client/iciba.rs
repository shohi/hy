use super::{Item, ItemError, Query};

use serde_derive::{Deserialize, Serialize};

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

        let body = reqwest::get(&url).unwrap().text().unwrap();
        println!("body = {:?}", body);
        let mut item = Item::new();
        item.query = keyword.into();

        Ok(item)
    }
}

// TODO: implement deserialize
#[derive(Debug, Deserialize)]
struct Dict {
    key: String,
    prons: Vec<Pronounciation>,
    accepts: Vec<Acceptation>,
    sents: Vec<Sentence>,
}

impl Dict {
    fn new() -> Dict {
        Dict {
            key: String::new(),
            prons: Vec::<Pronounciation>::new(),
            accepts: Vec::<Acceptation>::new(),
            sents: Vec::<Sentence>::new(),
        }
    }
}

// TODO: implemnt deserialize
#[derive(Debug, Deserialize)]
struct Pronounciation {
    ps: String,
    pron: String,
}

// TODO: implemnt deserialize
#[derive(Debug, Deserialize)]
struct Acceptation {
    pos: String,
    acceptation: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename = "sent")]
struct Sentence {
    orig: String,
    trans: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query() {
        let cb = Iciba::new();
        cb.query("hello");
    }
}
