use super::*;
use escape_string::escape;
use pest::error::ErrorVariant;
use pest::iterators::{Pair, Pairs};
use pest::Span;
use serde::Serialize;
use std::collections::HashSet;
use std::path::Path;

#[derive(Parser)]
#[grammar = "./preprocess/phase4.pest"]
struct Phase4Parser;

pub enum Macro<'a> {
    Object(Option<Pair<'a, Rule>>),
    Function(
        /// arguments
        Vec<String>,
        /// is variadic
        bool,
        /// body
        Option<Vec<Replacement>>,
    ),
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum Replacement {
    Text(String),
    Parameter(String),
    Stringizing(String),
    Concat(String, String),
    VAArg,
}

pub fn phase4<'a>(
    code: &'a str,
    defined: &mut HashMap<String, Macro<'a>>,
    include_dirs: &[&str],
    code_arena: &'a Arena<String>,
) -> Result<String, Box<dyn Error>> {
    let pairs = match Phase4Parser::parse(Rule::cc99, code)?.next() {
        Some(p) => p.into_inner(),
        None => unreachable!(),
    };
    let mut result = String::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::group => {
                result.push_str(
                    build_group(pair, defined, include_dirs, Default::default(), code_arena)?
                        .as_str(),
                );
            }
            Rule::WHITESPACE | Rule::EOI => {
                result.push_str(pair.as_str());
            }
            _ => unreachable!(),
        }
    }
    Ok(result)
}

fn build_group<'a>(
    pair: Pair<'a, Rule>,
    defined: &mut HashMap<String, Macro<'a>>,
    include_dirs: &[&str],
    mut extracting_macro: HashSet<String>,
    code_arena: &'a Arena<String>,
) -> Result<String, Box<dyn Error>> {
    let mut modified = false;
    let mut result = String::new();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::control_line => {
                modified = true;
                result.push_str(&build_control_line(token, defined, include_dirs)?);
            }
            Rule::token_string_line => {
                result.push_str(
                    build_token_string_line(token, defined, &mut extracting_macro, &mut modified)?
                        .as_str(),
                );
            }
            Rule::conditional => {
                modified = true;
                result.push_str(
                    build_conditional(token, defined, include_dirs, &extracting_macro, code_arena)?
                        .as_str(),
                );
            }
            _ => unreachable!(),
        }
    }
    match modified {
        true => {
            let result = code_arena.alloc(result).as_str();
            let pairs = match Phase4Parser::parse(Rule::cc99, result).unwrap().next() {
                Some(p) => p.into_inner(),
                None => unreachable!(),
            };
            let mut result = String::new();
            for pair in pairs {
                match pair.as_rule() {
                    Rule::group => {
                        result.push_str(
                            build_group(
                                pair,
                                defined,
                                include_dirs,
                                extracting_macro.clone(),
                                code_arena,
                            )?
                            .as_str(),
                        );
                    }
                    Rule::WHITESPACE | Rule::EOI => {
                        result.push_str(pair.as_str());
                    }
                    _ => unreachable!(),
                }
            }
            Ok(result)
        }
        false => Ok(result),
    }
}

fn build_control_line<'a>(
    pair: Pair<'a, Rule>,
    defined: &mut HashMap<String, Macro<'a>>,
    include_dirs: &[&str],
) -> Result<String, Box<dyn Error>> {
    let pair = pair.into_inner().next().unwrap();
    let span = pair.as_span();

    let mut search_current_first = false;
    let mut path: Option<&str> = None;

    match pair.as_rule() {
        Rule::function_like_macro => {
            build_function_like_macro(pair, defined)?;
        }
        Rule::object_like_macro => {
            build_object_like_macro(pair, defined)?;
        }
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
        Rule::pragma_macro => {
            return Err(Box::new(pest::error::Error::<Rule>::new_from_span(
                ErrorVariant::CustomError {
                    message: "unsupported pragma macro".to_string(),
                },
                span,
            )));
            // TODO: no pragma macro now
        }
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
            let code_arena = Arena::new();
            let code = phase4(&code, &mut defined, include_dirs, &code_arena)?;
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

fn build_object_like_macro<'a>(
    pair: Pair<'a, Rule>,
    defined: &mut HashMap<String, Macro<'a>>,
) -> Result<(), Box<dyn Error>> {
    let mut identifier: String = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::define__ | Rule::WHITESPACE => {}
            Rule::identifier => {
                identifier = token.as_str().to_string();
            }
            Rule::token_string => {
                defined.insert(identifier, Macro::Object(Some(token)));
                return Ok(());
            }
            _ => unreachable!(),
        }
    }
    defined.insert(identifier, Macro::Object(None));
    Ok(())
}

