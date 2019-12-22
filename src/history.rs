use shellexpand;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

const HIST_BASEDIR: &'static str = "~/.config/hy";
const HIST_FILENAME: &'static str = "history";

mod record;
use record::Record;

pub struct History {}

impl History {
    fn open_history_file() -> io::Result<File> {
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

        let line = match line {
            Ok(s) => s,
            Err(_) => return 0,
        };

        let record = Record::parse(&line);
        return record.no;
    }

    // TODO: update error handling
    pub fn show_records() {
        let f = Self::open_history_file();
        let f = match f {
            Ok(file) => file,
            Err(e) => {
                println!("open history file error: {:?}", e);
                return;
            }
        };

        let f = BufReader::new(f);
        for line in f.lines() {
            let line = match line {
                Ok(v) => v,
                Err(_) => continue,
            };

            let record = Record::parse(&line);
            println!("{}", record.to_console_string())
        }
    }

    // TODO: refactor error handling
    pub fn record_search(word: &str) {
        let f = Self::open_history_file();
        let mut f = match f {
            Ok(file) => file,
            Err(e) => {
                println!("open history file error: {:?}", e);
                return;
            }
        };

        let max_no = Self::get_max_history_no(&f);
        let record = Record::new(max_no + 1, word.to_string());
        let result = writeln!(&mut f, "{}", record);

        match result {
            Ok(()) => {}
            Err(e) => {
                println!("write history error: {:?}", e);
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_file() {
        let f = History::open_history_file();
        match f {
            Ok(_) => println!("create ok"),
            Err(e) => println!("err: {:?}", e),
        }
    }

    #[test]
    fn test_record_search() {
        let word: &str = "hello";
        History::record_search(word);
    }
}
