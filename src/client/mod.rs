mod exp;

mod dictionary;
mod iciba;
mod youdao;

pub trait Query {
    fn query(&self, key: &str) -> Result<Item, ItemError>;
}

pub struct Item {
    pub query: String,
    pub phonetic: Vec<String>,
    pub acceptations: Vec<String>,
    pub sentences: Vec<String>,
}

impl Item {
    pub fn new() -> Self {
        Item {
            query: String::new(),
            phonetic: Vec::new(),
            acceptations: Vec::new(),
            sentences: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ItemError {
    pub message: String,
}
