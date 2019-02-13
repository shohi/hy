use std::process::Command;
use std::time::Duration;
use wait_timeout::ChildExt;

#[cfg(not(target_os = "macos"))]
pub fn say(_: &str) {
    // TODO: add log - unimplemented!
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

    // TODO: use log instead
    println!("status code: {:?}", status_code);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_say() {
        say("hello");
    }
}
