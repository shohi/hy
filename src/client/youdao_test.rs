mod tests {
    use super::super::*;

    #[test]
    fn test_query() {
        let yd = YouDao::new();
        yd.query("http://fanyi.youdao.com/openapi.do?keyfrom=node-fanyi&key=110811608&type=data&doctype=json&version=1.1&q=hello");
    }
}
