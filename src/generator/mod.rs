use std::collections::{HashMap, VecDeque};
use std::ops::Deref;
use std::path::Path;
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{FunctionValue, PointerValue};
use anyhow::Result;
use crate::ast::{AST, BaseType, BasicType, Declaration, Expression, Statement, Type};

pub struct Generator<'ctx> {
    source_path: &'ctx str,
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,

    //>>>>>>>>>>>>>>>>>>>>>>>>
    //      LLVM Blocks
    //<<<<<<<<<<<<<<<<<<<<<<<<

    addr_map_stack: Vec<HashMap<String, (BasicType, PointerValue<'ctx>)>>,
    // current function block
    current_function: Option<(FunctionValue<'ctx>, Option<BasicType>)>,
    // break labels (in loop statements)
    break_labels: VecDeque<BasicBlock<'ctx>>,
    // continue labels (in loop statements)
    continue_labels: VecDeque<BasicBlock<'ctx>>,
    // hashset for functions
    function_map: HashMap<String, (Option<BasicType>, Vec<BasicType>)>,
    // hashset for global variable
    global_variable_map: HashMap<String, (BasicType, PointerValue<'ctx>)>,
}

impl<'ctx> Generator<'ctx> {
    // new LLVM context
    pub fn new(context: &'ctx Context, source_path: &'ctx str) -> Generator<'ctx> {
        let module_name = Path::new(source_path).file_stem().unwrap().to_str().unwrap().to_string();
        let module = context.create_module(module_name.as_str());
        let builder = context.create_builder();

        // set variable scope
        let mut addr_map_stack = Vec::new();
        let global_map: HashMap<String, (BasicType, PointerValue<'ctx>)> = HashMap::new();
        addr_map_stack.push(global_map); // push global variable hashmap

        Generator { // return value
            source_path,
            // module_name,
            context,
            module,
            builder,
            addr_map_stack,
            current_function: None,
            break_labels: VecDeque::new(),
            continue_labels: VecDeque::new(),
            function_map: HashMap::new(),
            global_variable_map: HashMap::new(),
        }
    }

    // first-time scanning
    pub fn gen(&mut self, ast: &Box<AST>) -> Result<()> {
        let AST::GlobalDeclaration(ref declarations) = ast.deref();
        declarations.iter()
            .filter_map(|declaration| {
                if let Declaration::Declaration(
                    ref type_info,
                    ref identifier,
                    ref initializer,
                ) = declaration {
                    Some((type_info, identifier, initializer))
                } else {
                    None
                }
            })
            .map(|(type_info, identifier, initializer)| {
                if let BaseType::Function(
                    ref return_type,
                    ref params_type,
                    ref param_identifier,
                ) = type_info.basic_type.base_type {
                    self.gen_function_proto(
                        return_type,
                        identifier,
                        params_type
                    )
                } else {
                    self.gen_global_variable(
                        type_info,
                        identifier,
                        initializer,
                    )
                }
            })
            .collect::<Result<()>>()
    }

    fn gen_function_proto(
        &mut self,
        ret_type: &Box<BasicType>,
        func_name: &Option<String>,
        func_param: &Vec<BasicType>
    ) -> Result<()> {
        Ok(())
    }

    fn gen_function_def(
        &mut self,
        func_type: &Type,
        func_name: &Option<String>,
        func_param: &Vec<Option<String>>,
        func_body: &Statement
    ) -> Result<()> {
        Ok(())
    }
    fn gen_global_variable(
        &mut self,
        var_type: &Type,
        var_name: &Option<String>,
        ptr_to_init: &Option<Box<Expression>>
    ) -> Result<()> {
        Ok(())
    }
}