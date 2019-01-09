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
        let item: Item = sd_xml::from_str(src).unwrap();

        println!("origin: {:?}", should_be);
        println!("data: {:?}", item);

        // serialize
        let serialized = sd_xml::to_string(&item).unwrap();
        println!("serialize: {:?}", serialized);
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

        let project: Project = sd_xml::from_str(s).unwrap();
        println!("{:#?}", project);

        // serialize
        let serialized = sd_xml::to_string(&project).unwrap();
        println!("serialize: {:?}", serialized);
    }

}
