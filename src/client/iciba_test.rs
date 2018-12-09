mod tests {
    use super::super::*;
    use serde_xml_rs::serialize;

    #[test]
    fn test_query() {
        let cb = Iciba::new();
        cb.query("hello");
    }

    // FIXME
    #[test]
    fn test_serialize() {
        // let dt = Dict::new();

        let mut dt = Vec::<Sentence>::new();
        dt.push(Sentence {
            orig: "orig".to_string(),
            trans: "trans".to_string(),
        });

        // serialize
        let mut buffer = Vec::new();
        serialize(&dt, &mut buffer).unwrap();

        let ser_str = String::from_utf8(buffer).unwrap();
        println!("serialize: {:?}", ser_str);
    }

    // FIXME
    #[test]
    fn test_get_data() {
        let cb = Iciba::new();
        let url = format!("{}?key={}&w={}", cb.base_url, cb.key, "hello");
        println!("url = {}", url);
        cb.get_data(&url);
    }
}