fn build_function_like_macro<'a>(
    pair: Pair<'a, Rule>,
    defined: &mut HashMap<String, Macro<'a>>,
) -> Result<(), Box<dyn Error>> {
    let mut identifier: Option<String> = None;
    let mut params: Vec<String> = Default::default();
    let mut is_variadic = false;

    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::define__ | Rule::WHITESPACE => {}
            Rule::identifier => {
                if identifier.is_none() {
                    identifier = Some(token.as_str().to_string());
                } else {
                    params.push(token.as_str().to_string());
                }
            }
            Rule::variadic_ => {
                is_variadic = true;
            }
            Rule::token_string => {
                let body = build_function_macro_body(token, &params, is_variadic)?;
                defined.insert(
                    identifier.unwrap(),
                    Macro::Function(params, is_variadic, Some(body)),
                );
                return Ok(());
            }
            _ => unreachable!(),
        }
    }
    defined.insert(
        identifier.unwrap(),
        Macro::Function(params, is_variadic, None),
    );
    Ok(())
}

fn build_function_macro_body(
    pair: Pair<'_, Rule>,
    params: &[String],
    is_variadic: bool,
) -> Result<Vec<Replacement>, Box<dyn Error>> {
    let mut result: Vec<Replacement> = Default::default();
    let mut remaining_text: Option<String> = None;
    for token in pair.into_inner() {
        let token = token.into_inner().next().unwrap();
        match token.as_rule() {
            Rule::macro_expression => {
                if let Some(text) = remaining_text {
                    result.push(Replacement::Text(text));
                    remaining_text = None;
                }
                let token = token.into_inner().next().unwrap();
                let span = token.as_span();
                match token.as_rule() {
                    Rule::token_pasting => {
                        let mut token_iter = token.into_inner();
                        let lhs = token_iter.next().unwrap().as_str();
                        let rhs = token_iter.next().unwrap().as_str();
                        result.push(Replacement::Concat(lhs.to_string(), rhs.to_string()));
                    }
                    Rule::stringizing => {
                        let token = token.into_inner().next().unwrap().as_str().to_string();
                        if token == "__VA_ARGS__" {
                            if is_variadic {
                                result.push(Replacement::VAArg);
                            } else {
                                return Err(Box::new(pest::error::Error::<Rule>::new_from_span(
                                    ErrorVariant::CustomError {
                                        message: "__VA_ARGS__ is not allowed in this macro"
                                            .to_string(),
                                    },
                                    span,
                                )));
                            }
                        } else if params.contains(&token) {
                            result.push(Replacement::Stringizing(token));
                        } else {
                            return Err(Box::new(pest::error::Error::<Rule>::new_from_span(
                                ErrorVariant::CustomError {
                                    message: "unknown parameter".to_string(),
                                },
                                span,
                            )));
                        }
                    }
                    _ => unreachable!(),
                }
            }
            Rule::keyword | Rule::identifier => {
                let token = token.as_str().to_string();
                if params.contains(&token) {
                    if let Some(text) = remaining_text {
                        result.push(Replacement::Text(text));
                        remaining_text = None;
                    }
                    result.push(Replacement::Parameter(token));
                } else if remaining_text.is_some() {
                    remaining_text.as_mut().unwrap().push_str(token.as_str());
                } else {
                    remaining_text = Some(token.as_str().to_string());
                }
            }
            Rule::string_literal | Rule::constant | Rule::punctuator | Rule::WHITESPACE => {
                if remaining_text.is_some() {
                    remaining_text.as_mut().unwrap().push_str(token.as_str());
                } else {
                    remaining_text = Some(token.as_str().to_string());
                }
            }
            _ => unreachable!(),
        }
    }
    if let Some(remaining_text) = remaining_text {
        result.push(Replacement::Text(remaining_text));
    }
    Ok(result)
}

