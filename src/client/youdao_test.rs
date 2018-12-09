mod tests {
    use super::super::*;

    #[test]
    fn test_query() {
        let yd = YouDao::new();
        yd.query("hello");
    }
}
