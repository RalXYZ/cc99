use crate::ast::{
    Expression, ExpressionEnum, ForInitClause, ForInitClauseEnum, Span, Statement, StatementEnum,
    StatementOrDeclaration, StatementOrDeclarationEnum,
};
use crate::generator::Generator;
use crate::utils::CompileErr as CE;
use std::collections::HashMap;

impl<'ctx> Generator<'ctx> {
    pub(crate) fn gen_statement(&mut self, statement: &Statement) -> Result<(), CE> {
        match statement.node {
            StatementEnum::Compound(ref state_or_decl) => {
                self.gen_compound_statement(state_or_decl)?
            }
            StatementEnum::While(ref cond, ref body) => {
                self.gen_while_statement(cond, body, false)?
            }
            StatementEnum::DoWhile(ref body, ref cond) => {
                self.gen_while_statement(cond, body, true)?
            }
            StatementEnum::For(ref init, ref cond, ref iter, ref body) => {
                self.gen_for_statement(init, cond, iter, body)?
            }
            StatementEnum::Break => self.gen_break_statement(statement.span)?,
            StatementEnum::Continue => self.gen_continue_statement(statement.span)?,
            StatementEnum::If(ref cond, ref then_stmt, ref else_stmt) => {
                self.gen_if_statement(cond, then_stmt, else_stmt)?
            }
            StatementEnum::Return(ref expr) => self.gen_return_statement(expr)?,
            StatementEnum::Expression(ref expr) => {
                self.gen_expression(expr)?;
            }
            _ => {
                dbg!(statement);
                unimplemented!()
            }
        };
        Ok(())
    }

    fn gen_compound_statement(&mut self, statements: &[StatementOrDeclaration]) -> Result<(), CE> {
        self.val_map_block_stack.push(HashMap::new());

        // generate IR for each statement or declaration in function body
        for element in statements {
            match element.node {
                StatementOrDeclarationEnum::Statement(ref state) => {
                    self.gen_statement(state)?;
                }
                StatementOrDeclarationEnum::LocalDeclaration(ref decl) => {
                    self.gen_decl_in_fn(decl)?;
                }
            }
        }

        self.val_map_block_stack.pop();
        Ok(())
    }

    fn gen_while_statement(
        &mut self,
        cond: &Expression,
        body: &Statement,
        is_do_while: bool,
    ) -> Result<(), CE> {
        let func_val = self.current_function.as_ref().unwrap().0;

        let before_while_block = self.context.append_basic_block(func_val, "before_while");
        let while_block = self
            .context
            .append_basic_block(func_val, if is_do_while { "do_while" } else { "while" });
        let after_while_block = self.context.append_basic_block(func_val, "after_loop");

        self.continue_labels.push_back(after_while_block);
        self.break_labels.push_back(after_while_block);

        self.builder.build_unconditional_branch(before_while_block);
        self.builder.position_at_end(before_while_block);
        let condition_val_int_val = self.gen_expression(cond)?.1.into_int_value();
        if self.no_terminator() {
            if is_do_while {
                self.builder.build_unconditional_branch(while_block);
            } else {
                self.builder.build_conditional_branch(
                    condition_val_int_val,
                    while_block,
                    after_while_block,
                );
            }
        }

        self.builder.position_at_end(while_block);

        // body must be Statement::Compound
        self.gen_statement(body)?;
        if self.no_terminator() {
            if is_do_while {
                self.builder.build_conditional_branch(
                    condition_val_int_val,
                    before_while_block,
                    after_while_block,
                );
            } else {
                self.builder.build_unconditional_branch(before_while_block);
            }
        }

        self.builder.position_at_end(after_while_block);

        self.break_labels.pop_back();
        self.continue_labels.pop_back();

        Ok(())
    }

