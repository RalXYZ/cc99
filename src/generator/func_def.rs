use crate::ast::{
    AssignOperation, AssignOperationEnum, BaseType, BasicType, Declaration, DeclarationEnum,
    Expression, ExpressionEnum, Span, Statement, StatementEnum, StatementOrDeclarationEnum,
};
use crate::generator::Generator;
use crate::utils::CompileErr as CE;
use inkwell::values::{BasicValue, PointerValue};
use std::collections::HashMap;

impl<'ctx> Generator<'ctx> {
    pub(crate) fn gen_func_def(
        &mut self,
        return_type: &BasicType,
        func_name: &String,
        func_param: &Vec<(BasicType, Option<String>)>,
        func_body: &Statement,
        span: Span,
    ) -> Result<(), CE> {
        let func = self.module.get_function(func_name.as_str()).unwrap();
        self.val_map_block_stack.push(HashMap::new());

        let func_ty = self.function_map.get(func_name).unwrap().to_owned();
        self.current_function = Some((func, func_ty.0));

        let mut func_param_alloca = Vec::new();

        // create function block
        let func_block = self.context.append_basic_block(func, "entry");
        self.builder.position_at_end(func_block);

        for (i, param) in func.get_param_iter().enumerate() {
            // TODO: validate param type

            if func_param[i].1.is_some() {
                param.set_name(func_param[i].1.as_ref().unwrap().as_str());
            }

            let builder = self.context.create_builder();
            let func_entry = func.get_first_basic_block().unwrap();

            match func_entry.get_first_instruction() {
                Some(first_inst) => builder.position_before(&first_inst),
                None => builder.position_at_end(func_entry),
            }

            let alloca = builder.build_alloca(
                self.convert_llvm_type(&func_param[i].0.base_type),
                func_param[i]
                    .1
                    .as_ref()
                    .unwrap_or(&("__param__".to_string() + func_name + &i.to_string()))
                    .as_str(),
            );

            func_param_alloca.push(alloca);

            if func_param[i].1.is_some() {
                self.insert_to_val_map(
                    &func_param[i].0,
                    &func_param[i].1.as_ref().unwrap(),
                    alloca,
                    span,
                )?;
            }
        }

        // store params on the stack
        for (i, param) in func.get_param_iter().enumerate() {
            self.builder.build_store(func_param_alloca[i], param);
        }

        // generate IR for each statement or declaration in function body
        if let StatementEnum::Compound(ref state_or_decl) = func_body.node {
            for element in state_or_decl {
                match element.node {
                    StatementOrDeclarationEnum::Statement(ref state) => {
                        self.gen_statement(state)?;
                    }
                    StatementOrDeclarationEnum::LocalDeclaration(ref decl) => {
                        self.gen_decl_in_fn(decl)?;
                    }
                }
            }
        } else {
            panic!("internal error: func_body is not Statement::Compound");
        }

        // build terminator for any block that is not terminated
        let mut iter_block = func.get_first_basic_block();
        while iter_block.is_some() {
            let block = iter_block.unwrap();
            if block.get_terminator().is_none() {
                let terminator_builder = self.context.create_builder();
                terminator_builder.position_at_end(block);
                match return_type.base_type {
                    BaseType::Void => {
                        terminator_builder.build_return(None);
                    }
                    _ => {
                        let null_val = self.context.i32_type().const_zero();
                        terminator_builder.build_return(Some(&null_val));
                    }
                }
            }
            iter_block = block.get_next_basic_block();
        }

        if !func.verify(true) {
            func.print_to_stderr();
            panic!()
        }

        self.val_map_block_stack.pop();
        self.current_function = None;
        return Ok(());
    }

    fn insert_to_val_map(
        &mut self,
        var_type: &BasicType,
        identifier: &String,
        ptr: PointerValue<'ctx>,
        span: Span,
    ) -> Result<(), CE> {
        let local_map = self.val_map_block_stack.last_mut().unwrap();

        if local_map.contains_key(identifier) {
            return Err(CE::duplicated_variable(identifier.to_string(), span).into());
        }

        local_map.insert(identifier.to_string(), (var_type.clone(), ptr));
        Ok(())
    }

    pub(crate) fn gen_decl_in_fn(&mut self, decl: &Declaration) -> Result<(), CE> {
        if let DeclarationEnum::Declaration(ref var_type, ref identifier, ref expr) = decl.node {
            let llvm_type = self.convert_llvm_type(&var_type.basic_type.base_type);
            let p_val = self
                .builder
                .build_alloca(llvm_type, &identifier.to_owned().unwrap());
            self.insert_to_val_map(
                &var_type.basic_type,
                &identifier.to_owned().unwrap(),
                p_val,
                decl.span,
            )?;
            if let Some(ref expr) = expr {
                self.gen_assignment(
                    &AssignOperation {
                        node: AssignOperationEnum::Naive,
                        span: expr.span.clone(),
                    },
                    &Box::new(Expression {
                        node: ExpressionEnum::Identifier(identifier.to_owned().unwrap()),
                        span: expr.span.clone(),
                    }),
                    expr,
                    decl.span,
                )?;
            }
            Ok(())
        } else {
            Err(CE::plain_error(
                "FunctionDefinition cannot exist in function".to_string(),
                decl.span,
            )
            .into())
        }
    }
}
