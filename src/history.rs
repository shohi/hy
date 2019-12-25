use shellexpand;
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

const HIST_BASEDIR: &'static str = "~/.config/hy";
const HIST_FILENAME: &'static str = "history";

mod record;
use record::{Record, RecordStat};

pub struct History {}

impl History {
    fn open() -> io::Result<File> {
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

    fn max_record_no(f: &File) -> u64 {
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
    pub fn dump() {
        let mut map: HashMap<String, RecordStat> = HashMap::new();

        let f = Self::open();
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

            match map.get_mut(&record.word) {
                Some(v) => {
                    v.update(&record);
                }
                None => {
                    map.insert(record.word.clone(), RecordStat::new(&record));
                }
            };

            println!("{}", record.to_console_string())
        }

        // println!("map ==> {:?}", map);
    }

    // TODO: refactor error handling
    pub fn add(word: &str) {
        let f = Self::open();
        let mut f = match f {
            Ok(file) => file,
            Err(e) => {
                println!("open history file error: {:?}", e);
                return;
            }
        };

        let max_no = Self::max_record_no(&f);
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
        let f = History::open();
        match f {
            Ok(_) => println!("create ok"),
            Err(e) => println!("err: {:?}", e),
        }
    }

    #[test]
    fn test_record_search() {
        let word: &str = "hello";
        History::add(word);
    }
}
