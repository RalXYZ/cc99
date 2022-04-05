use pest::error::ErrorVariant;
use pest::iterators::Pair;

use super::*;

pub fn build_expression(pair: Pair<'_, Rule>) -> Result<Expression, Box<dyn Error>> {
    let mut expression: Option<Expression> = None;
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::assignment_expression => {
                expression = Some(match expression {
                    Some(e) => Expression::Binary(
                        BinaryOperation::Comma,
                        Box::new(e),
                        Box::new(build_assignment_expression(token)?),
                    ),
                    None => build_assignment_expression(token)?,
                });
            }
            _ => unreachable!(),
        }
    }
    Ok(expression.unwrap())
}

pub fn build_assignment_expression(pair: Pair<'_, Rule>) -> Result<Expression, Box<dyn Error>> {
    let mut lhs: Expression = Default::default();
    let mut rhs: Expression = Default::default();
    let mut assignment_operator: AssignOperation = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::conditional_expression => {
                return build_conditional_expression(token);
            }
            Rule::unary_expression => {
                lhs = build_unary_expression(token)?;
            }
            Rule::assignment_operator => {
                assignment_operator = match token.into_inner().next().unwrap().as_rule() {
                    Rule::assign_naive_op => AssignOperation::Naive,
                    Rule::assign_add_op => AssignOperation::Addition,
                    Rule::assign_sub_op => AssignOperation::Subtraction,
                    Rule::assign_mul_op => AssignOperation::Multiplication,
                    Rule::assign_div_op => AssignOperation::Division,
                    Rule::assign_mod_op => AssignOperation::Modulo,
                    Rule::assign_bitwise_and_op => AssignOperation::BitwiseAnd,
                    Rule::assign_bitwise_or_op => AssignOperation::BitwiseOr,
                    Rule::assign_bitwise_xor_op => AssignOperation::BitwiseXor,
                    Rule::assign_left_shift_op => AssignOperation::LeftShift,
                    Rule::assign_right_shift_op => AssignOperation::RightShift,
                    _ => unreachable!(),
                };
            }
            Rule::assignment_expression => {
                rhs = build_assignment_expression(token)?;
            }
            _ => unreachable!(),
        }
    }
    Ok(Expression::Assignment(
        assignment_operator,
        Box::new(lhs),
        Box::new(rhs),
    ))
}

pub fn build_conditional_expression(pair: Pair<'_, Rule>) -> Result<Expression, Box<dyn Error>> {
    let mut expressions: Vec<Expression> = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::logical_or_expression => {
                expressions.push(build_binary_expression(token)?);
            }
            Rule::expression => {
                expressions.push(build_expression(token)?);
            }
            Rule::conditional_expression => {
                expressions.push(build_conditional_expression(token)?);
            }
            _ => unreachable!(),
        }
    }
    Ok(match expressions.len() {
        1 => expressions[0].to_owned(),
        3 => Expression::Conditional(
            Box::new(expressions[0].to_owned()),
            Box::new(expressions[1].to_owned()),
            Box::new(expressions[2].to_owned()),
        ),
        _ => unreachable!(),
    })
}

pub fn build_binary_expression(pair: Pair<'_, Rule>) -> Result<Expression, Box<dyn Error>> {
    // recursive termination condition
    if pair.as_rule() == Rule::unary_expression {
        return build_unary_expression(pair);
    }

    let mut expression: Option<Expression> = None;
    let mut operation: BinaryOperation = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::logical_or_op => {
                operation = BinaryOperation::LogicalOr;
            }
            Rule::logical_and_op => {
                operation = BinaryOperation::LogicalAnd;
            }
            Rule::bitwise_or_op => {
                operation = BinaryOperation::BitwiseOr;
            }
            Rule::bitwise_xor_op => {
                operation = BinaryOperation::BitwiseXor;
            }
            Rule::bitwise_and_op => {
                operation = BinaryOperation::BitwiseAnd;
            }
            Rule::equal_op => {
                operation = BinaryOperation::Equal;
            }
            Rule::not_equal_op => {
                operation = BinaryOperation::NotEqual;
            }
            Rule::less_than_op => {
                operation = BinaryOperation::LessThan;
            }
            Rule::greater_than_op => {
                operation = BinaryOperation::GreaterThan;
            }
            Rule::less_than_or_equal_op => {
                operation = BinaryOperation::LessThanOrEqual;
            }
            Rule::greater_than_or_equal_op => {
                operation = BinaryOperation::GreaterThanOrEqual;
            }
            Rule::left_shift_op => {
                operation = BinaryOperation::LeftShift;
            }
            Rule::right_shift_op => {
                operation = BinaryOperation::RightShift;
            }
            Rule::add_op => {
                operation = BinaryOperation::Addition;
            }
            Rule::sub_op => {
                operation = BinaryOperation::Subtraction;
            }
            Rule::mul_op => {
                operation = BinaryOperation::Multiplication;
            }
            Rule::div_op => {
                operation = BinaryOperation::Division;
            }
            Rule::mod_op => {
                operation = BinaryOperation::Modulo;
            }
            Rule::logical_or_expression
            | Rule::logical_and_expression
            | Rule::bitwise_or_expression
            | Rule::bitwise_xor_expression
            | Rule::bitwise_and_expression
            | Rule::equal_expression
            | Rule::relational_expression
            | Rule::shift_expression
            | Rule::add_expression
            | Rule::mul_expression
            | Rule::unary_expression => {
                expression = Some(match expression {
                    Some(e) => Expression::Binary(
                        operation.clone(),
                        Box::new(e),
                        Box::new(build_binary_expression(token)?),
                    ),
                    None => build_binary_expression(token)?,
                });
            }
            _ => unreachable!(),
        }
    }
    Ok(expression.unwrap())
}

