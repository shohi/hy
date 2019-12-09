use log::{debug, info};
// TODO: once tokio upgraded to v0.2.0. this should be updated.
use std::time::Duration;
use tokio::prelude::*;
use tokio_net::process::Command;

#[cfg(not(target_os = "macos"))]
pub fn say(_: &str) {
    error!("say command is unimplemented on non-macOS");
}
// https://stackoverflow.com/questions/1366068/
// whats-the-complete-range-for-chinese-characters-in-unicode
const CJK_LOWER: char = '\u{4E00}';
const CJK_UPPER: char = '\u{9FFF}';

fn is_chinese_char(c: char) -> bool {
    CJK_LOWER <= c && c <= CJK_UPPER
}

fn is_chinese(word: &str) -> bool {
    let chars = word.chars();

    let mut init_flag = true;

    for c in chars {
        if init_flag {
            init_flag = false
        }
        if !is_chinese_char(c) {
            return false;
        }
    }

    if init_flag {
        return false;
    }
    println!("true");

    true
}

#[cfg(target_os = "macos")]
pub async fn say(word: &str) {
    let mut cmd = Command::new("say");
    cmd.arg(word);

    if is_chinese(word) {
        cmd.arg("-v").arg("Ting-Ting");
    }

    let status = cmd
        .status()
        .timeout(Duration::from_secs(2))
        .await
        .expect("say command failed to run");

    let status_code = match status {
        Ok(s) => match s.code() {
            Some(c) => c,
            None => 0,
        },
        Err(e) => {
            info!("call say error: {:?}", e);
            -1
        }
    };

    debug!(
        "status code of say command for word [{}]: {:?}",
        word, status_code
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger::Builder;
    use log::LevelFilter;
    use std::sync::Once;
    use tokio;

    static INIT: Once = Once::new();

    fn setup() {
        INIT.call_once(|| {
            Builder::new().filter(None, LevelFilter::Debug).init();
        });
    }

    #[tokio::test]
    async fn test_say() {
        setup();
        say("hello").await;
    }
}
