use pest::Parser;
use std::error::Error;
use std::fs;

mod phase2;
mod phase3;
mod phase4;
mod phase6;

use phase2::*;
use phase3::*;
use phase4::*;
use phase6::*;

pub fn preprocess_file(path: &str) -> Result<String, Box<dyn Error>> {
    let source_content =
        fs::read_to_string(path).unwrap_or_else(|_| panic!("Unable to read source file {}", path));
    preprocess(&source_content)
}

pub fn preprocess(code: &str) -> Result<String, Box<dyn Error>> {
    let code = phase2(code);
    let code = phase3(&code)?;
    Ok(code)
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
