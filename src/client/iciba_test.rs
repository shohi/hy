mod tests {
    use super::super::*;

    #[test]
    fn test_query() {
        let cb = Iciba::new();
        cb.query("hello");
    }

    // FIXME
    #[test]
    fn test_get_data() {
        let cb = Iciba::new();
        let url = format!("{}?key={}&w={}", cb.base_url, cb.key, "hello");
        cb.get_data(&url);
    }
}
