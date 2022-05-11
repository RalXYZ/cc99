use std::collections::HashMap;
use anyhow::Result;
use pest::consumes_to;
use crate::generator::Generator;
use crate::ast::{Expression, ForInitClause, Statement, StatementOrDeclaration};
use crate::utils::CompileErr as CE;

impl<'ctx> Generator<'ctx> {
    pub(crate) fn gen_statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::Compound(state_or_decl) =>
                self.gen_compound_statement(state_or_decl)?,
            Statement::While(cond, body) =>
                self.gen_while_statement(cond, body, false)?,
            Statement::DoWhile(body, cond) =>
                self.gen_while_statement(cond, body, true)?,
            Statement::For(init, cond, iter, body) =>
                self.gen_for_statement(init, cond, iter, body)?,
            Statement::Break =>
                self.gen_break_statement()?,
            Statement::Continue =>
                self.gen_continue_statement()?,
            Statement::If(cond, then_stmt, else_stmt) =>
                self.gen_if_statement(cond, then_stmt, else_stmt)?,
            Statement::Return(expr) =>
                self.gen_return_statement(expr)?,
            _ =>
                unimplemented!()
        };
        Ok(())
    }

    fn gen_compound_statement(&mut self, statements: &Vec<StatementOrDeclaration>) -> Result<()> {
        self.val_map_block_stack.push(HashMap::new());

        // generate IR for each statement or declaration in function body
        for element in statements {
            match element {
                StatementOrDeclaration::Statement(state) => {
                    self.gen_statement(state)?;
                },
                StatementOrDeclaration::LocalDeclaration(decl) => {
                    self.gen_decl_in_fn(decl)?;
                },
            }
        }

        self.val_map_block_stack.pop();
        Ok(())
    }

    fn gen_while_statement(&mut self, cond: &Expression, body: &Statement, is_do_while: bool) -> Result<()> {
        let func_val = self.current_function.as_ref().unwrap().0;

        let before_while_block =
            self.context.append_basic_block(func_val, "before_while");
        let while_block =
            self.context.append_basic_block(func_val, if is_do_while { "do_while" } else { "while" });
        let after_while_block =
            self.context.append_basic_block(func_val, "after_loop");

        self.continue_labels.push_back(after_while_block);
        self.break_labels.push_back(after_while_block);

        self.builder.build_unconditional_branch(before_while_block);
        self.builder.position_at_end(before_while_block);
        let condition_val_int_val = self.gen_expression(cond)?.1.into_int_value();
        if is_do_while {
            self.builder.build_unconditional_branch(while_block);
        } else {
            self.builder.build_conditional_branch(condition_val_int_val,while_block, after_while_block);
        }

        self.builder.position_at_end(while_block);

        // body must be Statement::Compound
        self.gen_statement(body)?;
        // FIXME: maybe we need to add if self.no_terminator()
        if is_do_while {
            self.builder.build_conditional_branch(condition_val_int_val, before_while_block, after_while_block);
        } else {
            self.builder.build_unconditional_branch(before_while_block);
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
    ) -> Result<()> {
        unimplemented!()
    }

    fn gen_break_statement(&mut self) -> Result<()> {
        if self.break_labels.is_empty() {
            return Err(CE::KeywordNotInLoop("break".to_string()).into());
        }
        let break_block = self.break_labels.back().unwrap();
        self.builder.build_unconditional_branch(*break_block);
        Ok(())
    }

    fn gen_continue_statement(&mut self) -> Result<()> {
        if self.continue_labels.is_empty() {
            return Err(CE::KeywordNotInLoop("continue".to_string()).into());
        }
        let continue_block = self.continue_labels.back().unwrap();
        self.builder.build_unconditional_branch(*continue_block);
        Ok(())
    }

    fn gen_if_statement(
        &mut self,
        cond: &Box<Expression>,
        then_stmt: &Box<Statement>,
        else_stmt: &Option<Box<Statement>>,
    ) -> Result<()> {
        let func_val = self.current_function.as_ref().unwrap().0;

        let if_block = self.context.append_basic_block(func_val, "if_block");
        let else_block = self.context.append_basic_block(func_val, "else_block");
        let after_block = self.context.append_basic_block(func_val, "after_block");

        let cond_int_value = self.gen_expression(cond)?.1.into_int_value();
        self.builder.build_conditional_branch(cond_int_value, if_block, else_block);

        self.builder.position_at_end(if_block);
        self.gen_statement(then_stmt)?;
        // FIXME: maybe we need to add if self.no_terminator()
        self.builder.build_unconditional_branch(after_block);

        if let Some(ref else_stmt) = *else_stmt {
            self.builder.position_at_end(else_block);
            self.gen_statement(else_stmt)?;
            // FIXME: maybe we need to add if self.no_terminator()
            self.builder.build_unconditional_branch(after_block);
        }

        self.builder.position_at_end(after_block);

        Ok(())
    }

    fn gen_return_statement(&mut self, expr: &Option<Box<Expression>>) -> Result<()> {
        // if expr.is_none() {
        //     self.builder.build_return(None);
        //     return Ok(());
        // }
        //
        // let expr = expr.unwrap()
        //
        // let func_type = self.current_function.as_ref().unwrap().1;
        //
        // Ok(())
        unimplemented!()
    }
}
