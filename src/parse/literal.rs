use pest::error::ErrorVariant;
use pest::iterators::Pair;

use super::*;

pub fn build_string_literal(pair: Pair<'_, Rule>) -> Result<Expression, Box<dyn Error>> {
    let mut string_literal: String = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::char_no_escape => {
                string_literal.push_str(token.as_str());
            }
            Rule::escape_sequence => {
                string_literal.push(build_escape_sequence(token)?);
            }
            _ => unreachable!(),
        }
    }
    Ok(Expression::StringLiteral(string_literal))
}

pub fn build_escape_sequence(pair: Pair<'_, Rule>) -> Result<char, Box<dyn Error>> {
    let escape_sequence = pair.as_str();
    Ok(match escape_sequence {
        "\\'" => '\'',
        "\\\"" => '\"',
        "\\?" => '?',
        "\\\\" => '\\',
        "\\a" => '\x07',
        "\\b" => '\x08',
        "\\f" => '\x0c',
        "\\n" => '\n',
        "\\r" => '\r',
        "\\t" => '\t',
        "\\v" => '\x0b',
        _ => {
            if escape_sequence == "\\0" {
                return Ok('\0');
            }
            unimplemented!();
        }
    })
}

pub fn build_constant(pair: Pair<'_, Rule>) -> Result<Expression, Box<dyn Error>> {
    let token = pair.into_inner().next().unwrap();
    match token.as_rule() {
        Rule::integer_constant => build_integer_constant(token),
        Rule::character_constant => build_character_constant(token),
        Rule::floating_constant => build_floating_constant(token),
        _ => unreachable!(),
    }
}

