use pest::iterators::Pair;

use super::*;

impl Parse {
    fn build_statement(&mut self, pair: Pair<'_, Rule>) -> Result<Statement, Box<dyn Error>> {
        let token = pair.into_inner().next().unwrap();
        match token.as_rule() {
            Rule::labeled_statement => self.build_labeled_statement(token),
            Rule::case_statement => self.build_case_statement(token),
            Rule::expression_statement => self.build_expression_statement(token),
            Rule::compound_statement => self.build_compound_statement(token),
            Rule::selection_statement => self.build_selection_statement(token),
            Rule::iteration_statement => self.build_iteration_statement(token),
            Rule::jump_statement => self.build_jump_statement(token),
            _ => unreachable!(),
        }
    }

    fn build_labeled_statement(
        &mut self,
        pair: Pair<'_, Rule>,
    ) -> Result<Statement, Box<dyn Error>> {
        let span = pair.as_span();
        let mut label: String = Default::default();
        let mut statement = Default::default();
        for token in pair.into_inner() {
            match token.as_rule() {
                Rule::identifier => {
                    label = token.as_str().to_string();
                }
                Rule::statement => {
                    statement = self.build_statement(token)?;
                }
                _ => unreachable!(),
            }
        }
        Ok(Statement {
            node: StatementEnum::Labeled(label, Box::new(statement)),
            span: Span::from(span),
        })
    }

    fn build_case_statement(&mut self, pair: Pair<'_, Rule>) -> Result<Statement, Box<dyn Error>> {
        let span = pair.as_span();
        let mut expression: Option<Box<Expression>> = None;
        let mut statement = Default::default();
        for token in pair.into_inner() {
            match token.as_rule() {
                Rule::case_ => {}
                Rule::default_ => {}
                Rule::assignment_expression => {
                    expression = Some(Box::new(self.build_assignment_expression(token)?));
                }
                Rule::statement => {
                    statement = self.build_statement(token)?;
                }
                _ => unreachable!(),
            }
        }
        Ok(Statement {
            node: StatementEnum::Case(expression, Box::new(statement)),
            span: Span::from(span),
        })
    }

    fn build_expression_statement(
        &mut self,
        pair: Pair<'_, Rule>,
    ) -> Result<Statement, Box<dyn Error>> {
        let span = pair.as_span();
        let mut expression: Option<Box<Expression>> = None;
        for token in pair.into_inner() {
            match token.as_rule() {
                Rule::expression => {
                    expression = Some(Box::new(self.build_expression(token)?));
                }
                _ => unreachable!(),
            }
        }
        Ok(Statement {
            node: match expression {
                Some(expr) => StatementEnum::Expression(expr),
                None => StatementEnum::Expression(Box::new(Expression {
                    node: ExpressionEnum::Empty,
                    span: Span::from(span.clone()),
                })),
            },
            span: Span::from(span),
        })
    }

