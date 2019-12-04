use clap::{App, Arg};

use chrono::Local;
use env_logger::Builder;
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
        .version("0.1.0")
        .about("command line translation tool implemented in Rust")
        .arg(
            Arg::with_name("WORD")
                .help("set the word to translate")
                .takes_value(true)
                .required(true)
                .index(1),
        )
        .get_matches();

    let word = matches.value_of("WORD").unwrap();
    service::translate(word).await;
}
