extern crate cc99;

#[cfg(test)]
mod tests {
    use cc99::*;
    use walkdir::WalkDir;

    #[test]
    fn parse_test_file() {
        let include_dirs = vec![];
        for entry in WalkDir::new("./tests")
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir())
        {
            let raw_path = entry.path().to_str();
            if raw_path.is_none() {
                continue;
            }

            let source_path = raw_path.unwrap();
            if !source_path.ends_with(".c") {
                continue;
            }
            println!(">>> {} {} <<<", "Start compiling", source_path);

            let res = preprocess_file(&source_path, &include_dirs).unwrap();
            let ast = parse(&res).unwrap();
            println!("{}", serde_json::to_string(&ast).unwrap());
            println!(">>> {} <<<", "Finish Parsing");
        }
    }
}
