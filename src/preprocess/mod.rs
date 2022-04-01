use pest::Parser;
use serde::Serialize;
use std::error::Error;

#[derive(Parser, Serialize)]
#[grammar = "./preprocess/preprocess.pest"]
pub struct PreprocessParser;

pub fn preprocess(code: &str) -> Result<String, Box<dyn Error>> {
    let pairs = match PreprocessParser::parse(Rule::cc99, code)?.next() {
        Some(p) => p.into_inner(),
        None => panic!("Fail to parse an empty file"),
    };
    let mut result = String::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::line_break => {}
            Rule::cpp_comment => result.push('\n'),
            Rule::c_comment => result.push(' '),
            _ => result.push_str(pair.as_str()),
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use walkdir::WalkDir;
    #[test]
    #[should_panic]
    fn process_comments_fail() {
        let code = r#"/* "#;
        println!("result: {}", preprocess(code).unwrap());
    }

    #[test]
    fn process_comments() {
        let code = r#"
int main() {
    '"'; "//";
    // This is a comment
    /* This is a comment */
    /** This is a comment
    //
    */
    return 0;
}
"#;
        let expected = r#"
int main() {
    '"'; "//";
    
     
     
    return 0;
}
"#;
        assert_eq!(expected, preprocess(code).unwrap());
    }

    #[test]
    fn process_continued_lines() {
        let code = r#"
int main() { \
}
"#;
        let expected = r#"
int main() { }
"#;
        assert_eq!(expected, preprocess(code).unwrap());
    }

    #[test]
    fn process_continued_lines_and_comments() {
        let code = r#"
int main() {
    // This is a comment \
    This is a comment, too
    return 0;
}
"#;
        let expected = r#"
int main() {
    
    return 0;
}
"#;
        assert_eq!(expected, preprocess(code).unwrap());
    }

    //如果要看输出，请使用cargo test -- --nocapture 开启输出
    #[test]
    fn process_test_file() {
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

            println!("{}", res);
            println!(">>> {} <<<", "Finish PreProcess");
        }
    }
}
