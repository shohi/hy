use crate::client::{Item, ItemError};
use regex::{self, Regex, RegexBuilder};
use termion::{color, style};

// TODO: implement
pub fn render(items: &[Item]) {
    for m in items.iter() {
        render_item(m);
    }
}

fn render_item(item: &Item) {
    println!();
    println!("{} {}", &item.query, item.phonetic.dump(),);
    println!();

    for s in item.acceptations.iter() {
        println!(
            "{}- {}{}",
            color::Fg(color::LightBlack),
            color::Fg(color::Green),
            &s,
        );
    }
    println!("{}", color::Fg(color::Reset));

    for (i, p) in item.sentences.iter().enumerate() {
        // sequence starts from 1
        print!("{}{}. ", color::Fg(color::LightBlack), i + 1);
        highlight(p.from.clone(), item.query.clone());
        println!("{}{}{}", color::Fg(color::Cyan), " ".repeat(3), &p.to);
    }

    println!("{}\n{}", color::Fg(color::LightBlack), "--".repeat(4));
    println!("{}", color::Fg(color::Reset))
}

// TODO: highlight query works in examples
// and use &str instead
fn highlight(s: String, key: String) {
    let exp = format!("(?P<k>{})", key);
    let re = RegexBuilder::new(exp.as_str())
        .case_insensitive(true)
        .build()
        .expect("invalid regexp");

    let result = re.replace_all(
        s.as_str(),
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

    #[test]
    fn test_termion() {
        println!("{}Red", color::Fg(color::Red));
        println!("{}Blue", color::Fg(color::Blue));
        println!("{}Blue'n'Bold{}", style::Bold, style::Reset);
        println!("{}Just plain italic", style::Italic);
    }

    #[test]
    fn test_render() {
        let vec = Vec::new();
        render(&vec);
    }

    #[test]
    fn test_highlight() {
        let s = "hello world hello".into();
        let key = "world".into();

        highlight(s, key);
    }
}
