use chrono::Local;
use std::fmt;

#[derive(Default)]
pub(super) struct Record {
    pub no: u64,
    pub timestamp: i64,
    pub word: String,
}

impl Record {
    pub(super) fn new(no: u64, word: String) -> Self {
        let dt = Local::now();
        Record {
            no,
            timestamp: dt.timestamp(),
            word,
        }
    }
    // record format
    // [no]:[timestamp];[search word]
    pub(super) fn parse(line: &String) -> Self {
        let tokens: Vec<&str> = line.split(|c| c == ':' || c == ';').collect();

        let no = match tokens[0].parse::<u64>() {
            Ok(v) => v,
            Err(_) => 0,
        };

        let mut timestamp: i64 = 0;
        if tokens.len() > 1 {
            match tokens[1].parse::<i64>() {
                Ok(v) => timestamp = v,
                Err(_) => {}
            }
        }

        let mut word = String::new();
        if tokens.len() > 2 {
            word = tokens[2].into()
        }

        Record {
            no,
            timestamp,
            word,
        }
    }

    pub(super) fn to_console_string(&self) -> String {
        format!("{:width$}  {}", self.no, &self.word, width = 3)
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.no, self.timestamp, &self.word)
    }
}
