use super::*;
use pest::error::ErrorVariant;
use pest::iterators::Pair;
use std::path::Path;

#[derive(Parser)]
#[grammar = "./preprocess/phase4.pest"]
struct Phase4Parser;

pub enum Macro<'a> {
    Object(Pair<'a, Rule>),
    Function(
        /// arguments
        Vec<String>,
        /// is variadic
        bool,
        /// body
        Pair<'a, Rule>,
    ),
}

pub fn phase4(
    code: &str,
    defined: &mut HashMap<String, Macro>,
    include_dirs: &[&str],
) -> Result<String, Box<dyn Error>> {
    let pairs = match Phase4Parser::parse(Rule::cc99, code)?.next() {
        Some(p) => p.into_inner(),
        None => unreachable!(),
    };
    let mut result = String::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::group => {
                for pair in pair.into_inner() {
                    match pair.as_rule() {
                        Rule::control_line => {
                            result.push_str(&build_control_line(pair, defined, include_dirs)?)
                        }
                        Rule::token_string_line => {
                            result.push_str(pair.as_str());
                            // TODO(TO/GA)
                        }
                        Rule::conditional => {
                            result.push_str(pair.as_str());
                            // TODO(TO/GA)
                        }
                        _ => unreachable!(),
                    }
                }
            }
            Rule::WHITESPACE | Rule::EOI => { // preserve indentation
                result.push_str(pair.as_str());
            }
            _ => unreachable!(),
        }
    }
    Ok(result)
}

pub fn build_control_line(
    pair: Pair<'_, Rule>,
    defined: &mut HashMap<String, Macro>,
    include_dirs: &[&str],
) -> Result<String, Box<dyn Error>> {
    let pair = pair.into_inner().next().unwrap();
    let span = pair.as_span();

    let mut search_current_first = false;
    let mut path: Option<&str> = None;

    match pair.as_rule() {
        Rule::function_like_macro => unimplemented!(),
        Rule::object_like_macro => unimplemented!(),
        Rule::current_include => {
            search_current_first = true;
            for token in pair.into_inner() {
                if token.as_rule() == Rule::path_spec {
                    path = Some(token.as_str());
                }
            }
        }
        Rule::standard_include => {
            for token in pair.into_inner() {
                if token.as_rule() == Rule::path_spec {
                    path = Some(token.as_str());
                }
            }
        }
        Rule::line_info => unimplemented!(),
        Rule::undef_macro => {
            for token in pair.into_inner() {
                match token.as_rule() {
                    Rule::identifier => {
                        defined.remove(token.as_str());
                    }
                    _ => unreachable!(),
                }
            }
        }
        Rule::error_macro => {}
        Rule::pragma_macro => unimplemented!(),
        _ => unreachable!(),
    }

    if let Some(path) = path {
        let mut complete_path: Option<String> = None;

        // search current
        if Path::new(path).exists() {
            complete_path = Some(path.to_string());
        } else {
            for &include_dir in include_dirs {
                let tmp_path = format!("{}/{}", include_dir, path);
                if Path::new(&tmp_path).exists() {
                    complete_path = Some(tmp_path);
                    break;
                }
            }
        }

        // search standard
        if !search_current_first {
            // TODO: no standard include file now
        }

        if let Some(path) = complete_path {
            let code = fs::read_to_string(&path)
                .unwrap_or_else(|_| panic!("Unable to read source file {}", path));
            let code = phase2(&code);
            let code = phase3(&code)?;

            let mut defined: HashMap<String, Macro> = Default::default();
            let code = phase4(&code, &mut defined, include_dirs)?;
            return Ok(code);
        } else {
            return Err(Box::new(pest::error::Error::<Rule>::new_from_span(
                ErrorVariant::CustomError {
                    message: "couldn't find such a header file".to_string(),
                },
                span,
            )));
        }
    }

    Ok("".to_string())
}
