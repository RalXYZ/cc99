use super::*;
use pest::error::ErrorVariant;
use pest::iterators::Pair;
use std::collections::HashSet;
use std::path::Path;

#[derive(Parser)]
#[grammar = "./preprocess/phase4.pest"]
struct Phase4Parser;

pub enum Macro<'a> {
    Empty,
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

pub fn build_group<'a>(
    pair: Pair<'a, Rule>,
    defined: &mut HashMap<String, Macro<'a>>,
    include_dirs: &[&str],
    mut extracting_macro: HashSet<String>,
    code_arena: &'a Arena<String>,
) -> Result<String, Box<dyn Error>> {
    let mut modified = false;
    let mut result = String::new();
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::control_line => {
                modified = true;
                result.push_str(&build_control_line(pair, defined, include_dirs)?);
            }
            Rule::token_string_line => {
                result.push_str(
                    build_token_string_line(pair, defined, &mut extracting_macro, &mut modified)?
                        .as_str(),
                );
            }
            Rule::conditional => {
                modified = true;
                result.push_str(
                    build_conditional(pair, defined, include_dirs, &extracting_macro, code_arena)?
                        .as_str(),
                );
            }
            _ => unreachable!(),
        }
    }
    match modified {
        true => {
            let result = code_arena.alloc(result).as_str();
            let pairs = match Phase4Parser::parse(Rule::cc99, result)?.next() {
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

pub fn build_control_line<'a>(
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

pub fn build_object_like_macro<'a>(
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
                defined.insert(identifier.to_owned(), Macro::Object(token));
                return Ok(());
            }
            _ => unreachable!(),
        }
    }
    defined.insert(identifier.to_owned(), Macro::Empty);
    Ok(())
}

pub fn build_function_like_macro(
    pair: Pair<'_, Rule>,
    defined: &mut HashMap<String, Macro>,
) -> Result<(), Box<dyn Error>> {
    unimplemented!()
}

pub fn build_token_string_line<'a>(
    pair: Pair<'a, Rule>,
    defined: &mut HashMap<String, Macro<'a>>,
    extracting_macro: &mut HashSet<String>,
    modified: &mut bool,
) -> Result<String, Box<dyn Error>> {
    let mut result = String::new();
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::token_string => {
                result.push_str(
                    build_token_string(pair, defined, extracting_macro, modified)?.as_str(),
                );
            }
            Rule::empty_line | Rule::WHITESPACE => {
                result.push_str(pair.as_str());
            }
            _ => unreachable!(),
        }
    }
    result.push('\n');
    Ok(result)
}

pub fn build_token_string<'a>(
    pair: Pair<'a, Rule>,
    defined: &mut HashMap<String, Macro<'a>>,
    extracting_macro: &mut HashSet<String>,
    modified: &mut bool,
) -> Result<String, Box<dyn Error>> {
    let mut result = String::new();
    let mut pair = pair.into_inner();
    while let Some(pair) = pair.next() {
        match pair.as_rule() {
            Rule::WHITESPACE => {
                result.push_str(pair.as_str());
            }
            Rule::token => {
                let token = pair.into_inner().next().unwrap();
                match token.as_rule() {
                    Rule::keyword | Rule::string_literal | Rule::constant | Rule::WHITESPACE => {
                        result.push_str(token.as_str());
                    }
                    Rule::identifier => {
                        if let Some(macro_) = defined.get(token.as_str()) {
                            *modified = true;
                            extracting_macro.insert(token.as_str().to_owned());
                            match macro_ {
                                Macro::Empty => {}
                                Macro::Object(body) => {
                                    result.push_str(body.as_str());
                                }
                                Macro::Function(_, _, _) => {
                                    unimplemented!();
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

pub fn build_conditional<'a>(
    pair: Pair<'a, Rule>,
    defined: &mut HashMap<String, Macro<'a>>,
    include_dirs: &[&str],
    extracting_macro: &HashSet<String>,
    code_arena: &'a Arena<String>,
) -> Result<String, Box<dyn Error>> {
    let mut result = String::new();
    let mut taken = false;
    let mut pair = pair.into_inner();
    while let Some(pair) = pair.next() {
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
                        match token.as_rule() {
                            Rule::constant_expression => {
                                taken = build_constant_expression(token, defined)?;
                            }
                            _ => {}
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

pub fn build_constant_expression(
    pair: Pair<'_, Rule>,
    defined: &HashMap<String, Macro>,
) -> Result<bool, Box<dyn Error>> {
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::identifier => {
                return Ok(defined.contains_key(token.as_str()));
            }
            _ => {}
        }
    }
    unreachable!()
}
