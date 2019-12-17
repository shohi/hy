#[cfg(test)]
mod tests {
    use shellexpand;
    use std::path::Path;
    use std::{fs, fs::File};

    #[test]
    fn test_create_file() {
        let p = shellexpand::tilde("~/.config/hy").into_owned();
        println!("base dir: {:?}", &p);

        let path_buf = Path::new(p.as_str()).join("history");
        let hist_path = path_buf.as_path();

        println!("path: {:?} exist-{:?}", path_buf, hist_path.exists());

        if hist_path.exists() {
            return;
        }

        let result = fs::create_dir_all(&p);
        match result {
            Ok(()) => println!("create successfully"),
            Err(e) => println!("err: {:?}", e),
        }

        let f = File::create(hist_path);
        match f {
            Ok(_) => println!("create ok"),
            Err(e) => println!("err: {:?}", e),
        }
    }
}