fn build_token_string_line<'a>(
    pair: Pair<'a, Rule>,
    defined: &mut HashMap<String, Macro<'a>>,
    extracting_macro: &mut HashSet<String>,
    modified: &mut bool,
) -> Result<String, Box<dyn Error>> {
    let mut result = String::new();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::token_string => {
                result.push_str(
                    build_token_string(token, defined, extracting_macro, modified)?.as_str(),
                );
            }
            Rule::empty_line | Rule::WHITESPACE => {
                result.push_str(token.as_str());
            }
            _ => unreachable!(),
        }
    }
    result.push('\n');
    Ok(result)
}

fn build_token_string<'a>(
    pair: Pair<'a, Rule>,
    defined: &mut HashMap<String, Macro<'a>>,
    extracting_macro: &mut HashSet<String>,
    modified: &mut bool,
) -> Result<String, Box<dyn Error>> {
    let mut result = String::new();
    let mut token_iter = pair.into_inner();
    while let Some(token) = token_iter.next() {
        match token.as_rule() {
            Rule::WHITESPACE => {
                result.push_str(token.as_str());
            }
            Rule::token => {
                let token = token.into_inner().next().unwrap();
                match token.as_rule() {
                    Rule::keyword | Rule::string_literal | Rule::constant | Rule::WHITESPACE => {
                        result.push_str(token.as_str());
                    }
                    Rule::identifier => {
                        if let Some(macro_) = defined.get(token.as_str()) {
                            *modified = true;
                            extracting_macro.insert(token.as_str().to_owned());
                            match macro_ {
                                Macro::Object(body) => {
                                    if let Some(body) = body {
                                        result.push_str(body.as_str());
                                    }
                                }
                                Macro::Function(params, is_variadic, body) => {
                                    if let Some(body) = body {
                                        result.push_str(
                                            extract_function_like_macro(
                                                &mut token_iter,
                                                token.as_span(),
                                                params,
                                                *is_variadic,
                                                body,
                                            )?
                                            .as_str(),
                                        );
                                    }
                                }
                            }
                        } else {
                            result.push_str(token.as_str());
                        }
                    }
                    Rule::punctuator => {
                        result.push_str(token.as_str());
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
    Ok(result)
}

fn build_conditional<'a>(
    pair: Pair<'a, Rule>,
    defined: &mut HashMap<String, Macro<'a>>,
    include_dirs: &[&str],
    extracting_macro: &HashSet<String>,
    code_arena: &'a Arena<String>,
) -> Result<String, Box<dyn Error>> {
    let mut result = String::new();
    let mut taken = false;
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::if_line => {
                let mut negative_predicate = false;
                for token in pair.into_inner() {
                    match token.as_rule() {
                        Rule::if__ | Rule::ifdef__ => {
                            negative_predicate = false;
                        }
                        Rule::ifndef__ => {
                            negative_predicate = true;
                        }
                        Rule::constant_expression => {
                            taken = negative_predicate ^ build_constant_expression(token, defined)?;
                        }
                        Rule::identifier => {
                            taken = negative_predicate ^ defined.contains_key(token.as_str());
                        }
                        Rule::WHITESPACE => {}
                        _ => unreachable!(),
                    }
                }
            }
            Rule::elif_line => match taken {
                true => {
                    // already taken
                    break;
                }
                false => {
                    for token in pair.into_inner() {
                        if token.as_rule() == Rule::constant_expression {
                            taken = build_constant_expression(token, defined)?;
                        }
                    }
                }
            },
            Rule::else_line => match taken {
                true => {
                    // already taken
                    break;
                }
                false => {
                    taken = true;
                }
            },
            Rule::endif_line => {}
            Rule::group => {
                if taken {
                    result.push_str(
                        build_group(
                            pair,
                            defined,
                            include_dirs,
                            extracting_macro.clone(),
                            code_arena,
                        )?
                        .as_str(),
                    );
                }
            }
            Rule::WHITESPACE => {}
            _ => unreachable!(),
        }
    }
    Ok(result)
}

fn build_constant_expression(
    pair: Pair<'_, Rule>,
    defined: &HashMap<String, Macro>,
) -> Result<bool, Box<dyn Error>> {
    for token in pair.into_inner() {
        if token.as_rule() == Rule::identifier {
            return Ok(defined.contains_key(token.as_str()));
        }
    }
    unreachable!()
}

fn extract_function_like_macro(
    token_iter: &mut Pairs<'_, Rule>,
    span: Span<'_>,
    params: &[String],
    is_variadic: bool,
    body: &[Replacement],
) -> Result<String, Box<dyn Error>> {
    // consume '('
    match token_iter.next() {
        Some(token) => {
            if token.as_str() != "(" {
                return Err(Box::new(pest::error::Error::<Rule>::new_from_span(
                    ErrorVariant::CustomError {
                        message: "expected macro function call".to_string(),
                    },
                    span,
                )));
            }
        }
        None => {
            return Err(Box::new(pest::error::Error::<Rule>::new_from_span(
                ErrorVariant::CustomError {
                    message: "unexpected end of file".to_string(),
                },
                span,
            )));
        }
    }
    // get args
    let mut parenthesis_level = 1;
    let mut args: Vec<String> = vec!["".to_string()];
    loop {
        match token_iter.next() {
            Some(token) => match token.as_str() {
                "(" => {
                    parenthesis_level += 1;
                    args.last_mut().unwrap().push_str(token.as_str());
                }
                ")" => {
                    parenthesis_level -= 1;
                    if parenthesis_level == 0 {
                        break;
                    }
                    args.last_mut().unwrap().push_str(token.as_str());
                }
                "," => {
                    if parenthesis_level != 1 {
                        // old args
                        args.last_mut().unwrap().push_str(token.as_str());
                    } else {
                        // new args
                        args.push("".to_string());
                    }
                }
                " " => {}
                _ => {
                    args.last_mut().unwrap().push_str(token.as_str());
                }
            },
            None => {
                return Err(Box::new(pest::error::Error::<Rule>::new_from_span(
                    ErrorVariant::CustomError {
                        message: "unexpected end of file".to_string(),
                    },
                    span,
                )))
            }
        }
    }
    // replace
    if !(params.is_empty() && args.len() == 1 && args[0].is_empty()) // function without params
        && ((!is_variadic && args.len() != params.len())
            || (is_variadic && args.len() < params.len()))
    {
        return Err(Box::new(pest::error::Error::<Rule>::new_from_span(
            ErrorVariant::CustomError {
                message: "number of arguments mismatch".to_string(),
            },
            span,
        )));
    }
    let mut result = String::new();
    for action in body {
        match action {
            Replacement::Text(text) => {
                result.push_str(text);
            }
            Replacement::Parameter(para_name) => {
                let index = params.iter().position(|r| r == para_name).unwrap();
                result.push_str(args[index].as_str());
            }
            Replacement::Stringizing(para_name) => {
                let index = params.iter().position(|r| r == para_name).unwrap();
                result.push('"');
                result.push_str(&escape(args[index].as_str()));
                result.push('"');
            }
            Replacement::Concat(lhs, rhs) => {
                let lhs = match params.contains(lhs) {
                    true => {
                        let index = params.iter().position(|r| r == lhs).unwrap();
                        escape(args[index].as_str()).to_string()
                    }
                    false => lhs.clone(),
                };
                let rhs = match params.contains(rhs) {
                    true => {
                        let index = params.iter().position(|r| r == rhs).unwrap();
                        escape(args[index].as_str()).to_string()
                    }
                    false => rhs.clone(),
                };
                result.push_str(&format!("{}{}", lhs, rhs));
            }
            Replacement::VAArg => {
                if params.len() == args.len() {
                    break;
                }
                result.push_str(args[params.len()].as_str());
                for arg in args.iter().skip(params.len() + 1) {
                    result.push(',');
                    result.push_str(arg.as_str());
                }
            }
        }
    }
    Ok(result)
}
