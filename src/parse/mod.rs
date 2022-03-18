use combine::{
    attempt,
    error::{ParseError, StreamError},
    parser::char::{char, crlf, string},
    parser::repeat::{repeat_until, many},
    parser::token::{any, eof},
    stream::{position, StreamErrorFor},
    EasyParser, Parser, Stream,
};

fn newline_<Input>() -> impl Parser<Input, Output = char>
where
    Input: Stream<Token = char>,
{
    combine::parser::char::newline().or(crlf())
}

fn fake_newline<Input>() -> impl Parser<Input, Output = char>
where
    Input: Stream<Token = char>,
{
    (char('\\'), newline_()).map(|(_, _)| '\0')
}

fn newline<Input>() -> impl Parser<Input, Output = char>
where
    Input: Stream<Token = char, Position = position::SourcePosition>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    attempt(fake_newline())
        .or(newline_())
        .and_then(|c| match c {
            '\0' => Err(StreamErrorFor::<Input>::expected_static_message(
                "not a logical newline",
            )),
            _ => Ok('\n'),
        })
}

fn trim_fake_newline<Input>() -> impl Parser<Input, Output = String>
where
    Input: Stream<Token = char, Position = position::SourcePosition>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    attempt(fake_newline()).or(any()).map(|c| match c {
        '\0' => String::new(),
        _ => dbg!(c).to_string(),
    })
}

fn c_comment<Input>() -> impl Parser<Input, Output = String>
where
    Input: Stream<Token = char, Position = position::SourcePosition>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (
        string("/*"),
        repeat_until(any(), attempt(eof().map(|_| "EOF").or(string("*/")))),
        eof().map(|_| "EOF").or(string("*/")),
    )
        .map(|(_, comment, exit)| match exit {
            "*/" => comment,
            _ => panic!("unterminated comment"),
        })
}

fn cpp_comment<Input>() -> impl Parser<Input, Output = String>
where
    Input: Stream<Token = char, Position = position::SourcePosition>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (
        string("//"),
        repeat_until(trim_fake_newline(), attempt(newline().or(eof().map(|_| '\0')))),
    )
        .map(|(_, comment)| dbg!(comment))
}

fn comment<Input>() -> impl Parser<Input, Output = String>
where
    Input: Stream<Token = char, Position = position::SourcePosition>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    attempt(cpp_comment()).or(c_comment())
}

fn remove_comment_and_continued_lines<Input>() -> impl Parser<Input, Output = String>
where
    Input: Stream<Token = char, Position = position::SourcePosition>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (
        many((repeat_until(trim_fake_newline(), attempt(comment())), comment()).map(|(code, _)| code))
            .map(|code: Vec<String>| code.into_iter().collect::<String>()),
        many(trim_fake_newline()).map(|code: Vec<String>| code.into_iter().collect::<String>()),
    )
        .map(|(mut code, remaining)| {
            println!("code: {}", code);
            println!("remaining: {}", remaining);
            code.push_str(&remaining);
            code
        })
}

fn preprocessing(code: &str) -> String {
    let code = remove_comment_and_continued_lines()
        .message("preprocess failed")
        .easy_parse(position::Stream::new(code))
        .unwrap_or_else(|e| panic!("{}", e));
    code.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_test() {
        println!(
            "result: {}",
            preprocessing(
                r#"// This is a comment
            return 0;
        "#
            )
        );
    }

    #[test]
    #[should_panic]
    fn process_comments_fail() {
        let code = r#"/* "#;
        println!("result: {}", preprocessing(code));
    }

    #[test]
    fn process_comments() {
        let code = r#"
int main() {
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
    
    
    
    return 0;
}
"#;
        assert_eq!(expected, preprocessing(code));
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
        assert_eq!(expected, preprocessing(code));
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
        assert_eq!(expected, preprocessing(code));
    }
}
