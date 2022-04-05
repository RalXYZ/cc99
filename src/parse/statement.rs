use pest::iterators::Pair;

use super::*;

pub fn build_statement(pair: Pair<'_, Rule>) -> Result<Statement, Box<dyn Error>> {
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

pub fn build_labeled_statement(pair: Pair<'_, Rule>) -> Result<Statement, Box<dyn Error>> {
    let mut label: String = Default::default();
    let mut statement: Statement = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::identifier => {
                label = token.as_str().to_string();
            }
            Rule::statement => {
                statement = build_statement(token)?;
            }
            _ => unreachable!(),
        }
    }
    Ok(Statement::Labeled(label, Box::new(statement)))
}

pub fn build_case_statement(pair: Pair<'_, Rule>) -> Result<Statement, Box<dyn Error>> {
    let mut expression: Option<Box<Expression>> = None;
    let mut statement: Statement = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::case_ => {}
            Rule::default_ => {}
            Rule::assignment_expression => {
                expression = Some(Box::new(build_assignment_expression(token)?));
            }
            Rule::statement => {
                statement = build_statement(token)?;
            }
            _ => unreachable!(),
        }
    }
    Ok(Statement::Case(expression, Box::new(statement)))
}

pub fn build_expression_statement(pair: Pair<'_, Rule>) -> Result<Statement, Box<dyn Error>> {
    let mut expression: Option<Box<Expression>> = None;
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::expression => {
                expression = Some(Box::new(build_expression(token)?));
            }
            _ => unreachable!(),
        }
    }
    Ok(match expression {
        Some(expr) => Statement::Expression(expr),
        None => Statement::Expression(Box::new(Expression::Empty)),
    })
}

pub fn build_compound_statement(pair: Pair<'_, Rule>) -> Result<Statement, Box<dyn Error>> {
    let mut statements: Vec<StatementOrDeclaration> = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::statement => {
                statements.push(StatementOrDeclaration::Statement(build_statement(token)?));
            }
            Rule::declaration => {
                let mut sub_ast = Vec::new();
                build_declaration(&mut sub_ast, token)?;
                for declaration in sub_ast {
                    match declaration {
                        Declaration::Declaration(declaration_type, identifier, initializer) => {
                            statements.push(StatementOrDeclaration::Declaration(
                                Declaration::Declaration(declaration_type, identifier, initializer),
                            ));
                        }
                        Declaration::FunctionDefinition(_, _, _, _) => {
                            unreachable!();
                        }
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    Ok(Statement::Compound(statements))
}

pub fn build_selection_statement(pair: Pair<'_, Rule>) -> Result<Statement, Box<dyn Error>> {
    let token = pair.into_inner().next().unwrap();
    match token.as_rule() {
        Rule::if_statement => build_if_statement(token),
        Rule::switch_statement => build_switch_statement(token),
        _ => unreachable!(),
    }
}

pub fn build_iteration_statement(pair: Pair<'_, Rule>) -> Result<Statement, Box<dyn Error>> {
    let token = pair.into_inner().next().unwrap();
    match token.as_rule() {
        Rule::for_statement => build_for_statement(token),
        Rule::while_statement => build_while_statement(token),
        Rule::do_while_statement => build_do_while_statement(token),
        _ => unreachable!(),
    }
}

pub fn build_jump_statement(pair: Pair<'_, Rule>) -> Result<Statement, Box<dyn Error>> {
    let token = pair.into_inner().next().unwrap();
    Ok(match token.as_rule() {
        Rule::break_statement => Statement::Break,
        Rule::continue_statement => Statement::Continue,
        Rule::return_statement => build_return_statement(token)?,
        Rule::goto_statement => build_goto_statement(token)?,
        _ => unreachable!(),
    })
}

pub fn build_if_statement(pair: Pair<'_, Rule>) -> Result<Statement, Box<dyn Error>> {
    let mut expression: Expression = Default::default();
    let mut statements: Vec<Statement> = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::if_ => {}
            Rule::else_ => {}
            Rule::expression => {
                expression = build_expression(token)?;
            }
            Rule::statement => {
                statements.push(build_statement(token)?);
            }
            _ => unreachable!(),
        }
    }
    Ok(Statement::If(
        Box::new(expression),
        Box::new(statements[0].to_owned()),
        match statements.len() {
            1 => None,
            2 => Some(Box::new(statements[1].to_owned())),
            _ => unreachable!(),
        },
    ))
}

pub fn build_switch_statement(pair: Pair<'_, Rule>) -> Result<Statement, Box<dyn Error>> {
    let mut expression: Expression = Default::default();
    let mut statement: Statement = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::switch_ => {}
            Rule::expression => {
                expression = build_expression(token)?;
            }
            Rule::statement => {
                statement = build_statement(token)?;
            }
            _ => unreachable!(),
        }
    }
    Ok(Statement::Switch(Box::new(expression), Box::new(statement)))
}

