mod tests {
    use super::super::*;

    #[test]
    fn test_query() {
        let cb = Iciba::new();
        cb.query("hello");
    }
}
