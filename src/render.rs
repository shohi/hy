use crate::client::{Item, Phonetic};
use regex::{self, RegexBuilder};
use termion::color;

fn decorate_phonetic(p: &Phonetic) -> String {
    format!(
        "{}{}  {}{}  ~  {}",
        color::Fg(color::Magenta),
        &p.en,
        &p.us,
        color::Fg(color::LightBlack),
        &p.api,
    )
}

fn decorate_acception(c: &str) -> String {
    format!(
        "{}- {}{}",
        color::Fg(color::LightBlack),
        color::Fg(color::Green),
        c,
    )
}

pub fn render_item(item: &Item) {
    println!();
    println!("{} {}", &item.query, decorate_phonetic(&item.phonetic));
    println!("{}", color::Fg(color::Reset));

    for s in item.acceptations.iter() {
        println!("{}", decorate_acception(&s))
    }
    println!("{}", color::Fg(color::Reset));

    for (i, p) in item.sentences.iter().enumerate() {
        // sequence starts from 1
        print!("{}{}. ", color::Fg(color::LightBlack), i + 1);
        highlight(&p.from, &item.query);
        println!("{}{}{}", color::Fg(color::Cyan), " ".repeat(3), &p.to);
    }

    println!("{}\n{}", color::Fg(color::LightBlack), "--".repeat(4));
    println!("{}", color::Fg(color::Reset))
}

// highlight query word in examples
// and use &str instead
fn highlight(s: &str, key: &str) {
    let exp = format!("(?P<k>{})", key);
    let re = RegexBuilder::new(exp.as_str())
        .case_insensitive(true)
        .build()
        .expect("invalid regexp");

    let result = re.replace_all(
        s,
        format!(
            "{}$k{}",
            color::Fg(color::Yellow),
            color::Fg(color::LightBlack)
        )
        .as_str(),
    );

    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    use termion::style;

    #[test]
    fn test_termion() {
        println!("{}Red", color::Fg(color::Red));
        println!("{}Blue", color::Fg(color::Blue));
        println!("{}Blue'n'Bold{}", style::Bold, style::Reset);
        println!("{}Just plain italic", style::Italic);
    }

    #[test]
    fn test_highlight() {
        let s = "hello world hello".into();
        let key = "world".into();

        highlight(s, key);
    }
}
