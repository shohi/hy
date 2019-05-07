use crate::client::{Item, ItemError};
use termion::{color, style};

// TODO
pub fn render(items: &[Item]) {
    // TODO: traverse item
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
}
