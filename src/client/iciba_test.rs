mod tests {
    use super::super::*;

    #[test]
    fn test_query() {
        let cb = Iciba::new();
        cb.query("hello");
    }

    // FIXME
    #[test]
    #[ignore]
    fn test_serialize() {
        // let dt = Dict::new();

        let mut dt = Vec::<Sentence>::new();
        dt.push(Sentence {
            orig: "orig".to_string(),
            trans: "trans".to_string(),
        });

        // serialize
        let serialized = sd_xml::to_string(&dt).unwrap();
        println!("serialize: {:?}", serialized);
    }

    // FIXME
    #[test]
    #[ignore]
    fn test_get_data() {
        let cb = Iciba::new();
        let url = format!("{}?key={}&w={}", cb.base_url, cb.key, "hello");
        println!("url = {}", url);
        cb.get_data(&url);
    }
}
