use clap::{App, Arg};

use chrono::Local;
use env_logger::Builder;
use humantime;
use log::LevelFilter;
use std::io::Write;
use tokio;

mod client;
mod render;
mod say;
mod service;

fn setup_logger() {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
}

#[tokio::main]
async fn main() {
    setup_logger();

    let matches = App::new("hy")
        .version("0.2.2")
        .about("command line translation tool implemented in Rust")
        .arg(
            Arg::with_name("WORD")
                .help("set the word to translate")
                .takes_value(true)
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("timeout")
                .long("timeout")
                .help("timeout for http request")
                .takes_value(true)
                .default_value("2s"),
        )
        .get_matches();

    let word = matches.value_of("WORD").unwrap();
    let timeout_str = matches.value_of("timeout").unwrap();
    let timeout = humantime::parse_duration(timeout_str).unwrap();
    client::translate(word, timeout).await;
}
