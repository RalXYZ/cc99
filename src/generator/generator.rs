use std::collections::{HashMap, VecDeque};
use std::ops::Deref;
use std::path::Path;
use inkwell::context::Context;
use inkwell::values::{BasicValue, BasicValueEnum, PointerValue};
use anyhow::Result;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType};
use crate::ast::IntegerType;
use crate::ast::{AST, BaseType, BasicType as CC99BasicTYpe, Declaration, Expression, Statement, Type};
use crate::Generator;
use crate::utils::CompileErr;

impl<'ctx> Generator<'ctx> {
    // new LLVM context
    pub fn new(context: &'ctx Context, source_path: &'ctx str) -> Generator<'ctx> {
        let module_name = Path::new(source_path).file_stem().unwrap().to_str().unwrap().to_string();
        let module = context.create_module(module_name.as_str());
        let builder = context.create_builder();

        // set variable scope
        let mut addr_map_stack = Vec::new();
        let global_map: HashMap<String, (CC99BasicTYpe, PointerValue<'ctx>)> = HashMap::new();
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

    fn gen_function_proto(
        &mut self,
        ret_type: &CC99BasicTYpe,
        func_name: &String,
        func_param: &Vec<CC99BasicTYpe>
    ) -> Result<()> {
        // cannot handle duplicate function
        if self.function_map.contains_key(func_name) {
            return Err(CompileErr::DuplicateFunction(func_name.to_string()).into());
        }
        if self.global_variable_map.contains_key(func_name) {
            return Err(CompileErr::Redefinition(func_name.to_string()).into());
        }

        // function parameter should be added in this llvm_func_type
        let mut llvm_params: Vec<BasicTypeEnum<'ctx>> = Vec::new();
        let mut params: Vec<CC99BasicTYpe> = Vec::new();

        for param in func_param {
            params.push(param.to_owned());
            llvm_params.push(param.base_type.to_llvm_type(self.context));
        }

        let llvm_func_ty = self.to_return_type(ret_type, &llvm_params)?;

        // create function
        self.module.add_function(func_name.as_str(), llvm_func_ty, None);

        let ret_ty = if ret_type.base_type != BaseType::Void {
            Some(ret_type.to_owned())
        } else {
            None
        };

        self.function_map.insert(func_name.to_owned(), (ret_ty, params));
        Ok(())
    }

    // add void type as return type
    fn to_return_type(
        &self,
        in_type: &CC99BasicTYpe,
        param_types: &Vec<BasicTypeEnum<'ctx>>
    ) -> Result<FunctionType<'ctx>> {
        let param_types_meta = param_types.iter()
            .map(|ty| BasicMetadataTypeEnum::from(*ty))
            .collect::<Vec<BasicMetadataTypeEnum>>();

        match in_type.base_type {
            BaseType::Void => Ok(self.context.void_type().fn_type(
                &param_types_meta,
                false,
            )),
            _ => {
                let basic_type = in_type.base_type.to_llvm_type(self.context);
                Ok(basic_type.fn_type(&param_types_meta, false))
            }
        }
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

    fn cast_value(&self,
                  cur_ty: &BaseType,
                  cur_val: &BasicValueEnum<'ctx>,
                  cast_ty: &BaseType,
    ) -> Result<BasicValueEnum<'ctx>> {
        if cur_ty == cast_ty {
            return Ok(cur_val.to_owned());
        }

        Ok(self.builder.build_cast(
            self.gen_cast_llvm_instruction(cur_ty, cast_ty)?, *cur_val,
            cast_ty.to_llvm_type(self.context), "cast",
        ))
    }

    fn get_variable(&self, identifier: &String) -> Result<(CC99BasicTYpe, PointerValue<'ctx>)> {
        let mut result = None;

        self.addr_map_stack.iter().rev().for_each(|addr_map| {
            if let Some(val) = addr_map.get(identifier) {
                result = Some(val.to_owned());
            }
        });

        if result.is_none() {
            result = self.global_variable_map.get(identifier).cloned();
        }

        if result.is_none() {
            return Err(CompileErr::MissingVariable(identifier.to_string()).into());
        }

        Ok(result.unwrap())
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
                    BaseType::Pointer(Box::new(CC99BasicTYpe{
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