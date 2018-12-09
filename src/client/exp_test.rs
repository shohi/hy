mod tests {
    use super::super::*;

    #[test]
    fn test_serde() {
        let src = r#"<Item><name>Banana</name><source>Store</source></Item>"#;
        let should_be = Item {
            name: "Banana".to_string(),
            source: "Store".to_string(),
        };

        // deserialize
        let item: Item = deserialize(src.as_bytes()).unwrap();

        println!("origin: {:?}", should_be);
        println!("data: {:?}", item);

        // serialize
        let mut buffer = Vec::new();
        serialize(&item, &mut buffer).unwrap();

        let ser_str = String::from_utf8(buffer).unwrap();
        println!("serialize: {:?}", ser_str);
    }

    #[test]
    fn it_works() {
        let s = r##"
        <Project name="my_project">
            <Item name="hello" source="world.rs" />
            <Item name="hello1" source="world.rs" />
            <Value name="hello1" source="world.rs" />
        </Project>
        "##;

        let project: Project = deserialize(s.as_bytes()).unwrap();
        println!("{:#?}", project);

        // serialize
        let mut buffer = Vec::new();
        serialize(&project, &mut buffer).unwrap();

        let ser_str = String::from_utf8(buffer).unwrap();
        println!("serialize: {:?}", ser_str);
    }

}
