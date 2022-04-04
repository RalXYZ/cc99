use pest::iterators::Pair;

use super::*;

pub fn build_function_definition(ast: &mut Vec<Declaration>, pair: Pair<'_, Rule>) {
    let mut derived_type: Type = Default::default();
    let mut identifier: String = Default::default();
    let mut parameter_names: Vec<Option<String>> = Default::default();
    let mut function_body: Statement = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::declaration_specifiers => {
                derived_type = build_declaration_specifiers(ast, token);
            }
            Rule::pointer => {
                build_pointer(&mut derived_type, token);
            }
            Rule::identifier => {
                identifier = token.as_str().to_string();
            }
            Rule::function_parameter_list => {
                parameter_names = build_function_parameter_list(ast, &mut derived_type, token);
            }
            Rule::compound_statement => {
                function_body = build_compound_statement(token);
            }
            _ => unreachable!(),
        }
    }
    ast.push(Declaration::FunctionDefinition(
        derived_type,
        identifier,
        parameter_names,
        function_body,
    ));
}

pub fn build_statement(pair: Pair<'_, Rule>) -> Statement {
    let token = pair.into_inner().next().unwrap();
    match token.as_rule() {
        Rule::labeled_statement => build_labeled_statement(token),
        Rule::case_statement => build_case_statement(token),
        Rule::expression_statement => build_expression_statement(token),
        Rule::compound_statement => build_compound_statement(token),
        Rule::selection_statement => build_selection_statement(token),
        Rule::iteration_statement => build_iteration_statement(token),
        Rule::jump_statement => build_jump_statement(token),
        _ => unreachable!(),
    }
}

pub fn build_labeled_statement(pair: Pair<'_, Rule>) -> Statement {
    let mut label: String = Default::default();
    let mut statement: Statement = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::identifier => {
                label = token.as_str().to_string();
            }
            Rule::statement => {
                statement = build_statement(token);
            }
            _ => unreachable!(),
        }
    }
    Statement::Labeled(label, Box::new(statement))
}

pub fn build_case_statement(pair: Pair<'_, Rule>) -> Statement {
    let mut expression: Option<Box<Expression>> = None;
    let mut statement: Statement = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::case_ => {}
            Rule::default_ => {}
            Rule::assignment_expression => {
                expression = Some(Box::new(build_assignment_expression(token)));
            }
            Rule::statement => {
                statement = build_statement(token);
            }
            _ => unreachable!(),
        }
    }
    Statement::Case(expression, Box::new(statement))
}

pub fn build_expression_statement(pair: Pair<'_, Rule>) -> Statement {
    let mut expression: Option<Box<Expression>> = None;
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::expression => {
                expression = Some(Box::new(build_expression(token)));
            }
            _ => unreachable!(),
        }
    }
    match expression {
        Some(expr) => Statement::Expression(expr),
        None => Statement::Expression(Box::new(Expression::Empty)),
    }
}

pub fn build_compound_statement(pair: Pair<'_, Rule>) -> Statement {
    let mut statements: Vec<StatementOrDeclaration> = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::statement => {
                statements.push(StatementOrDeclaration::Statement(build_statement(token)));
            }
            Rule::declaration => {
                let mut sub_ast = Vec::new();
                build_declaration(&mut sub_ast, token);
                for declaration in sub_ast {
                    match declaration {
                        Declaration::Declaration(declaration_type, identifier, initializer) => {
                            statements.push(StatementOrDeclaration::Declaration(
                                Declaration::Declaration(declaration_type, identifier, initializer),
                            ));
                        }
                        Declaration::FunctionDefinition(_, _, _, _) => {
                            // TODO(TO/GA): throw error
                            unreachable!();
                        }
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    Statement::Compound(statements)
}

pub fn build_selection_statement(pair: Pair<'_, Rule>) -> Statement {
    let token = pair.into_inner().next().unwrap();
    match token.as_rule() {
        Rule::if_statement => build_if_statement(token),
        Rule::switch_statement => build_switch_statement(token),
        _ => unreachable!(),
    }
}

pub fn build_iteration_statement(pair: Pair<'_, Rule>) -> Statement {
    let token = pair.into_inner().next().unwrap();
    match token.as_rule() {
        Rule::for_statement => build_for_statement(token),
        Rule::while_statement => build_while_statement(token),
        Rule::do_while_statement => build_do_while_statement(token),
        _ => unreachable!(),
    }
}

pub fn build_jump_statement(pair: Pair<'_, Rule>) -> Statement {
    let token = pair.into_inner().next().unwrap();
    match token.as_rule() {
        Rule::break_statement => Statement::Break,
        Rule::continue_statement => Statement::Continue,
        Rule::return_statement => build_return_statement(token),
        Rule::goto_statement => build_goto_statement(token),
        _ => unreachable!(),
    }
}

pub fn build_if_statement(pair: Pair<'_, Rule>) -> Statement {
    let mut expression: Expression = Default::default();
    let mut statements: Vec<Statement> = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::if_ => {}
            Rule::else_ => {}
            Rule::expression => {
                expression = build_expression(token);
            }
            Rule::statement => {
                statements.push(build_statement(token));
            }
            _ => unreachable!(),
        }
    }
    Statement::If(
        Box::new(expression),
        Box::new(statements[0].to_owned()),
        match statements.len() {
            1 => None,
            2 => Some(Box::new(statements[1].to_owned())),
            _ => unreachable!(),
        },
    )
}

pub fn build_switch_statement(pair: Pair<'_, Rule>) -> Statement {
    let mut expression: Expression = Default::default();
    let mut statement: Statement = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::switch_ => {}
            Rule::expression => {
                expression = build_expression(token);
            }
            Rule::statement => {
                statement = build_statement(token);
            }
            _ => unreachable!(),
        }
    }
    Statement::Switch(Box::new(expression), Box::new(statement))
}