    pub fn build_compound_statement(
        &mut self,
        pair: Pair<'_, Rule>,
    ) -> Result<Statement, Box<dyn Error>> {
        let span = pair.as_span();
        let mut statements: Vec<StatementOrDeclaration> = Default::default();
        for token in pair.into_inner() {
            let token_span = token.as_span();
            match token.as_rule() {
                Rule::statement => {
                    statements.push(StatementOrDeclaration {
                        node: StatementOrDeclarationEnum::Statement(self.build_statement(token)?),
                        span: Span::from(token_span),
                    });
                }
                Rule::declaration => {
                    let mut sub_ast = Vec::new();
                    self.build_declaration(&mut sub_ast, token)?;
                    for declaration in sub_ast {
                        match declaration.node {
                            DeclarationEnum::Declaration(
                                declaration_type,
                                identifier,
                                initializer,
                            ) => {
                                statements.push(StatementOrDeclaration {
                                    node: StatementOrDeclarationEnum::LocalDeclaration(
                                        Declaration {
                                            node: DeclarationEnum::Declaration(
                                                declaration_type,
                                                identifier,
                                                initializer,
                                            ),
                                            span: Span::from(token_span.clone()),
                                        },
                                    ),
                                    span: Span::from(token_span.clone()),
                                });
                            }
                            DeclarationEnum::FunctionDefinition(_, _, _, _, _, _, _) => {
                                unreachable!();
                            }
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        Ok(Statement {
            node: StatementEnum::Compound(statements),
            span: Span::from(span),
        })
    }

    fn build_selection_statement(
        &mut self,
        pair: Pair<'_, Rule>,
    ) -> Result<Statement, Box<dyn Error>> {
        let token = pair.into_inner().next().unwrap();
        match token.as_rule() {
            Rule::if_statement => self.build_if_statement(token),
            Rule::switch_statement => self.build_switch_statement(token),
            _ => unreachable!(),
        }
    }

    fn build_iteration_statement(
        &mut self,
        pair: Pair<'_, Rule>,
    ) -> Result<Statement, Box<dyn Error>> {
        let token = pair.into_inner().next().unwrap();
        match token.as_rule() {
            Rule::for_statement => self.build_for_statement(token),
            Rule::while_statement => self.build_while_statement(token),
            Rule::do_while_statement => self.build_do_while_statement(token),
            _ => unreachable!(),
        }
    }

    fn build_jump_statement(&mut self, pair: Pair<'_, Rule>) -> Result<Statement, Box<dyn Error>> {
        let token = pair.into_inner().next().unwrap();
        Ok(match token.as_rule() {
            Rule::break_statement => Statement {
                node: StatementEnum::Break,
                span: Span::from(token.as_span()),
            },
            Rule::continue_statement => Statement {
                node: StatementEnum::Continue,
                span: Span::from(token.as_span()),
            },
            Rule::return_statement => self.build_return_statement(token)?,
            Rule::goto_statement => self.build_goto_statement(token)?,
            _ => unreachable!(),
        })
    }

    fn build_if_statement(&mut self, pair: Pair<'_, Rule>) -> Result<Statement, Box<dyn Error>> {
        let span = pair.as_span();
        let mut expression = Default::default();
        let mut statements: Vec<Statement> = Default::default();
        for token in pair.into_inner() {
            match token.as_rule() {
                Rule::if_ => {}
                Rule::else_ => {}
                Rule::expression => {
                    expression = self.build_expression(token)?;
                }
                Rule::statement => {
                    statements.push(self.build_statement(token)?);
                }
                _ => unreachable!(),
            }
        }
        Ok(Statement {
            node: StatementEnum::If(
                Box::new(expression),
                Box::new(statements[0].to_owned()),
                match statements.len() {
                    1 => None,
                    2 => Some(Box::new(statements[1].to_owned())),
                    _ => unreachable!(),
                },
            ),
            span: Span::from(span),
        })
    }

    fn build_switch_statement(
        &mut self,
        pair: Pair<'_, Rule>,
    ) -> Result<Statement, Box<dyn Error>> {
        let span = pair.as_span();
        let mut expression = Default::default();
        let mut statement = Default::default();
        for token in pair.into_inner() {
            match token.as_rule() {
                Rule::switch_ => {}
                Rule::expression => {
                    expression = self.build_expression(token)?;
                }
                Rule::statement => {
                    statement = self.build_statement(token)?;
                }
                _ => unreachable!(),
            }
        }
        Ok(Statement {
            node: StatementEnum::Switch(Box::new(expression), Box::new(statement)),
            span: Span::from(span),
        })
    }

    fn build_for_statement(&mut self, pair: Pair<'_, Rule>) -> Result<Statement, Box<dyn Error>> {
        let span = pair.as_span();
        let mut init_clause: Option<Box<ForInitClause>> = None;
        let mut condition_expression: Option<Box<Expression>> = None;
        let mut iteration_expression: Option<Box<Expression>> = None;
        let mut statement = Default::default();

        for token in pair.into_inner() {
            match token.as_rule() {
                Rule::for_ => {}
                Rule::for_init_clause => {
                    init_clause = Some(Box::new(self.build_for_init_clause(token)?));
                }
                Rule::for_cond_expression => {
                    condition_expression = Some(Box::new(
                        self.build_expression(token.into_inner().next().unwrap())?,
                    ));
                }
                Rule::for_iteration_expression => {
                    iteration_expression = Some(Box::new(
                        self.build_expression(token.into_inner().next().unwrap())?,
                    ));
                }
                Rule::statement => {
                    statement = self.build_statement(token)?;
                }
                _ => unreachable!(),
            }
        }
        Ok(Statement {
            node: StatementEnum::For(
                init_clause,
                condition_expression,
                iteration_expression,
                Box::new(statement),
            ),
            span: Span::from(span),
        })
    }

    fn build_while_statement(&mut self, pair: Pair<'_, Rule>) -> Result<Statement, Box<dyn Error>> {
        let span = pair.as_span();
        let mut expression = Default::default();
        let mut statement = Default::default();
        for token in pair.into_inner() {
            match token.as_rule() {
                Rule::while_ => {}
                Rule::expression => {
                    expression = self.build_expression(token)?;
                }
                Rule::statement => {
                    statement = self.build_statement(token)?;
                }
                _ => unreachable!(),
            }
        }
        Ok(Statement {
            node: StatementEnum::While(Box::new(expression), Box::new(statement)),
            span: Span::from(span),
        })
    }

    fn build_do_while_statement(
        &mut self,
        pair: Pair<'_, Rule>,
    ) -> Result<Statement, Box<dyn Error>> {
        let span = pair.as_span();
        let mut expression = Default::default();
        let mut statement = Default::default();
        for token in pair.into_inner() {
            match token.as_rule() {
                Rule::do_ => {}
                Rule::while_ => {}
                Rule::statement => {
                    statement = self.build_statement(token)?;
                }
                Rule::expression => {
                    expression = self.build_expression(token)?;
                }
                _ => unreachable!(),
            }
        }
        Ok(Statement {
            node: StatementEnum::DoWhile(Box::new(statement), Box::new(expression)),
            span: Span::from(span),
        })
    }

    fn build_return_statement(
        &mut self,
        pair: Pair<'_, Rule>,
    ) -> Result<Statement, Box<dyn Error>> {
        let span = pair.as_span();
        let mut expression: Option<Box<Expression>> = None;
        for token in pair.into_inner() {
            match token.as_rule() {
                Rule::return_ => {}
                Rule::expression => {
                    expression = Some(Box::new(self.build_expression(token)?));
                }
                _ => unreachable!(),
            }
        }
        Ok(Statement {
            node: StatementEnum::Return(expression),
            span: Span::from(span),
        })
    }

    fn build_goto_statement(&mut self, pair: Pair<'_, Rule>) -> Result<Statement, Box<dyn Error>> {
        let span = pair.as_span();
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
        Ok(Statement {
            node: StatementEnum::Goto(label),
            span: Span::from(span),
        })
    }

    fn build_for_init_clause(
        &mut self,
        pair: Pair<'_, Rule>,
    ) -> Result<ForInitClause, Box<dyn Error>> {
        let span = pair.as_span();
        let mut expression: Option<Expression> = None;
        let mut basic_type: Type = Default::default();
        let mut sub_ast = Vec::new();

        for token in pair.into_inner() {
            match token.as_rule() {
                Rule::expression => {
                    expression = Some(self.build_expression(token)?);
                }
                Rule::declaration_specifiers => {
                    basic_type = self.build_declaration_specifiers(&mut sub_ast, token)?;
                }
                Rule::declarator_and_initializer_list => {
                    for list_entry in token.into_inner() {
                        match list_entry.as_rule() {
                            Rule::declarator_and_initializer => {
                                self.build_declarator_and_initializer(
                                    &mut sub_ast,
                                    list_entry,
                                    &basic_type,
                                )?;
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        Ok(ForInitClause {
            node: match expression {
                Some(expression) => ForInitClauseEnum::Expression(expression),
                None => ForInitClauseEnum::ForDeclaration(sub_ast),
            },
            span: Span::from(span),
        })
    }
}
