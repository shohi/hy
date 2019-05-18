use crate::client::{Item, ItemError};
use termion::{color, style};

// TODO: implement
pub fn render(items: &[Item]) {
    for m in items.iter() {
        render_item(m);
        println!("{}\n", "--".repeat(4));
    }
}

fn render_item(item: &Item) {
    println!("\n{} {}\n", &item.query, item.phonetic.dump());

    for s in item.acceptations.iter() {
        println!("- {}", &s);
    }
    println!();

    for (i, p) in item.sentences.iter().enumerate() {
        // sequence starts from 1
        println!("{}. {}\n{}{}", i + 1, &p.from, " ".repeat(3), &p.to);
    }
    println!()
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
}
