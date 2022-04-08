use super::*;

#[derive(Parser)]
#[grammar = "./preprocess/phase3.pest"]
struct Phase3Parser;

pub fn phase3(code: &str) -> Result<String, Box<dyn Error>> {
    let pairs = match Phase3Parser::parse(Rule::cc99, code)?.next() {
        Some(p) => p.into_inner(),
        None => unreachable!(),
    };
    let mut result = String::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::cpp_comment => result.push('\n'),
            Rule::c_comment => result.push(' '),
            _ => result.push_str(pair.as_str()),
        }
    }
    Ok(result)
}
