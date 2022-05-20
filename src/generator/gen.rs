use crate::ast::{
    BaseType, BasicType as BT, Declaration, Expression, IntegerType, StorageClassSpecifier, Type,
    AST,
};
use crate::generator::Generator;
use crate::utils::CompileErr;
use anyhow::Result;
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
    pub fn new(context: &'ctx Context, source_path: &'ctx str) -> Generator<'ctx> {
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

        Generator {
            module_name,
            context,
            module,
            builder,
            val_map_block_stack,
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
        declarations
            .iter()
            .filter_map(|declaration| {
                if let Declaration::Declaration(ref type_info, ref identifier, ref initializer) =
                    declaration
                {
                    Some((type_info, identifier, initializer))
                } else {
                    None
                }
            })
            .try_for_each(|(type_info, identifier, initializer)| -> Result<()> {
                if let BaseType::Function(ref return_type, ref params_type, is_variadic) =
                    type_info.basic_type.base_type
                {
                    self.gen_function_proto(
                        &type_info.storage_class_specifier,
                        return_type,
                        identifier.as_ref().unwrap(),
                        params_type,
                        is_variadic,
                    )?;
                } else {
                    self.gen_global_variable(type_info, identifier.as_ref().unwrap(), initializer)?;
                }
                Ok(())
            })?;

        declarations
            .iter()
            .try_for_each(|declaration| -> Result<()> {
                if let Declaration::FunctionDefinition(
                    _,
                    ref storage_class,
                    ref return_type,
                    ref identifier,
                    ref params_type,
                    is_variadic,
                    ref statements,
                ) = declaration
                {
                    if !self.function_map.contains_key(identifier) {
                        self.gen_function_proto(
                            storage_class,
                            return_type,
                            identifier,
                            &params_type.iter().map(|param| param.0.clone()).collect(),
                            *is_variadic,
                        )?;
                    }
                    self.gen_func_def(&return_type, identifier, params_type, statements)?;
                }
                Ok(())
            })?;

        Ok(())
    }

    fn gen_function_proto(
        &mut self,
        storage_class: &StorageClassSpecifier,
        ret_type: &BT,
        func_name: &String,
        func_param: &Vec<BT>,
        is_variadic: bool,
    ) -> Result<()> {
        if self.function_map.contains_key(func_name) {
            return Err(CompileErr::DuplicateFunction(func_name.to_string()).into());
        }
        if self.global_variable_map.contains_key(func_name) {
            return Err(CompileErr::Redefinition(func_name.to_string()).into());
        }

        // function parameter should be added in this llvm_func_type
        let mut llvm_params: Vec<BasicTypeEnum<'ctx>> = Vec::new();
        let mut params: Vec<BT> = Vec::new();

        for param in func_param {
            params.push(param.to_owned());
            llvm_params.push(self.convert_llvm_type(&param.base_type));
        }

        let llvm_func_ty = self.to_return_type(ret_type, &llvm_params, is_variadic)?;

        let linkage = match storage_class {
            StorageClassSpecifier::Static => Some(Linkage::Internal),
            StorageClassSpecifier::Extern => Some(Linkage::External),
            StorageClassSpecifier::Auto => None,
            StorageClassSpecifier::Typedef => unimplemented!(),
            _ => unreachable!(),
        };

        // create function
        self.module
            .add_function(func_name.as_str(), llvm_func_ty, linkage);
        self.function_map.insert(
            func_name.to_owned(),
            (ret_type.to_owned(), params, is_variadic),
        );
        Ok(())
    }

    // add void type as return type
    fn to_return_type(
        &mut self,
        in_type: &BT,
        param_types: &Vec<BasicTypeEnum<'ctx>>,
        is_variadic: bool,
    ) -> Result<FunctionType<'ctx>> {
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
        &mut self,
        curr_type: &BaseType,
        curr_val: &BasicValueEnum<'ctx>,
        dest_type: &BaseType,
    ) -> Result<BasicValueEnum<'ctx>> {
        if curr_type.equal_discarding_qualifiers(dest_type) {
            return Ok(curr_val.to_owned());
        }

        let llvm_type = self.convert_llvm_type(dest_type);

        Ok(self.builder.build_cast(
            self.gen_cast_llvm_instruction(curr_type, dest_type)?,
            *curr_val,
            llvm_type,
            "cast",
        ))
    }

    pub(crate) fn get_variable(&self, identifier: &String) -> Result<(BT, PointerValue<'ctx>)> {
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
            return Err(CompileErr::MissingVariable(identifier.to_string()).into());
        }

        Ok(result.unwrap())
    }

    fn gen_global_variable(
        &mut self,
        var_type: &Type,
        var_name: &String,
        ptr_to_init: &Option<Box<Expression>>,
    ) -> Result<()> {
        if self.global_variable_map.contains_key(var_name) {
            return Err(CompileErr::DuplicatedGlobalVariable(var_name.to_string()).into());
        } else if self.function_map.contains_key(var_name) {
            return Err(CompileErr::DuplicatedSymbol(var_name.to_string()).into());
        }

        let llvm_type = self.convert_llvm_type(&var_type.basic_type.base_type);
        let global_value = self.module.add_global(llvm_type, None, var_name.as_str());

        // if ptr_to_init is not None
        if let Some(ptr_to_init) = ptr_to_init {
            let init_val_pair = self.gen_expression(&**ptr_to_init)?;
            init_val_pair.0.test_cast(&var_type.basic_type.base_type)?;
            let value_after_cast = self.cast_value(
                &init_val_pair.0,
                &init_val_pair.1,
                &var_type.basic_type.base_type,
            )?;

            global_value.set_initializer(&value_after_cast);
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

    pub(crate) fn convert_llvm_type(&mut self, base_type: &BaseType) -> BasicTypeEnum<'ctx> {
        match base_type {
            &BaseType::Bool => self.context.bool_type().as_basic_type_enum(),
            &BaseType::SignedInteger(IntegerType::Char) => {
                self.context.i8_type().as_basic_type_enum()
            }
            &BaseType::UnsignedInteger(IntegerType::Char) => {
                self.context.i8_type().as_basic_type_enum()
            }
            &BaseType::SignedInteger(IntegerType::Short) => {
                self.context.i16_type().as_basic_type_enum()
            }
            &BaseType::UnsignedInteger(IntegerType::Short) => {
                self.context.i16_type().as_basic_type_enum()
            }
            &BaseType::SignedInteger(IntegerType::Int) => {
                self.context.i32_type().as_basic_type_enum()
            }
            &BaseType::UnsignedInteger(IntegerType::Int) => {
                self.context.i32_type().as_basic_type_enum()
            }
            &BaseType::SignedInteger(IntegerType::Long) => {
                self.context.i64_type().as_basic_type_enum()
            }
            &BaseType::UnsignedInteger(IntegerType::Long) => {
                self.context.i64_type().as_basic_type_enum()
            }
            &BaseType::SignedInteger(IntegerType::LongLong) => {
                self.context.i64_type().as_basic_type_enum()
            }
            &BaseType::UnsignedInteger(IntegerType::LongLong) => {
                self.context.i64_type().as_basic_type_enum()
            }
            &BaseType::Float => self.context.f32_type().as_basic_type_enum(),
            &BaseType::Double => self.context.f64_type().as_basic_type_enum(),
            &BaseType::Pointer(ref basic_type) => self
                .convert_llvm_type(&basic_type.base_type)
                .ptr_type(AddressSpace::Generic)
                .as_basic_type_enum(),
            &BaseType::Array(ref basic_type, ref length) => self
                .convert_llvm_type(&basic_type.base_type)
                .array_type(
                    self.gen_expression(length)
                        .unwrap()
                        .1
                        .into_int_value()
                        .get_zero_extended_constant()
                        .unwrap() as u32,
                )
                .as_basic_type_enum(),
            _ => panic!(),
        }
    }
}
