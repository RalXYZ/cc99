use std::collections::{HashMap, VecDeque};
use std::ops::Deref;
use std::path::Path;
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicValue, BasicValueEnum, FunctionValue, PointerValue};
use anyhow::Result;
use crate::ast::IntegerType;
use crate::ast::{AST, BaseType, BasicType, Declaration, Expression, Statement, Type};
use crate::utils::CompileErr;

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
                        identifier.as_ref().unwrap(),
                        params_type
                    )
                } else {
                    self.gen_global_variable(
                        type_info,
                        identifier.as_ref().unwrap(),
                        initializer,
                    )
                }
            })
            .collect::<Result<()>>()
    }

    // FIXME: implement this
    fn gen_function_proto(
        &mut self,
        ret_type: &Box<BasicType>,
        func_name: &String,
        func_param: &Vec<BasicType>
    ) -> Result<()> {
        unimplemented!();
    }

    // FIXME: implement this
    fn gen_function_def(
        &mut self,
        func_type: &Type,
        func_name: &String,
        func_param: &Vec<Option<String>>,
        func_body: &Statement
    ) -> Result<()> {
        unimplemented!();
    }

    // FIXME: implement this
    fn cast_value(&self,
                  cur_ty: &BaseType,
                  cur_val: &BasicValueEnum<'ctx>,
                  cast_ty: &BaseType,
    ) -> Result<BasicValueEnum<'ctx>> {
        unimplemented!();
    }

    // FIXME: implement this
    fn get_variable(&self, identifier: &String) -> Result<(BasicType, PointerValue<'ctx>)> {
        unimplemented!();
    }

    fn gen_global_variable(
        &mut self,
        var_type: &Type,
        var_name: &String,
        ptr_to_init: &Option<Box<Expression>>
    ) -> Result<()> {
        if self.global_variable_map.contains_key(var_name) {
            return Err(CompileErr::DuplicatedGlobalVariable(var_name.to_string()).into());
        } else if self.function_map.contains_key(var_name) {
            return Err(CompileErr::DuplicatedSymbol(var_name.to_string()).into());
        }

        let global_value = self.module.add_global(
            var_type.basic_type.base_type.to_llvm_type(self.context),
            None,
            var_name.as_str(),
        );

        // if ptr_to_init is not None
        if let Some(ptr_to_init) = ptr_to_init {
            let init_val_pair = self.gen_expression(&**ptr_to_init)?;
            let cast_ty = init_val_pair.0.default_cast(&var_type.basic_type.base_type)?;
            let cast_v = self.cast_value(&init_val_pair.0, &init_val_pair.1, &cast_ty)?;

            global_value.set_initializer(&cast_v);
        }

        self.global_variable_map.insert(
            var_name.to_string(),
            (var_type.basic_type.to_owned(), global_value.as_pointer_value()),
        );

        Ok(())
    }

    fn gen_expression(&self, expr: &Expression) -> Result<(BaseType, BasicValueEnum<'ctx>)> {
        match expr {
            Expression::CharacterConstant(ref value) => {
                Ok((
                    BaseType::SignedInteger(IntegerType::Char),
                    self.context
                        .i8_type()
                        .const_int(
                            *value as u64,
                            false,
                        )
                        .as_basic_value_enum(),
                ))
            },
            Expression::IntegerConstant(ref value) => {
                Ok((
                    BaseType::SignedInteger(IntegerType::Int),
                    self.context
                        .i32_type()
                        .const_int(
                            *value as u64,
                            false,
                        )
                        .as_basic_value_enum(),
                ))
            },
            Expression::UnsignedIntegerConstant(ref value) => {
                Ok((
                    BaseType::UnsignedInteger(IntegerType::Int),
                    self.context
                        .i32_type()
                        .const_int(
                            *value as u64,
                            false,
                        )
                        .as_basic_value_enum(),
                ))
            },
            Expression::UnsignedLongConstant(ref value) |
            Expression::UnsignedLongLongConstant(ref value) => {
                Ok((
                    BaseType::UnsignedInteger(IntegerType::Long),
                    self.context
                        .i64_type()
                        .const_int(
                            *value as u64,
                            false,
                        )
                        .as_basic_value_enum(),
                ))
            },
            Expression::FloatConstant(ref value) => {
                Ok((
                    BaseType::Float,
                    self.context
                        .f32_type()
                        .const_float(
                            *value as f64,
                        )
                        .as_basic_value_enum(),
                ))
            },
            Expression::DoubleConstant(ref value) => {
                Ok((
                    BaseType::Float,
                    self.context
                        .f64_type()
                        .const_float(
                            *value as f64,
                        )
                        .as_basic_value_enum(),
                ))
            },
            Expression::Identifier(ref string_literal) => {
                let deref = self.get_variable(string_literal)?;
                let val = self.builder.build_load(deref.1, "load");
                Ok((deref.0.base_type, val))
            },
            Expression::StringLiteral(ref string) => {
                Ok((
                    BaseType::Pointer(Box::new(BasicType{
                        qualifier: vec![],
                        base_type: BaseType::SignedInteger(IntegerType::Char),
                    })),
                    self.builder
                        .build_global_string_ptr(string.as_str(), "str")
                        .as_basic_value_enum(),
                ))
            },
            _ => { return Err(CompileErr::UnknownExpression(expr.to_string()).into()); }
        }
    }
}