pub fn build_for_statement(pair: Pair<'_, Rule>) -> Result<Statement, Box<dyn Error>> {
    let mut init_clause: Option<Box<ForInitClause>> = None;
    let mut condition_expression: Option<Box<Expression>> = None;
    let mut iteration_expression: Option<Box<Expression>> = None;
    let mut statement: Statement = Default::default();

    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::for_ => {}
            Rule::for_init_clause => {
                init_clause = Some(Box::new(build_for_init_clause(token)?));
            }
            Rule::for_cond_expression => {
                condition_expression = Some(Box::new(build_expression(
                    token.into_inner().next().unwrap(),
                )?));
            }
            Rule::for_iteration_expression => {
                iteration_expression = Some(Box::new(build_expression(
                    token.into_inner().next().unwrap(),
                )?));
            }
            Rule::statement => {
                statement = build_statement(token)?;
            }
            _ => unreachable!(),
        }
    }
    Ok(Statement::For(
        init_clause,
        condition_expression,
        iteration_expression,
        Box::new(statement),
    ))
}

pub fn build_while_statement(pair: Pair<'_, Rule>) -> Result<Statement, Box<dyn Error>> {
    let mut expression: Expression = Default::default();
    let mut statement: Statement = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::while_ => {}
            Rule::expression => {
                expression = build_expression(token)?;
            }
            Rule::statement => {
                statement = build_statement(token)?;
            }
            _ => unreachable!(),
        }
    }
    Ok(Statement::While(Box::new(expression), Box::new(statement)))
}

pub fn build_do_while_statement(pair: Pair<'_, Rule>) -> Result<Statement, Box<dyn Error>> {
    let mut expression: Expression = Default::default();
    let mut statement: Statement = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::do_ => {}
            Rule::while_ => {}
            Rule::statement => {
                statement = build_statement(token)?;
            }
            Rule::expression => {
                expression = build_expression(token)?;
            }
            _ => unreachable!(),
        }
    }
    Ok(Statement::DoWhile(
        Box::new(statement),
        Box::new(expression),
    ))
}

pub fn build_return_statement(pair: Pair<'_, Rule>) -> Result<Statement, Box<dyn Error>> {
    let mut expression: Option<Box<Expression>> = None;
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::return_ => {}
            Rule::expression => {
                expression = Some(Box::new(build_expression(token)?));
            }
            _ => unreachable!(),
        }
    }
    Ok(Statement::Return(expression))
}

pub fn build_goto_statement(pair: Pair<'_, Rule>) -> Result<Statement, Box<dyn Error>> {
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
    Ok(Statement::Goto(label))
}

pub fn build_for_init_clause(pair: Pair<'_, Rule>) -> Result<ForInitClause, Box<dyn Error>> {
    let mut expression: Option<Expression> = None;
    let mut basic_type: Type = Default::default();
    let mut sub_ast = Vec::new();

    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::expression => {
                expression = Some(build_expression(token)?);
            }
            Rule::declaration_specifiers => {
                basic_type = build_declaration_specifiers(&mut sub_ast, token)?;
            }
            Rule::declarator_and_initializer_list => {
                for list_entry in token.into_inner() {
                    match list_entry.as_rule() {
                        Rule::declarator_and_initializer => {
                            build_declarator_and_initializer(
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
    Ok(match expression {
        Some(expression) => ForInitClause::Expression(expression),
        None => ForInitClause::Declaration(sub_ast),
    })
}
