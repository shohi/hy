use crate::client;
use crate::render;
use crate::say;

pub fn translate(word: &str) {
    say::say(word);
    let vec = client::query_all(word);
    render::render(&vec);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate() {
        translate("hello");
    }
}