pub fn build_unary_expression(pair: Pair<'_, Rule>) -> Result<Expression, Box<dyn Error>> {
    let mut cast_type: Option<BasicType> = None;
    let mut unary_operation: UnaryOperation = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::sizeof_ => {}
            Rule::type_name => {
                return Ok(Expression::SizeofType(build_type_name(token)?));
            }
            Rule::prefix_unary_operator => {
                let sub_token = token.into_inner().next().unwrap();
                unary_operation = match sub_token.as_rule() {
                    Rule::prefix_inc_op => UnaryOperation::PrefixIncrement,
                    Rule::prefix_dec_op => UnaryOperation::PrefixDecrement,
                    Rule::unary_plus_op => UnaryOperation::UnaryPlus,
                    Rule::unary_minus_op => UnaryOperation::UnaryMinus,
                    Rule::logical_not_op => UnaryOperation::LogicalNot,
                    Rule::bitwise_not_op => UnaryOperation::BitwiseNot,
                    Rule::dereference_op => UnaryOperation::Dereference,
                    Rule::reference_op => UnaryOperation::Reference,
                    Rule::sizeof_ => UnaryOperation::SizeofExpr,
                    Rule::type_name => {
                        cast_type = Some(build_type_name(sub_token)?);
                        Default::default()
                    }
                    _ => unreachable!(),
                };
            }
            Rule::unary_expression => match cast_type {
                Some(cast_type) => {
                    return Ok(Expression::TypeCast(
                        cast_type,
                        Box::new(build_unary_expression(token)?),
                    ));
                }
                None => {
                    return Ok(Expression::Unary(
                        unary_operation,
                        Box::new(build_unary_expression(token)?),
                    ));
                }
            },
            Rule::postfix_unary_expression => {
                return build_postfix_unary_expression(token);
            }
            _ => unreachable!(),
        }
    }
    Ok(Default::default())
}

pub fn build_postfix_unary_expression(pair: Pair<'_, Rule>) -> Result<Expression, Box<dyn Error>> {
    let mut expression: Expression = Default::default();
    let mut object_or_pointer = true; // true if object, false otherwise
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::primary_expression => {
                expression = build_primary_expression(token)?;
            }
            Rule::postfix_inc_op => {
                expression =
                    Expression::Unary(UnaryOperation::PostfixIncrement, Box::new(expression));
            }
            Rule::postfix_dec_op => {
                expression =
                    Expression::Unary(UnaryOperation::PostfixDecrement, Box::new(expression));
            }
            Rule::function_call => {
                let mut arguments: Vec<Expression> = Default::default();
                for argument_list in token.into_inner() {
                    for argument in argument_list.into_inner() {
                        arguments.push(build_assignment_expression(argument)?);
                    }
                }
                expression = Expression::FunctionCall(Box::new(expression), arguments);
            }
            Rule::expression => {
                expression = Expression::ArraySubscript(
                    Box::new(expression),
                    Box::new(build_expression(token)?),
                );
            }
            Rule::member_of_object_op => {
                object_or_pointer = true;
            }
            Rule::member_of_pointer_op => {
                object_or_pointer = false;
            }
            Rule::identifier => {
                expression = match object_or_pointer {
                    true => {
                        Expression::MemberOfObject(Box::new(expression), token.as_str().to_owned())
                    }
                    false => {
                        Expression::MemberOfPointer(Box::new(expression), token.as_str().to_owned())
                    }
                };
            }
            _ => unreachable!(),
        }
    }
    Ok(expression)
}

pub fn build_primary_expression(pair: Pair<'_, Rule>) -> Result<Expression, Box<dyn Error>> {
    let token = pair.into_inner().next().unwrap();
    match token.as_rule() {
        Rule::identifier => Ok(Expression::Identifier(token.as_str().to_owned())),
        Rule::constant => build_constant(token),
        Rule::string_literal => build_string_literal(token),
        Rule::expression => build_expression(token),
        _ => unreachable!(),
    }
}

pub fn build_type_name(pair: Pair<'_, Rule>) -> Result<BasicType, Box<dyn Error>> {
    let span = pair.as_span();
    let mut fake_ast: Vec<Declaration> = Default::default();
    let mut derived_type: Type = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::declaration_specifiers => {
                derived_type = build_declaration_specifiers(&mut fake_ast, token)?;
            }
            Rule::pointer => {
                build_pointer(&mut derived_type, token)?;
            }
            Rule::function_parameter_list => {
                build_function_parameter_list(&mut fake_ast, &mut derived_type, token)?;
            }
            Rule::assignment_expression => {
                derived_type.basic_type = BasicType {
                    qualifier: vec![],
                    base_type: BaseType::Array(
                        Box::new(derived_type.basic_type),
                        Box::new(build_assignment_expression(token)?),
                    ),
                };
            }
            _ => unreachable!(),
        }
    }
    if derived_type.storage_class_specifier != StorageClassSpecifier::Auto {
        return Err(Box::new(pest::error::Error::<Rule>::new_from_span(
            ErrorVariant::CustomError {
                message: "the type to be casted to can't have any storage class specifier"
                    .to_string(),
            },
            span,
        )));
    }
    Ok(derived_type.basic_type)
}