pub fn build_integer_constant(pair: Pair<'_, Rule>) -> Result<Expression, Box<dyn Error>> {
    let span = pair.as_span();
    let mut is_decimal_base = false;
    let mut number: i128 = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::decimal_constant => {
                is_decimal_base = true;
                number = match token.as_str().to_string().parse::<i128>() {
                    Ok(number) => number,
                    Err(_) => {
                        return Err(Box::new(pest::error::Error::<Rule>::new_from_span(
                            ErrorVariant::CustomError {
                                message: "integer constant overflow".to_string(),
                            },
                            span,
                        )));
                    }
                };
            }
            Rule::octal_constant => {
                let number_str = token.as_str();
                number = match number_str.len() {
                    0 => unreachable!(),
                    1 => 0,
                    _ => match i128::from_str_radix(&number_str[1..number_str.len()], 8) {
                        Ok(number) => number,
                        Err(_) => {
                            return Err(Box::new(pest::error::Error::<Rule>::new_from_span(
                                ErrorVariant::CustomError {
                                    message: "integer constant overflow".to_string(),
                                },
                                span,
                            )));
                        }
                    },
                }
            }
            Rule::hex_constant => {
                let number_str = token.as_str();
                number = match i128::from_str_radix(&number_str[2..number_str.len()], 16) {
                    Ok(number) => number,
                    Err(_) => {
                        return Err(Box::new(pest::error::Error::<Rule>::new_from_span(
                            ErrorVariant::CustomError {
                                message: "integer constant overflow".to_string(),
                            },
                            span,
                        )));
                    }
                };
            }
            Rule::binary_constant => {
                let number_str = token.as_str();
                number = match i128::from_str_radix(&number_str[2..number_str.len()], 2) {
                    Ok(number) => number,
                    Err(_) => {
                        return Err(Box::new(pest::error::Error::<Rule>::new_from_span(
                            ErrorVariant::CustomError {
                                message: "integer constant overflow".to_string(),
                            },
                            span,
                        )));
                    }
                };
            }
            Rule::integer_suffix => match token.into_inner().next().unwrap().as_rule() {
                Rule::ull_ => match number.try_into() {
                    Ok(number) => {
                        return Ok(Expression::UnsignedLongLongConstant(number));
                    }
                    Err(_) => {
                        return Err(Box::new(pest::error::Error::<Rule>::new_from_span(
                            ErrorVariant::CustomError {
                                message: "integer constant overflow".to_string(),
                            },
                            span,
                        )));
                    }
                },
                Rule::ll_ => {
                    if let Ok(number) = number.try_into() {
                        return Ok(Expression::LongLongConstant(number));
                    }
                    if !is_decimal_base {
                        if let Ok(number) = number.try_into() {
                            return Ok(Expression::UnsignedLongLongConstant(number));
                        }
                    }
                    return Err(Box::new(pest::error::Error::<Rule>::new_from_span(
                        ErrorVariant::CustomError {
                            message: "integer constant overflow".to_string(),
                        },
                        span,
                    )));
                }
                Rule::ul_ => {
                    if let Ok(number) = number.try_into() {
                        return Ok(Expression::UnsignedLongConstant(number));
                    }
                    return Err(Box::new(pest::error::Error::<Rule>::new_from_span(
                        ErrorVariant::CustomError {
                            message: "integer constant overflow".to_string(),
                        },
                        span,
                    )));
                }
                Rule::l_ => {
                    if let Ok(number) = number.try_into() {
                        return Ok(Expression::LongConstant(number));
                    }
                    if !is_decimal_base {
                        if let Ok(number) = number.try_into() {
                            return Ok(Expression::UnsignedLongConstant(number));
                        }
                    }
                    return Err(Box::new(pest::error::Error::<Rule>::new_from_span(
                        ErrorVariant::CustomError {
                            message: "integer constant overflow".to_string(),
                        },
                        span,
                    )));
                }
                Rule::u_ => {
                    if let Ok(number) = number.try_into() {
                        return Ok(Expression::UnsignedIntegerConstant(number));
                    }
                    if let Ok(number) = number.try_into() {
                        return Ok(Expression::UnsignedLongConstant(number));
                    }
                    return Err(Box::new(pest::error::Error::<Rule>::new_from_span(
                        ErrorVariant::CustomError {
                            message: "integer constant overflow".to_string(),
                        },
                        span,
                    )));
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    if let Ok(number) = number.try_into() {
        return Ok(Expression::IntegerConstant(number));
    }
    if !is_decimal_base {
        if let Ok(number) = number.try_into() {
            return Ok(Expression::UnsignedIntegerConstant(number));
        }
    }
    if let Ok(number) = number.try_into() {
        return Ok(Expression::LongConstant(number));
    }
    if !is_decimal_base {
        if let Ok(number) = number.try_into() {
            return Ok(Expression::UnsignedLongLongConstant(number));
        }
    }
    return Err(Box::new(pest::error::Error::<Rule>::new_from_span(
        ErrorVariant::CustomError {
            message: "integer constant overflow".to_string(),
        },
        span,
    )));
}

pub fn build_character_constant(pair: Pair<'_, Rule>) -> Result<Expression, Box<dyn Error>> {
    let token = pair.into_inner().next().unwrap();
    match token.as_rule() {
        Rule::char_no_escape => Ok(Expression::CharacterConstant(
            token.as_str().chars().next().unwrap(),
        )),
        Rule::escape_sequence => Ok(Expression::CharacterConstant(build_escape_sequence(token)?)),
        _ => unreachable!(),
    }
}

pub fn build_floating_constant(pair: Pair<'_, Rule>) -> Result<Expression, Box<dyn Error>> {
    let token = pair.into_inner().next().unwrap();
    match token.as_rule() {
        Rule::decimal_floating_constant => build_decimal_floating_constant(token),
        Rule::hex_floating_constant => build_hex_floating_constant(token),
        _ => unreachable!(),
    }
}

pub fn build_decimal_floating_constant(pair: Pair<'_, Rule>) -> Result<Expression, Box<dyn Error>> {
    let mut number: f64 = Default::default();
    let mut is_double = true;
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::decimal_floating_constant_no_suffix => {
                number = token.as_str().to_string().parse::<f64>().unwrap(); // TODO(TO/GA): test
            }
            Rule::floating_suffix => {
                is_double = match token.into_inner().next().unwrap().as_rule() {
                    Rule::f_ => false,
                    Rule::l_ => true,
                    _ => unreachable!(),
                };
            }
            _ => {}
        }
    }
    Ok(match is_double {
        false => Expression::FloatConstant(number as f32),
        true => Expression::DoubleConstant(number),
    })
}

pub fn build_hex_floating_constant(_pair: Pair<'_, Rule>) -> Result<Expression, Box<dyn Error>> {
    // TODO(TO/GA)
    unimplemented!();
}
