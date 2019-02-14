use actix;

use actix::prelude::*;
use actix_web::client;
use clap::{App, Arg};
use futures::future::Future;

use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

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

fn main() {
    setup_logger();

    /*
    let matches = App::new("hy")
        .version("0.1.0")
        .about("command line translation tool implemented in Rust")
        .arg(
            Arg::with_name("WORD")
                .help("set the word to translate")
                .required(true)
                .min_values(1),
        ).get_matches();

    println!("matches: {:?}", matches);
    println!("Hello world");
    */

    /*
    actix::run(|| {
        client::ClientRequest::get("http://www.rust-lang.org") // <- Create request builder
            .header("User-Agent", "Actix-web")
            .finish().unwrap()
            .send()                                    // <- Send http request
            .map_err(|_| ())
            .and_then(|response| {                     // <- server http response
                println!("Response: {:?}", response);
                actix::System::current().stop();
                Ok(())
            })
    });
    */
}
