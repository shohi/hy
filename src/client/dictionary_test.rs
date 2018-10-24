mod tests {
    use super::super::*;

    #[test]
    fn test_get_data() {
        let keyword = "hello";
        let p = Proxy();
        p.query(keyword);
    }
}
