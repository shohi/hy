mod tests {
    use super::super::*;

    #[test]
    fn test_get_data() {
        let url = "http://www.dictionaryapi.com/api/v1/references/collegiate/xml/hello?key=82c5d495-ccf0-4e72-9051-5089e85c2975";
        get_data(url);
    }
}