pub fn build_for_statement(pair: Pair<'_, Rule>) -> Statement {
    let mut init_clause: Option<Box<ForInitClause>> = None;
    let mut condition_expression: Option<Box<Expression>> = None;
    let mut iteration_expression: Option<Box<Expression>> = None;
    let mut statement: Statement = Default::default();

    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::for_ => {}
            Rule::for_init_clause => {
                init_clause = Some(Box::new(build_for_init_clause(token)));
            }
            Rule::for_cond_expression => {
                condition_expression = Some(Box::new(build_expression(
                    token.into_inner().next().unwrap(),
                )));
            }
            Rule::for_iteration_expression => {
                iteration_expression = Some(Box::new(build_expression(
                    token.into_inner().next().unwrap(),
                )));
            }
            Rule::statement => {
                statement = build_statement(token);
            }
            _ => unreachable!(),
        }
    }
    Statement::For(
        init_clause,
        condition_expression,
        iteration_expression,
        Box::new(statement),
    )
}

pub fn build_while_statement(pair: Pair<'_, Rule>) -> Statement {
    let mut expression: Expression = Default::default();
    let mut statement: Statement = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::while_ => {}
            Rule::expression => {
                expression = build_expression(token);
            }
            Rule::statement => {
                statement = build_statement(token);
            }
            _ => unreachable!(),
        }
    }
    Statement::While(Box::new(expression), Box::new(statement))
}

pub fn build_do_while_statement(pair: Pair<'_, Rule>) -> Statement {
    let mut expression: Expression = Default::default();
    let mut statement: Statement = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::do_ => {}
            Rule::while_ => {}
            Rule::statement => {
                statement = build_statement(token);
            }
            Rule::expression => {
                expression = build_expression(token);
            }
            _ => unreachable!(),
        }
    }
    Statement::DoWhile(Box::new(statement), Box::new(expression))
}

pub fn build_return_statement(pair: Pair<'_, Rule>) -> Statement {
    let mut expression: Option<Box<Expression>> = None;
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::return_ => {}
            Rule::expression => {
                expression = Some(Box::new(build_expression(token)));
            }
            _ => unreachable!(),
        }
    }
    Statement::Return(expression)
}

pub fn build_goto_statement(pair: Pair<'_, Rule>) -> Statement {
    let mut label: String = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::goto_ => {}
            Rule::identifier => {
                label = token.as_str().to_owned();
            }
            _ => unreachable!(),
        }
    }
    Statement::Goto(label)
}

pub fn build_for_init_clause(pair: Pair<'_, Rule>) -> ForInitClause {
    let mut expression: Option<Expression> = None;
    let mut basic_type: Type = Default::default();
    let mut sub_ast = Vec::new();

    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::expression => {
                expression = Some(build_expression(token));
            }
            Rule::declaration_specifiers => {
                basic_type = build_declaration_specifiers(&mut sub_ast, token);
            }
            Rule::declarator_and_initializer_list => {
                for list_entry in token.into_inner() {
                    match list_entry.as_rule() {
                        Rule::declarator_and_initializer => {
                            build_declarator_and_initializer(&mut sub_ast, list_entry, &basic_type);
                        }
                        _ => unreachable!(),
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    // TODO(TO/GA): throw error if sub_ast has function declarations
    match expression {
        Some(expression) => ForInitClause::Expression(expression),
        None => ForInitClause::Declaration(sub_ast),
    }
}

pub fn build_expression(pair: Pair<'_, Rule>) -> Expression {
    let mut expression: Option<Expression> = None;
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::assignment_expression => {
                expression = Some(match expression {
                    Some(e) => Expression::Binary(
                        BinaryOperation::Comma,
                        Box::new(e),
                        Box::new(build_assignment_expression(token)),
                    ),
                    None => build_assignment_expression(token),
                });
            }
            _ => unreachable!(),
        }
    }
    expression.unwrap()
}

pub fn build_assignment_expression(pair: Pair<'_, Rule>) -> Expression {
    let mut lhs: Expression = Default::default();
    let mut rhs: Expression = Default::default();
    let mut assignment_operator: AssignOperation = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::conditional_expression => {
                return build_conditional_expression(token);
            }
            Rule::unary_expression => {
                lhs = build_unary_expression(token);
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
                rhs = build_assignment_expression(token);
            }
            _ => unreachable!(),
        }
    }
    Expression::Assignment(assignment_operator, Box::new(lhs), Box::new(rhs))
}

