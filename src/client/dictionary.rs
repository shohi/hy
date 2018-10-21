extern crate reqwest;

use super::{Item, ItemError};

pub fn get_data(url: &str) -> Result<Item, ItemError> {
    let body = reqwest::get(url).unwrap().text().unwrap();

    println!("body = {:?}", body);
    Ok(Item::new())
}

#[cfg(test)]
#[path = "./dictionary_test.rs"]
mod dictionary_test;
