extern crate cc99;

#[cfg(test)]
mod tests {
    use cc99::*;
    use serde::Serialize;
    use std::fs::File;
    use std::io::Read;
    use walkdir::WalkDir;

    #[test]
    fn parse_test_file() {
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

            let mut source_file = File::open(source_path).expect("Unable to open source file!");
            let mut source_content: String = String::new();

            source_file
                .read_to_string(&mut source_content)
                .expect("Unable to read source file!");

            let res = preprocess(&source_content).unwrap();
            let ast = parse(&res).unwrap();
            println!("{}", serde_json::to_string(&ast).unwrap());
            println!(">>> {} <<<", "Finish PreProcess");
        }
    }
}
