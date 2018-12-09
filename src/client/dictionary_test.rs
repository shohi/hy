mod tests {
    use super::super::*;

    #[test]
    fn test_query() {
        let keyword = "hello";
        let p = Dictionary::new();
        p.query(keyword);
    }
}
