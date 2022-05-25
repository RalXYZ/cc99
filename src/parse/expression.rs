use pest::error::ErrorVariant;
use pest::iterators::Pair;

use super::*;

impl<'ctx> Parse<'ctx> {
    pub fn build_expression(
        &mut self,
        pair: Pair<'ctx, Rule>,
    ) -> Result<Expression<'ctx>, Box<dyn Error>> {
        let mut op_span = Span::new(self.code, 0, 0).unwrap();
        let mut expression: Option<Expression> = None;
        for token in pair.into_inner() {
            let token_span = token.as_span();
            match token.as_rule() {
                Rule::comma => {
                    op_span = token_span;
                }
                Rule::assignment_expression => {
                    expression = Some(match expression {
                        Some(e) => {
                            let e_span = e.span.clone();
                            Expression {
                                node: ExpressionEnum::Binary(
                                    BinaryOperation {
                                        node: BinaryOperationEnum::Comma,
                                        span: op_span.clone(),
                                    },
                                    Box::new(e),
                                    Box::new(self.build_assignment_expression(token)?),
                                ),
                                span: Span::new(self.code, e_span.start(), token_span.end())
                                    .unwrap(),
                            }
                        }
                        None => self.build_assignment_expression(token)?,
                    });
                }
                _ => unreachable!(),
            }
        }
        Ok(expression.unwrap())
    }

    pub fn build_assignment_expression(
        &mut self,
        pair: Pair<'ctx, Rule>,
    ) -> Result<Expression<'ctx>, Box<dyn Error>> {
        let span = pair.as_span();
        let mut lhs = Expression::default(self.code);
        let mut rhs = Expression::default(self.code);
        let mut assignment_operator = AssignOperation::default(self.code);
        for token in pair.into_inner() {
            let token_span = token.as_span();
            match token.as_rule() {
                Rule::conditional_expression => {
                    return self.build_conditional_expression(token);
                }
                Rule::unary_expression => {
                    lhs = self.build_unary_expression(token)?;
                }
                Rule::assignment_operator => {
                    assignment_operator = AssignOperation {
                        node: match token.into_inner().next().unwrap().as_rule() {
                            Rule::assign_naive_op => AssignOperationEnum::Naive,
                            Rule::assign_add_op => AssignOperationEnum::Addition,
                            Rule::assign_sub_op => AssignOperationEnum::Subtraction,
                            Rule::assign_mul_op => AssignOperationEnum::Multiplication,
                            Rule::assign_div_op => AssignOperationEnum::Division,
                            Rule::assign_mod_op => AssignOperationEnum::Modulo,
                            Rule::assign_bitwise_and_op => AssignOperationEnum::BitwiseAnd,
                            Rule::assign_bitwise_or_op => AssignOperationEnum::BitwiseOr,
                            Rule::assign_bitwise_xor_op => AssignOperationEnum::BitwiseXor,
                            Rule::assign_left_shift_op => AssignOperationEnum::LeftShift,
                            Rule::assign_right_shift_op => AssignOperationEnum::RightShift,
                            _ => unreachable!(),
                        },
                        span: token_span,
                    };
                }
                Rule::assignment_expression => {
                    rhs = self.build_assignment_expression(token)?;
                }
                _ => unreachable!(),
            }
        }
        Ok(Expression {
            node: ExpressionEnum::Assignment(assignment_operator, Box::new(lhs), Box::new(rhs)),
            span,
        })
    }

    fn build_conditional_expression(
        &mut self,
        pair: Pair<'ctx, Rule>,
    ) -> Result<Expression<'ctx>, Box<dyn Error>> {
        let span = pair.as_span();
        let mut expressions: Vec<Expression> = Default::default();
        for token in pair.into_inner() {
            match token.as_rule() {
                Rule::logical_or_expression => {
                    expressions.push(self.build_binary_expression(token)?);
                }
                Rule::expression => {
                    expressions.push(self.build_expression(token)?);
                }
                Rule::conditional_expression => {
                    expressions.push(self.build_conditional_expression(token)?);
                }
                _ => unreachable!(),
            }
        }
        Ok(match expressions.len() {
            1 => expressions[0].to_owned(),
            3 => Expression {
                node: ExpressionEnum::Conditional(
                    Box::new(expressions[0].to_owned()),
                    Box::new(expressions[1].to_owned()),
                    Box::new(expressions[2].to_owned()),
                ),
                span,
            },
            _ => unreachable!(),
        })
    }

    fn build_binary_expression(
        &mut self,
        pair: Pair<'ctx, Rule>,
    ) -> Result<Expression<'ctx>, Box<dyn Error>> {
        // recursive termination condition
        if pair.as_rule() == Rule::unary_expression {
            return self.build_unary_expression(pair);
        }

        let span = pair.as_span();
        let mut expression: Option<Expression> = None;
        let mut operation = BinaryOperation::default(self.code);
        for token in pair.into_inner() {
            match token.as_rule() {
                Rule::logical_or_op => {
                    operation = BinaryOperation {
                        node: BinaryOperationEnum::LogicalOr,
                        span: token.as_span(),
                    };
                }
                Rule::logical_and_op => {
                    operation = BinaryOperation {
                        node: BinaryOperationEnum::LogicalAnd,
                        span: token.as_span(),
                    };
                }
                Rule::bitwise_or_op => {
                    operation = BinaryOperation {
                        node: BinaryOperationEnum::BitwiseOr,
                        span: token.as_span(),
                    };
                }
                Rule::bitwise_xor_op => {
                    operation = BinaryOperation {
                        node: BinaryOperationEnum::BitwiseXor,
                        span: token.as_span(),
                    };
                }
                Rule::bitwise_and_op => {
                    operation = BinaryOperation {
                        node: BinaryOperationEnum::BitwiseAnd,
                        span: token.as_span(),
                    };
                }
                Rule::equal_op => {
                    operation = BinaryOperation {
                        node: BinaryOperationEnum::Equal,
                        span: token.as_span(),
                    };
                }
                Rule::not_equal_op => {
                    operation = BinaryOperation {
                        node: BinaryOperationEnum::NotEqual,
                        span: token.as_span(),
                    };
                }
                Rule::less_than_op => {
                    operation = BinaryOperation {
                        node: BinaryOperationEnum::LessThan,
                        span: token.as_span(),
                    };
                }
                Rule::greater_than_op => {
                    operation = BinaryOperation {
                        node: BinaryOperationEnum::GreaterThan,
                        span: token.as_span(),
                    };
                }
                Rule::less_than_or_equal_op => {
                    operation = BinaryOperation {
                        node: BinaryOperationEnum::LessThanOrEqual,
                        span: token.as_span(),
                    };
                }
                Rule::greater_than_or_equal_op => {
                    operation = BinaryOperation {
                        node: BinaryOperationEnum::GreaterThanOrEqual,
                        span: token.as_span(),
                    };
                }
                Rule::left_shift_op => {
                    operation = BinaryOperation {
                        node: BinaryOperationEnum::LeftShift,
                        span: token.as_span(),
                    };
                }
                Rule::right_shift_op => {
                    operation = BinaryOperation {
                        node: BinaryOperationEnum::RightShift,
                        span: token.as_span(),
                    };
                }
                Rule::add_op => {
                    operation = BinaryOperation {
                        node: BinaryOperationEnum::Addition,
                        span: token.as_span(),
                    };
                }
                Rule::sub_op => {
                    operation = BinaryOperation {
                        node: BinaryOperationEnum::Subtraction,
                        span: token.as_span(),
                    };
                }
                Rule::mul_op => {
                    operation = BinaryOperation {
                        node: BinaryOperationEnum::Multiplication,
                        span: token.as_span(),
                    };
                }
                Rule::div_op => {
                    operation = BinaryOperation {
                        node: BinaryOperationEnum::Division,
                        span: token.as_span(),
                    };
                }
                Rule::mod_op => {
                    operation = BinaryOperation {
                        node: BinaryOperationEnum::Modulo,
                        span: token.as_span(),
                    };
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
                        Some(e) => Expression {
                            node: ExpressionEnum::Binary(
                                operation.clone(),
                                Box::new(e),
                                Box::new(self.build_binary_expression(token)?),
                            ),
                            span: span.clone(),
                        },
                        None => self.build_binary_expression(token)?,
                    });
                }
                _ => unreachable!(),
            }
        }
        Ok(expression.unwrap())
    }

    fn build_unary_expression(
        &mut self,
        pair: Pair<'ctx, Rule>,
    ) -> Result<Expression<'ctx>, Box<dyn Error>> {
        let span = pair.as_span();
        let mut unary_operation = UnaryOperation::default(self.code);
        for token in pair.into_inner() {
            let token_span = token.as_span();
            match token.as_rule() {
                Rule::sizeof_ => {}
                Rule::type_name => {
                    return Ok(Expression {
                        node: ExpressionEnum::SizeofType(self.build_type_name(token)?),
                        span,
                    });
                }
                Rule::prefix_unary_operator => {
                    let sub_token = token.into_inner().next().unwrap();
                    unary_operation = UnaryOperation {
                        node: match sub_token.as_rule() {
                            Rule::prefix_inc_op => UnaryOperationEnum::PrefixIncrement,
                            Rule::prefix_dec_op => UnaryOperationEnum::PrefixDecrement,
                            Rule::unary_plus_op => UnaryOperationEnum::UnaryPlus,
                            Rule::unary_minus_op => UnaryOperationEnum::UnaryMinus,
                            Rule::logical_not_op => UnaryOperationEnum::LogicalNot,
                            Rule::bitwise_not_op => UnaryOperationEnum::BitwiseNot,
                            Rule::dereference_op => UnaryOperationEnum::Dereference,
                            Rule::reference_op => UnaryOperationEnum::Reference,
                            Rule::sizeof_ => UnaryOperationEnum::SizeofExpr,
                            _ => unreachable!(),
                        },
                        span: token_span,
                    };
                }
                Rule::unary_expression => {
                    return Ok(Expression {
                        node: ExpressionEnum::Unary(
                            unary_operation,
                            Box::new(self.build_unary_expression(token)?),
                        ),
                        span,
                    });
                }
                Rule::postfix_unary_expression => {
                    return self.build_postfix_unary_expression(token);
                }
                _ => unreachable!(),
            }
        }
        Ok(Expression {
            node: ExpressionEnum::Empty,
            span,
        })
    }

    fn build_postfix_unary_expression(
        &mut self,
        pair: Pair<'ctx, Rule>,
    ) -> Result<Expression<'ctx>, Box<dyn Error>> {
        let mut expression = Expression::default(self.code);
        let mut object_or_pointer = true; // true if object, false otherwise
        for token in pair.into_inner() {
            let token_span = token.as_span();
            match token.as_rule() {
                Rule::primary_expression => {
                    expression = self.build_primary_expression(token)?;
                }
                Rule::postfix_inc_op => {
                    expression = {
                        let expr_span = expression.span.clone();
                        Expression {
                            node: ExpressionEnum::Unary(
                                UnaryOperation {
                                    node: UnaryOperationEnum::PostfixIncrement,
                                    span: token.as_span(),
                                },
                                Box::new(expression),
                            ),
                            span: Span::new(self.code, expr_span.start(), token.as_span().end())
                                .unwrap(),
                        }
                    };
                }
                Rule::postfix_dec_op => {
                    expression = {
                        let expr_span = expression.span.clone();
                        Expression {
                            node: ExpressionEnum::Unary(
                                UnaryOperation {
                                    node: UnaryOperationEnum::PostfixDecrement,
                                    span: token.as_span(),
                                },
                                Box::new(expression),
                            ),
                            span: Span::new(self.code, expr_span.start(), token.as_span().end())
                                .unwrap(),
                        }
                    };
                }
                Rule::function_call => {
                    let mut arguments: Vec<Expression> = Default::default();
                    for argument_list in token.into_inner() {
                        for argument in argument_list.into_inner() {
                            arguments.push(self.build_assignment_expression(argument)?);
                        }
                    }
                    expression = {
                        let expr_span = expression.span.clone();
                        Expression {
                            node: ExpressionEnum::FunctionCall(Box::new(expression), arguments),
                            span: Span::new(self.code, expr_span.start(), token_span.end())
                                .unwrap(),
                        }
                    };
                }
                Rule::expression => {
                    let expr = self.build_expression(token)?;
                    expression = match expression.node {
                        ExpressionEnum::ArraySubscript(base, ref mut index) => {
                            index.push(expr);
                            Expression {
                                node: ExpressionEnum::ArraySubscript(base, index.to_owned()),
                                span: Span::new(
                                    self.code,
                                    expression.span.start(),
                                    token_span.end(),
                                )
                                .unwrap(),
                            }
                        }
                        _ => {
                            let expr_span = expression.span.clone();
                            Expression {
                                node: ExpressionEnum::ArraySubscript(
                                    Box::new(expression),
                                    vec![expr],
                                ),
                                span: Span::new(self.code, expr_span.start(), token_span.end())
                                    .unwrap(),
                            }
                        }
                    }
                }
                Rule::member_of_object_op => {
                    object_or_pointer = true;
                }
                Rule::member_of_pointer_op => {
                    object_or_pointer = false;
                }
                Rule::identifier => {
                    expression = {
                        let expr_span = expression.span.clone();
                        Expression {
                            node: match object_or_pointer {
                                true => ExpressionEnum::MemberOfObject(
                                    Box::new(expression),
                                    token.as_str().to_owned(),
                                ),
                                false => ExpressionEnum::MemberOfPointer(
                                    Box::new(expression),
                                    token.as_str().to_owned(),
                                ),
                            },
                            span: Span::new(self.code, expr_span.start(), token.as_span().end())
                                .unwrap(),
                        }
                    };
                }
                Rule::type_name => {
                    expression = {
                        let expr_span = expression.span.clone();
                        Expression {
                            node: ExpressionEnum::TypeCast(
                                self.build_type_name(token)?,
                                Box::new(expression),
                            ),
                            span: Span::new(self.code, expr_span.start(), token_span.end())
                                .unwrap(),
                        }
                    };
                }
                Rule::as_ => {}
                _ => unreachable!(),
            }
        }
        Ok(expression)
    }

    fn build_primary_expression(
        &mut self,
        pair: Pair<'ctx, Rule>,
    ) -> Result<Expression<'ctx>, Box<dyn Error>> {
        let span = pair.as_span();
        let token = pair.into_inner().next().unwrap();
        match token.as_rule() {
            Rule::identifier => Ok(Expression {
                node: ExpressionEnum::Identifier(token.as_str().to_owned()),
                span,
            }),
            Rule::constant => self.build_constant(token),
            Rule::string_literal => self.build_string_literal(token),
            Rule::expression => self.build_expression(token),
            _ => unreachable!(),
        }
    }

    fn build_type_name(
        &mut self,
        pair: Pair<'ctx, Rule>,
    ) -> Result<BasicType<'ctx>, Box<dyn Error>> {
        let span = pair.as_span();
        let mut fake_ast: Vec<Declaration> = Default::default();
        let mut derived_type: Type = Default::default();
        let mut dimensions: Vec<Expression> = Default::default();
        for token in pair.into_inner() {
            match token.as_rule() {
                Rule::declaration_specifiers => {
                    derived_type = self.build_declaration_specifiers(&mut fake_ast, token)?;
                }
                Rule::pointer => {
                    self.build_pointer(&mut derived_type, token)?;
                }
                Rule::function_parameter_list => {
                    self.build_function_parameter_list(&mut fake_ast, &mut derived_type, token)?;
                }
                Rule::assignment_expression => {
                    dimensions.push(self.build_assignment_expression(token)?);
                }
                _ => unreachable!(),
            }
        }
        if !dimensions.is_empty() {
            derived_type.basic_type.base_type =
                BaseType::Array(Box::new(derived_type.basic_type.to_owned()), dimensions);
            derived_type.basic_type.qualifier = Default::default();
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
}
