use log::{error, info};
use std::process::Command;
use std::time::Duration;
use wait_timeout::ChildExt;

#[cfg(not(target_os = "macos"))]
pub fn say(_: &str) {
    error!("say command is unimplemented on non-macOS");
}

#[cfg(target_os = "macos")]
pub fn say(word: &str) {
    let mut child = Command::new("say")
        .arg(word)
        .spawn()
        .expect("say command failed to start");

    let one_sec = Duration::from_secs(2);
    let status_code = match child.wait_timeout(one_sec).unwrap() {
        Some(status) => status.code(),
        None => {
            // child hasn't exited yet
            child.kill().unwrap();
            child.wait().unwrap().code()
        }
    };

    info!(
        "status code of say command for word [{}]: {:?}",
        word, status_code
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger::Builder;
    use log::LevelFilter;
    use std::sync::{Once, ONCE_INIT};

    static INIT: Once = ONCE_INIT;

    fn setup() {
        INIT.call_once(|| {
            Builder::new().filter(None, LevelFilter::Debug).init();
        });
    }

    #[test]
    fn test_say() {
        setup();
        say("hello");
    }
}
