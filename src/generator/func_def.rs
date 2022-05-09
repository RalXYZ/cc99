use std::collections::HashMap;
use crate::ast::{BasicType, Statement, Declaration, StatementOrDeclaration, BaseType};
use crate::Generator;
use anyhow::Result;
use inkwell::values::{BasicValue, PointerValue};
use crate::utils::CompileErr as CE;

impl<'ctx> Generator<'ctx> {
    pub fn gen_func_def(
        &mut self,
        return_type: &BasicType,
        func_name: &String,
        func_param: &Vec<(BasicType, Option<String>)>,
        func_body: &Statement,
    ) -> Result<()> {
        let func = self.module.get_function(func_name.as_str()).unwrap();
        self.val_map_block_stack.push(HashMap::new());

        let func_ty = self.function_map.get(func_name).unwrap().to_owned();
        self.current_function = Some((func, func_ty.0));

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
                func_param[i].0.base_type.to_llvm_type(self.context),
                func_param[i].1.as_ref().unwrap_or(
                    &("__param__".to_string() + func_name + &i.to_string())
                ).as_str()
            );

            if func_param[i].1.is_some() {
                self.insert_to_val_map(&func_param[i].0, &func_param[i].1.as_ref().unwrap(), alloca)?;
            }
        }

        // generate IR for each statement or declaration in function body
        if let Statement::Compound(state_or_decl) = &func_body {
            for element in state_or_decl {
                match element {
                    StatementOrDeclaration::Statement(state) => {
                        self.gen_state(state)?;
                    },
                    StatementOrDeclaration::LocalDeclaration(decl) => {
                        self.gen_local_decl(decl)?;
                    },
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

        self.val_map_block_stack.pop();
        self.current_function = None;
        return Ok(())
    }

    fn insert_to_val_map(
        &mut self,
        var_type: &BasicType,
        identifier: &String,
        ptr: PointerValue<'ctx>
    ) -> Result<()> {
        let local_map = self.val_map_block_stack.last_mut().unwrap();

        if local_map.contains_key(identifier) {
            return Err(CE::DuplicatedVariable(identifier.to_string()).into());
        }

        local_map.insert(identifier.to_string(), (var_type.clone(), ptr));
        Ok(())
    }

    fn gen_state(&mut self, state: &Statement) -> Result<()> {
        unimplemented!()
    }

    fn gen_local_decl(&mut self, decl: &Declaration) -> Result<()> {
        unimplemented!()
    }
}