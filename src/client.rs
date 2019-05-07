mod dictionary;
mod iciba;
mod youdao;

pub trait Query {
    fn query(&self, keyword: &str) -> Result<Item, ItemError>;
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

    pub fn dump(&self) {
        // TODO
        let joined = self.acceptations.join("\n");
        println!("{}", joined);
    }
}

#[derive(Debug, Clone)]
pub struct ItemError {
    pub message: String,
}

pub fn QueryAll(word: &str) -> Vec<Item> {
    // TODO
    let vec = Vec::new();
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_join() {
        let mut vec = Vec::new();
        vec.push("hello");
        vec.push("world");

        println!("{}", vec.join("-"));
    }
}
