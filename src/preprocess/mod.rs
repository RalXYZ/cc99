use pest::Parser;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use typed_arena::Arena;

mod phase2;
mod phase3;
mod phase4;
mod phase6;

use phase2::*;
use phase3::*;
use phase4::*;
use phase6::*;

pub fn preprocess_file(path: &str, include_dirs: &[&str]) -> Result<String, Box<dyn Error>> {
    let source_content =
        fs::read_to_string(path).unwrap_or_else(|_| panic!("Unable to read source file {}", path));
    preprocess(&source_content, include_dirs)
}

pub fn preprocess(code: &str, include_dirs: &[&str]) -> Result<String, Box<dyn Error>> {
    let code = phase2(code);
    let code = phase3(&code)?;

    let mut defined: HashMap<String, Macro> = Default::default();
    let code_arena = Arena::new();
    let code = phase4(&code, &mut defined, include_dirs, &code_arena)?;

    let code = phase6(&code)?;
    Ok(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn process_comments_fail() {
        let code = r#"/* "#;
        let include_dirs = vec![];
        println!("result: {}", preprocess(code, &include_dirs).unwrap());
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
        let include_dirs = vec![];
        assert_eq!(expected, preprocess(code, &include_dirs).unwrap());
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
        let include_dirs = vec![];
        assert_eq!(expected, preprocess(code, &include_dirs).unwrap());
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
        let include_dirs = vec![];
        assert_eq!(expected, preprocess(code, &include_dirs).unwrap());
    }

    #[test]
    fn combine_adjacent_strings() {
        let code = r#"
int main() {
    char *x = "x" "y""z";
}
"#;
        let expected = r#"
int main() {
    char *x = "xyz";
}
"#;
        let include_dirs = vec![];
        assert_eq!(expected, preprocess(code, &include_dirs).unwrap());
    }

    #[test]
    fn process_object_define() {
        let code = r#"
#define b 0
#define x b
int main() {
    return x;
}
"#;
        let expected = r#"
int main() {
    return 0;
}
"#;
        let include_dirs = vec![];
        assert_eq!(expected, preprocess(code, &include_dirs).unwrap());
    }

    #[test]
    fn process_conditional_macro() {
        let code = r#"
        #define x
        #ifndef x
        #define x 1
        #else
        #if defined(y)
        #elif defined(x)
        #define x 0
        #endif
        #endif
        int main() { return x; }
"#;
        let expected = r#"
                        int main() { return 0; }
"#;
        let include_dirs = vec![];
        assert_eq!(expected, preprocess(code, &include_dirs).unwrap());
    }

    #[test]
    fn process_function_define() {
        let code = r#"
#define x(a) a
#define m_1(a, b) a##b
#define m_2(a) #a
#define m_3(a, ...) #__VA_ARGS__
#define m_4() 0
int main() {
    m_1(i, j);
    m_2(sadf);
    m_3(1, 2, -(3*4, 7), " ,");
    return x(0)m_4();
}
"#;
        let expected = r#"
int main() {
    ij;
    "sadf";
    2,-(3*4,7)," ,";
    return 00;
}
"#;
        let include_dirs = vec![];
        assert_eq!(expected, preprocess(code, &include_dirs).unwrap());
    }
}
