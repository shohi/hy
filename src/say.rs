use log::{debug, info};
// TODO: once tokio upgraded to v0.2.0. this should be updated.
use tokio_net::process::Command;
use tokio::prelude::*;
use std::time::Duration;

#[cfg(not(target_os = "macos"))]
pub fn say(_: &str) {
    error!("say command is unimplemented on non-macOS");
}

#[cfg(target_os = "macos")]
pub async fn say(word: &str) {
    let mut cmd = Command::new("say");

    let status = cmd
        .arg(word)
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
        },
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
