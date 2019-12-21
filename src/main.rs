use clap::{crate_version, App, AppSettings, Arg, SubCommand};

use chrono::Local;
use env_logger::Builder;
use humantime;
use log::LevelFilter;
use std::io::Write;
use tokio;

mod client;
mod history;
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

fn history_cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("history")
        .about("show history")
        .arg(
            // TODO: implement
            Arg::with_name("time")
                .long("time")
                .help("sort by time")
                .takes_value(true)
                .global(false)
                .default_value("true"),
        )
        .arg(
            // TODO: implement
            Arg::with_name("frequency")
                .long("fequency")
                .help("sort by frequency")
                .takes_value(true)
                .global(false)
                .default_value("false"),
        )
}

fn show_history() {
    history::show_records()
}

#[tokio::main]
async fn main() {
    setup_logger();

    let matches = App::new("hy")
        .settings(&[
            AppSettings::SubcommandsNegateReqs,
            AppSettings::ArgsNegateSubcommands,
        ])
        .version(crate_version!())
        .about("command line translation tool implemented in Rust")
        .arg(
            Arg::with_name("WORD")
                .help("set the word to translate")
                .takes_value(true)
                .global(false)
                .required(true),
        )
        .arg(
            Arg::with_name("timeout")
                .long("timeout")
                .help("timeout for http request")
                .takes_value(true)
                .global(false)
                .default_value("2s"),
        )
        // TODO: fix subcommand
        // After adding `history` subcommand, `hy` can not recoganize works started by
        // `he`, e.g. `hy hello` returns error.
        .subcommand(history_cmd())
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("history") {
        show_history();
        return;
    }

    let word = matches.value_of("WORD").unwrap();
    let timeout_str = matches.value_of("timeout").unwrap();
    let timeout = humantime::parse_duration(timeout_str).unwrap();
    client::translate(word, timeout).await;
}
