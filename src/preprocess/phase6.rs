use super::*;

#[derive(Parser)]
#[grammar = "./preprocess/phase6.pest"]
struct Phase6Parser;

pub fn phase6(code: &str) -> Result<String, Box<dyn Error>> {
    let pairs = match Phase6Parser::parse(Rule::cc99, code)?.next() {
        Some(p) => p.into_inner(),
        None => unreachable!(),
    };
    let mut result = String::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::sequence_string_literal => {
                result.push('"');
                for literal in pair.into_inner() {
                    if literal.as_rule() == Rule::string_literal {
                        let literal = literal.as_str();
                        result.push_str(&literal[1..literal.len() - 1]);
                    }
                }
                result.push('"');
            }
            _ => result.push_str(pair.as_str()),
        }
    }
    Ok(result)
}
