use crate::ast::{
    BaseType, BasicType as BT, DeclarationEnum, Expression, IntegerType, Span,
    StorageClassSpecifier, Type, AST,
};
use crate::generator::Generator;
use crate::utils::CompileErr as CE;
use inkwell::context::Context;
use inkwell::module::Linkage;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType};
use inkwell::values::{BasicValueEnum, PointerValue};
use inkwell::AddressSpace;
use std::collections::{HashMap, VecDeque};
use std::ops::Deref;
use std::path::Path;

impl<'ctx> Generator<'ctx> {
    // new LLVM context
    pub fn new(context: &'ctx Context, source_path: &'ctx str, code: &'ctx str) -> Generator<'ctx> {
        use codespan_reporting::files::SimpleFiles;

        let module_name = Path::new(source_path)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();
        let module = context.create_module(module_name);
        let builder = context.create_builder();

        // set variable scope
        let mut val_map_block_stack = Vec::new();
        let global_map: HashMap<String, (BT, PointerValue<'ctx>)> = HashMap::new();
        val_map_block_stack.push(global_map); // push global variable hashmap

        let mut files = SimpleFiles::new();
        files.add(source_path, code);

        Generator {
            files,
            module_name,
            context,
            module,
            builder,
            val_map_block_stack,
            global_struct_map: HashMap::new(),
            current_function: None,
            break_labels: VecDeque::new(),
            continue_labels: VecDeque::new(),
            function_map: HashMap::new(),
            global_variable_map: HashMap::new(),
        }
    }

    pub fn gen(&mut self, ast: &AST) {
        let AST::GlobalDeclaration(ref declarations) = ast.deref();

        let mut err: Vec<CE> = vec![];

        // first-time scanning, gen declarations
        err.extend(
            declarations
                .iter()
                .map(|declaration| -> Result<(), CE> {
                    match declaration.node {
                        DeclarationEnum::Declaration(
                            ref type_info,
                            ref identifier,
                            ref initializer,
                        ) => match type_info.basic_type.base_type {
                            BaseType::Function(ref return_type, ref params_type, is_variadic) => {
                                self.gen_function_proto(
                                    &type_info.storage_class_specifier,
                                    return_type,
                                    identifier.as_ref().unwrap(),
                                    params_type,
                                    is_variadic,
                                    declaration.span,
                                )
                            }
                            _ => self.gen_global_variable(
                                type_info,
                                identifier.as_ref().unwrap(),
                                initializer,
                                declaration.span,
                            ),
                        },
                        DeclarationEnum::FunctionDefinition(
                            _,
                            ref storage_class,
                            ref return_type,
                            ref identifier,
                            ref params_type,
                            ref is_variadic,
                            _,
                        ) => {
                            if !self.function_map.contains_key(identifier) {
                                self.gen_function_proto(
                                    storage_class,
                                    return_type,
                                    identifier,
                                    params_type
                                        .iter()
                                        .map(|param| param.0.clone())
                                        .collect::<Vec<_>>()
                                        .as_slice(),
                                    *is_variadic,
                                    declaration.span,
                                )
                            } else {
                                Ok(())
                            }
                        }
                    }
                })
                .filter_map(|result| if result.is_err() { result.err() } else { None }),
        );

        // second-time scanning, gen func definitions
        err.extend(
            declarations
                .iter()
                .map(|declaration| -> Result<(), Vec<CE>> {
                    if let DeclarationEnum::FunctionDefinition(
                        _,
                        _,
                        ref return_type,
                        ref identifier,
                        ref params_type,
                        _,
                        ref statements,
                    ) = declaration.node
                    {
                        self.gen_func_def(
                            return_type,
                            identifier,
                            params_type,
                            statements,
                            declaration.span,
                        )?;
                    }
                    Ok(())
                })
                .filter_map(|result| result.err())
                .flatten()
                .collect::<Vec<_>>(),
        );

        if !err.is_empty() {
            err.iter().for_each(|err| {
                self.gen_err_output(0, err);
            });
            std::process::exit(err.len() as i32);
        }
    }

    fn gen_function_proto(
        &mut self,
        storage_class: &StorageClassSpecifier,
        ret_type: &BT,
        func_name: &str,
        func_param: &[BT],
        is_variadic: bool,
        span: Span,
    ) -> Result<(), CE> {
        if self.function_map.contains_key(func_name) {
            return Err(CE::duplicated_function(func_name.to_string(), span));
        }
        if self.global_variable_map.contains_key(func_name) {
            return Err(CE::redefinition_symbol(func_name.to_string(), span));
        }

        // function parameter should be added in this llvm_func_type
        let mut llvm_params: Vec<BasicTypeEnum<'ctx>> = Vec::new();
        let mut params: Vec<BT> = Vec::new();

        for param in func_param {
            params.push(param.to_owned());
            llvm_params.push(self.convert_llvm_type(&param.base_type));
        }

        let llvm_func_ty = self.gen_return_type(ret_type, &llvm_params, is_variadic)?;

        let linkage = match storage_class {
            StorageClassSpecifier::Static => Some(Linkage::Internal),
            StorageClassSpecifier::Extern => Some(Linkage::External),
            StorageClassSpecifier::Auto => None,
            StorageClassSpecifier::Typedef => unimplemented!(),
            _ => unreachable!(),
        };

        // create function
        self.module.add_function(func_name, llvm_func_ty, linkage);
        self.function_map.insert(
            func_name.to_owned(),
            (ret_type.to_owned(), params, is_variadic),
        );
        Ok(())
    }

    // add void type as return type
    fn gen_return_type(
        &mut self,
        in_type: &BT,
        param_types: &[BasicTypeEnum<'ctx>],
        is_variadic: bool,
    ) -> Result<FunctionType<'ctx>, CE> {
        let param_types_meta = param_types
            .iter()
            .map(|ty| BasicMetadataTypeEnum::from(*ty))
            .collect::<Vec<BasicMetadataTypeEnum>>();

        match in_type.base_type {
            BaseType::Void => Ok(self
                .context
                .void_type()
                .fn_type(&param_types_meta, is_variadic)),
            _ => {
                let basic_type = self.convert_llvm_type(&in_type.base_type);
                Ok(basic_type.fn_type(&param_types_meta, is_variadic))
            }
        }
    }

    pub(crate) fn cast_value(
        &self,
        curr_type: &BaseType,
        curr_val: &BasicValueEnum<'ctx>,
        dest_type: &BaseType,
        span: Span,
    ) -> Result<BasicValueEnum<'ctx>, CE> {
        if curr_type.equal_discarding_qualifiers(dest_type) {
            return Ok(curr_val.to_owned());
        }

        let llvm_type = self.convert_llvm_type(dest_type);

        Ok(self.builder.build_cast(
            self.gen_cast_llvm_instruction(curr_type, dest_type, span)?,
            *curr_val,
            llvm_type,
            "cast",
        ))
    }

    pub(crate) fn get_variable(
        &self,
        identifier: &str,
        span: Span,
    ) -> Result<(BT, PointerValue<'ctx>), CE> {
        let mut result = None;

        self.val_map_block_stack.iter().rev().for_each(|addr_map| {
            if let Some(val) = addr_map.get(identifier) {
                result = Some(val.to_owned());
            }
        });

        if result.is_none() {
            result = self.global_variable_map.get(identifier).cloned();
        }

        if result.is_none() {
            return Err(CE::missing_variable(identifier.to_string(), span));
        }

        Ok(result.unwrap())
    }

    fn gen_global_variable(
        &mut self,
        var_type: &Type,
        var_name: &str,
        ptr_to_init: &Option<Box<Expression>>,
        span: Span,
    ) -> Result<(), CE> {
        if self.global_variable_map.contains_key(var_name) {
            return Err(CE::duplicated_global_variable(var_name.to_string(), span));
        } else if self.function_map.contains_key(var_name) {
            return Err(CE::duplicated_symbol(var_name.to_string(), span));
        }

        let llvm_type = self.convert_llvm_type(&var_type.basic_type.base_type);
        let global_value = self.module.add_global(llvm_type, None, var_name);
        global_value.set_linkage(Linkage::Common);

        if var_type.basic_type.is_const() {
            global_value.set_constant(true);
        }

        match ptr_to_init {
            Some(ptr_to_init) => {
                let (e_t, e_v) = self.gen_expression(&**ptr_to_init)?;
                e_t.test_cast(&var_type.basic_type.base_type, ptr_to_init.span)?;
                let value_after_cast =
                    self.cast_value(&e_t, &e_v, &var_type.basic_type.base_type, ptr_to_init.span)?;

                global_value.set_initializer(&value_after_cast);
            }
            None => {
                global_value.set_initializer(&llvm_type.const_zero());
            }
        }

        self.global_variable_map.insert(
            var_name.to_string(),
            (
                var_type.basic_type.to_owned(),
                global_value.as_pointer_value(),
            ),
        );

        Ok(())
    }

    pub(crate) fn convert_llvm_type(&self, base_type: &BaseType) -> BasicTypeEnum<'ctx> {
        match *base_type {
            BaseType::Bool => self.context.bool_type().as_basic_type_enum(),
            BaseType::SignedInteger(IntegerType::Char) => {
                self.context.i8_type().as_basic_type_enum()
            }
            BaseType::UnsignedInteger(IntegerType::Char) => {
                self.context.i8_type().as_basic_type_enum()
            }
            BaseType::SignedInteger(IntegerType::Short) => {
                self.context.i16_type().as_basic_type_enum()
            }
            BaseType::UnsignedInteger(IntegerType::Short) => {
                self.context.i16_type().as_basic_type_enum()
            }
            BaseType::SignedInteger(IntegerType::Int) => {
                self.context.i32_type().as_basic_type_enum()
            }
            BaseType::UnsignedInteger(IntegerType::Int) => {
                self.context.i32_type().as_basic_type_enum()
            }
            BaseType::SignedInteger(IntegerType::Long) => {
                self.context.i64_type().as_basic_type_enum()
            }
            BaseType::UnsignedInteger(IntegerType::Long) => {
                self.context.i64_type().as_basic_type_enum()
            }
            BaseType::SignedInteger(IntegerType::LongLong) => {
                self.context.i64_type().as_basic_type_enum()
            }
            BaseType::UnsignedInteger(IntegerType::LongLong) => {
                self.context.i64_type().as_basic_type_enum()
            }
            BaseType::Float => self.context.f32_type().as_basic_type_enum(),
            BaseType::Double => self.context.f64_type().as_basic_type_enum(),
            BaseType::Pointer(ref basic_type) => self
                .convert_llvm_type(&basic_type.base_type)
                .ptr_type(AddressSpace::Generic)
                .as_basic_type_enum(),
            BaseType::Array(ref basic_type, ref size) => size
                .iter()
                .rev()
                .map(|x| {
                    self.gen_expression(x)
                        .unwrap()
                        .1
                        .into_int_value()
                        .get_zero_extended_constant()
                        .unwrap() as u32
                })
                .fold(self.convert_llvm_type(&basic_type.base_type), |acc, len| {
                    acc.array_type(len).as_basic_type_enum()
                })
                .as_basic_type_enum(),
            BaseType::Struct(ref _name, ref members) => {
                let mut member_types = Vec::new();
                for x in members.clone().unwrap() {
                    member_types.push(self.convert_llvm_type(&x.member_type.base_type));
                }
                self.context
                    .struct_type(member_types.as_slice(), false)
                    .as_basic_type_enum()
            }
            _ => panic!(),
        }
    }
}