pub fn build_conditional_expression(pair: Pair<'_, Rule>) -> Expression {
    let mut expressions: Vec<Expression> = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::logical_or_expression => {
                expressions.push(build_binary_expression(token));
            }
            Rule::expression => {
                expressions.push(build_expression(token));
            }
            _ => unreachable!(),
        }
    }
    match expressions.len() {
        1 => expressions[0].to_owned(),
        3 => Expression::Conditional(
            Box::new(expressions[0].to_owned()),
            Box::new(expressions[1].to_owned()),
            Box::new(expressions[2].to_owned()),
        ),
        _ => unreachable!(),
    }
}

pub fn build_binary_expression(pair: Pair<'_, Rule>) -> Expression {
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
                        Box::new(build_binary_expression(token)),
                    ),
                    None => build_binary_expression(token),
                });
            }
            _ => unreachable!(),
        }
    }
    expression.unwrap()
}

pub fn build_unary_expression(pair: Pair<'_, Rule>) -> Expression {
    let mut cast_type: Option<BasicType> = None;
    let mut unary_operation: UnaryOperation = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::sizeof_ => {}
            Rule::type_name => {
                return Expression::SizeofType(build_type_name(token));
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
                        cast_type = Some(build_type_name(sub_token));
                        Default::default()
                    }
                    _ => unreachable!(),
                };
            }
            Rule::unary_expression => match cast_type {
                Some(cast_type) => {
                    return Expression::TypeCast(
                        cast_type,
                        Box::new(build_unary_expression(token)),
                    );
                }
                None => {
                    return Expression::Unary(
                        unary_operation,
                        Box::new(build_unary_expression(token)),
                    );
                }
            },
            Rule::postfix_unary_expression => {
                return build_postfix_unary_expression(token);
            }
            _ => unreachable!(),
        }
    }
    Default::default()
}

pub fn build_postfix_unary_expression(pair: Pair<'_, Rule>) -> Expression {
    let mut expression: Expression = Default::default();
    let mut object_or_pointer = true; // true if object, false otherwise
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::primary_expression => {
                expression = build_primary_expression(token);
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
                        arguments.push(build_assignment_expression(argument));
                    }
                }
                expression = Expression::FunctionCall(Box::new(expression), arguments);
            }
            Rule::expression => {
                expression = Expression::ArraySubscript(
                    Box::new(expression),
                    Box::new(build_expression(token)),
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
    expression
}

pub fn build_primary_expression(pair: Pair<'_, Rule>) -> Expression {
    let token = pair.into_inner().next().unwrap();
    match token.as_rule() {
        Rule::identifier => Expression::Identifier(token.as_str().to_owned()),
        Rule::constant => build_constant(token),
        Rule::string_literal => build_string_literal(token),
        Rule::expression => build_expression(token),
        _ => unreachable!(),
    }
}

pub fn build_type_name(pair: Pair<'_, Rule>) -> BasicType {
    let mut fake_ast: Vec<Declaration> = Default::default();
    let mut derived_type: Type = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::declaration_specifiers => {
                derived_type = build_declaration_specifiers(&mut fake_ast, token);
            }
            Rule::pointer => {
                build_pointer(&mut derived_type, token);
            }
            Rule::function_parameter_list => {
                build_function_parameter_list(&mut fake_ast, &mut derived_type, token);
            }
            Rule::assignment_expression => {
                derived_type.basic_type = BasicType {
                    qualifier: vec![],
                    base_type: BaseType::Array(
                        Box::new(derived_type.basic_type),
                        Box::new(build_assignment_expression(token)),
                    ),
                };
            }
            _ => unreachable!(),
        }
    }
    // TODO:(TO/GA) throw error if storage_class_specifier is not empty
    derived_type.basic_type
}

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