    fn gen_for_statement(
        &mut self,
        init: &Option<Box<ForInitClause>>,
        cond: &Option<Box<Expression>>,
        iter: &Option<Box<Expression>>,
        body: &Statement,
    ) -> Result<(), CE> {
        let mut new_block: Vec<StatementOrDeclaration> = vec![];
        if let Some(ref init) = init {
            match &init.node {
                ForInitClauseEnum::Expression(ref expr) => {
                    new_block.push(StatementOrDeclaration {
                        node: StatementOrDeclarationEnum::Statement(Statement {
                            node: StatementEnum::Expression(Box::new(expr.to_owned())),
                            span: init.span,
                        }),
                        span: init.span,
                    });
                }
                ForInitClauseEnum::ForDeclaration(decl) => {
                    new_block.append(
                        decl.iter()
                            .map(|d| StatementOrDeclaration {
                                node: StatementOrDeclarationEnum::LocalDeclaration(d.to_owned()),
                                span: d.span,
                            })
                            .collect::<Vec<StatementOrDeclaration>>()
                            .as_mut(),
                    );
                }
            }
        }
        let mut new_body = vec![StatementOrDeclaration {
            node: StatementOrDeclarationEnum::Statement(body.to_owned()),
            span: body.span,
        }];
        if let Some(iter) = iter {
            new_body.push(StatementOrDeclaration {
                node: StatementOrDeclarationEnum::Statement(Statement {
                    node: StatementEnum::Expression(iter.to_owned()),
                    span: iter.span,
                }),
                span: iter.span,
            });
        }
        let new_cond = match cond {
            Some(cond) => cond.to_owned(),
            None => Box::new(Expression {
                node: ExpressionEnum::Empty,
                span: Span::default(),
            }),
        };
        new_block.push(StatementOrDeclaration {
            node: StatementOrDeclarationEnum::Statement(Statement {
                node: StatementEnum::While(
                    new_cond.clone(),
                    Box::new(Statement {
                        node: StatementEnum::Compound(new_body),
                        span: body.span,
                    }),
                ),
                span: new_cond.span,
            }),
            span: new_cond.span,
        });
        self.gen_compound_statement(&new_block)?;
        Ok(())
    }

    fn gen_break_statement(&mut self, span: Span) -> Result<(), CE> {
        if self.break_labels.is_empty() {
            return Err(CE::keyword_not_in_a_loop("break".to_string(), span));
        }
        let break_block = self.break_labels.back().unwrap();
        self.builder.build_unconditional_branch(*break_block);
        Ok(())
    }

    fn gen_continue_statement(&mut self, span: Span) -> Result<(), CE> {
        if self.continue_labels.is_empty() {
            return Err(CE::keyword_not_in_a_loop("continue".to_string(), span));
        }
        let continue_block = self.continue_labels.back().unwrap();
        self.builder.build_unconditional_branch(*continue_block);
        Ok(())
    }

    fn gen_if_statement(
        &mut self,
        cond: &Expression,
        then_stmt: &Statement,
        else_stmt: &Option<Box<Statement>>,
    ) -> Result<(), CE> {
        let func_val = self.current_function.as_ref().unwrap().0;

        let if_block = self.context.append_basic_block(func_val, "if_block");
        let else_block = self.context.append_basic_block(func_val, "else_block");
        let after_block = self.context.append_basic_block(func_val, "after_block");

        let cond_int_value = self.gen_expression(cond)?.1.into_int_value();
        self.builder
            .build_conditional_branch(cond_int_value, if_block, else_block);

        self.builder.position_at_end(if_block);
        self.gen_statement(then_stmt)?;
        if self.no_terminator() {
            self.builder.build_unconditional_branch(after_block);
        };

        self.builder.position_at_end(else_block);
        if let Some(ref else_stmt) = *else_stmt {
            self.gen_statement(else_stmt)?;
        }
        if self.no_terminator() {
            self.builder.build_unconditional_branch(after_block);
        }

        self.builder.position_at_end(after_block);

        Ok(())
    }

    fn gen_return_statement(&mut self, expr: &Option<Box<Expression>>) -> Result<(), CE> {
        if expr.is_none() {
            self.builder.build_return(None);
            return Ok(());
        }

        let func_return_type = self
            .current_function
            .as_ref()
            .unwrap()
            .to_owned()
            .1
            .base_type;
        let (e_t, e_v) = self.gen_expression(&expr.to_owned().unwrap())?;

        e_t.test_cast(
            &func_return_type,
            expr.as_ref().unwrap().span,
            &self.typedef_map,
        )?;

        let return_val =
            self.cast_value(&e_t, &e_v, &func_return_type, expr.as_ref().unwrap().span)?;
        self.builder.build_return(Some(&return_val));

        Ok(())
    }
}
