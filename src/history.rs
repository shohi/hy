use chrono::Local;
use shellexpand;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::io::{BufRead, BufReader};
use std::path::Path;

const HIST_BASEDIR: &'static str = "~/.config/hy";
const HIST_FILENAME: &'static str = "history";

fn get_history_file() -> io::Result<File> {
    let p = shellexpand::tilde(HIST_BASEDIR).into_owned();
    let basedir_path = Path::new(p.as_str()).join("history");

    if !basedir_path.exists() {
        fs::create_dir_all(&p)?;
    }

    let path_buf = Path::new(p.as_str()).join(HIST_FILENAME);
    let file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(path_buf)?;

    Ok(file)
}

fn get_max_history_no(f: &File) -> u64 {
    let file = BufReader::new(f);
    let line = file.lines().last();
    let line = match line {
        Some(s) => s,
        None => return 0,
    };

    let (no, _) = parse_record(line);
    return no;
}

// record format
// [no]:[timestamp]:[search word]
//
fn parse_record(line: io::Result<String>) -> (u64, String) {
    let line = match line {
        Ok(s) => s,
        Err(_) => return (0, "".to_string()),
    };

    let tokens: Vec<&str> = line.split(|c| c == ':' || c == ';').collect();
    if tokens.len() < 1 {
        return (0, "".to_string());
    }

    let max_no = tokens[0].parse::<u64>();
    let max_no = match max_no {
        Ok(v) => v,
        Err(_) => 0,
    };

    let mut word = String::new();
    if tokens.len() > 2 {
        word = tokens[2].into()
    }

    return (max_no, word);
}

// TODO: implement
pub fn show_records() {
    let f = get_history_file();
    let mut f = match f {
        Ok(file) => file,
        Err(e) => {
            println!("open history file error: {:?}", e);
            return;
        }
    };
}

pub fn record_search(word: &str) {
    let f = get_history_file();
    let mut f = match f {
        Ok(file) => file,
        Err(e) => {
            println!("open history file error: {:?}", e);
            return;
        }
    };

    println!("record search");

    let dt = Local::now();
    let max_no = get_max_history_no(&f);
    let record_no = max_no + 1;

    let result = writeln!(&mut f, "{}:{};{}", record_no, dt.timestamp(), word);
    match result {
        Ok(()) => {}
        Err(e) => {
            println!("write history error: {:?}", e);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_file() {
        let f = get_history_file();
        match f {
            Ok(_) => println!("create ok"),
            Err(e) => println!("err: {:?}", e),
        }
    }

    #[test]
    fn test_record_search() {
        let word: &str = "hello";
        record_search(word);
    }
}
