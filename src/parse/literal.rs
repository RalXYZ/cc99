use pest::iterators::Pair;

use super::*;

pub fn build_string_literal(pair: Pair<'_, Rule>) -> Expression {
  let mut string_literal: String = Default::default();
  for token in pair.into_inner() {
      match token.as_rule() {
          Rule::char_no_escape => {
              string_literal.push_str(token.as_str());
          }
          Rule::escape_sequence => {
              string_literal.push(build_escape_sequence(token));
          }
          _ => unreachable!(),
      }
  }
  Expression::StringLiteral(string_literal)
}

pub fn build_escape_sequence(pair: Pair<'_, Rule>) -> char {
  let escape_sequence = pair.as_str();
  match escape_sequence {
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
              return '\0';
          }
          unimplemented!();
      }
  }
}

pub fn build_constant(pair: Pair<'_, Rule>) -> Expression {
  let token = pair.into_inner().next().unwrap();
  match token.as_rule() {
      Rule::integer_constant => build_integer_constant(token),
      Rule::character_constant => build_character_constant(token),
      Rule::floating_constant => build_floating_constant(token),
      _ => unreachable!(),
  }
}

pub fn build_integer_constant(pair: Pair<'_, Rule>) -> Expression {
  let mut number: i128 = Default::default();
  for token in pair.into_inner() {
      match token.as_rule() {
          Rule::decimal_constant => {
              number = token.as_str().to_string().parse::<i128>().unwrap();
          }
          Rule::octal_constant => {
              let number_str = token.as_str();
              number = match number_str.len() {
                  0 => unreachable!(),
                  1 => 0,
                  _ => i128::from_str_radix(&number_str[1..number_str.len()], 8).unwrap(),
              }
          }
          Rule::hex_constant => {
              let number_str = token.as_str();
              number = i128::from_str_radix(&number_str[2..number_str.len()], 16).unwrap()
          }
          Rule::binary_constant => {
              let number_str = token.as_str();
              number = i128::from_str_radix(&number_str[2..number_str.len()], 2).unwrap()
          }
          Rule::integer_suffix => match token.into_inner().next().unwrap().as_rule() {
              Rule::ull_ => {
                  return Expression::UnsignedLongLongConstant(number as u64);
              }
              Rule::ll_ => {
                  return Expression::LongLongConstant(number as i64);
              }
              Rule::ul_ => {
                  return Expression::UnsignedLongConstant(number as u64);
              }
              Rule::l_ => {
                  return Expression::LongConstant(number as i64);
              }
              Rule::u_ => {
                  return Expression::UnsignedIntegerConstant(number as u32);
              }
              _ => unreachable!(),
          },
          _ => unreachable!(),
      }
  }
  Expression::IntegerConstant(number as i32) // TODO(TO/GA): throw error if overflow
}

pub fn build_character_constant(pair: Pair<'_, Rule>) -> Expression {
  let token = pair.into_inner().next().unwrap();
  match token.as_rule() {
      Rule::char_no_escape => {
          Expression::CharacterConstant(token.as_str().chars().next().unwrap())
      }
      Rule::escape_sequence => Expression::CharacterConstant(build_escape_sequence(token)),
      _ => unreachable!(),
  }
}

pub fn build_floating_constant(pair: Pair<'_, Rule>) -> Expression {
  let token = pair.into_inner().next().unwrap();
  match token.as_rule() {
      Rule::decimal_floating_constant => build_decimal_floating_constant(token),
      Rule::hex_floating_constant => build_hex_floating_constant(token),
      _ => unreachable!(),
  }
}

pub fn build_decimal_floating_constant(pair: Pair<'_, Rule>) -> Expression {
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
  match is_double {
      false => Expression::FloatConstant(number as f32),
      true => Expression::DoubleConstant(number),
  }
}

pub fn build_hex_floating_constant(_pair: Pair<'_, Rule>) -> Expression {
  // TODO(TO/GA)
  unimplemented!();
}
