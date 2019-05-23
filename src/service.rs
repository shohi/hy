use futures::{Async, Future, Poll};
use tokio::runtime::Runtime;

use crate::client::{self, dictionary::Dictionary, AsyncQuery};
use crate::render;
use crate::say;

pub fn translate(word: &str) {
    translate_async(word);
}

fn translate_sync(word: &str) {
    say::say(word);
    let vec = client::query_all(word);
    render::render(&vec);
}

fn translate_async(word: &str) {
    let p = Dictionary::new();
    let i = p
        .query_async(word)
        .map(|item| {
            println!("value: {:#?}", item);
            ()
        })
        .map_err(|err| {
            println!("err: {:#?}", err);
            ()
        });

    let mut rt = Runtime::new().unwrap();
    // rt.spawn(i);
    // rt.shutdown_on_idle().wait().unwrap();
    rt.block_on(i);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate() {
        translate("hello");
    }
}
