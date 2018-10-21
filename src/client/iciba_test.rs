mod tests {
    use super::super::*;

    #[test]
    fn test_get_data() {
        let url = "http://dict-co.iciba.com/api/dictionary.php?key=D191EBD014295E913574E1EAF8E06666&w=hello";
        get_data(url);
    }
}
