use crate::client;
use crate::render;
use crate::say;

pub async fn translate(word: &str) {
    say::say(word);
    let vec = client::query_all(word).await;
    render::render(&vec);
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_translate() {
        translate("hello").await;
    }
}
