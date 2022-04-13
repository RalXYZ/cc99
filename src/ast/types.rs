use std::fmt;
use inkwell::AddressSpace;
use super::*;
use anyhow::Result;
use inkwell::context::Context;
use inkwell::types::{BasicType as LlvmBasicType, BasicTypeEnum};
use super::super::utils::CompileErr;

use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Clone, Default)]
pub struct Type {
    pub function_specifier: Vec<FunctionSpecifier>,
    pub storage_class_specifier: StorageClassSpecifier,
    pub basic_type: BasicType,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum StorageClassSpecifier {
    Typedef,
    Extern,
    Static,
    ThreadLocal,
    Auto,
    Register,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum TypeQualifier {
    Const,
    Volatile,
    Restrict,
    Atomic,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum FunctionSpecifier {
    Inline,
    Noreturn,
}

#[derive(Serialize, Debug, PartialEq, Clone, Default)]
pub struct BasicType {
    pub qualifier: Vec<TypeQualifier>,
    pub base_type: BaseType,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum BaseType {
    Void,
    SignedInteger(IntegerType),
    UnsignedInteger(IntegerType),
    Bool,
    Float,
    Double,
    Pointer(Box<BasicType>),
    Array(
        /// element type
        Box<BasicType>,
        /// array length
        Box<Expression>,
    ),
    Function(
        /// return type
        Box<BasicType>,
        /// parameters' types
        Vec<BasicType>,
        /// is variadic or not
        bool,
    ),
    Struct(
        /// struct name
        Option<String>,
        /// struct members
        Option<Vec<StructMember>>,
    ),
    Union(
        /// union name
        Option<String>,
        /// union members
        Option<Vec<StructMember>>,
    ),
    /// a name introduced by typedef/struct...
    Identifier(String),
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum IntegerType {
    Short,
    Char,
    Int,
    Long,
    LongLong,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct StructMember {
    pub member_name: String,
    pub member_type: BasicType,
}

impl Default for BaseType {
    fn default() -> Self {
        BaseType::SignedInteger(IntegerType::Int)
    }
}

impl<'ctx> BaseType {
    // convert from internal type to llvm type
    pub fn to_llvm_type(&self, ctx: &'ctx Context) -> BasicTypeEnum<'ctx> {
        match self {
            &BaseType::Bool => ctx.bool_type().as_basic_type_enum(),
            &BaseType::SignedInteger(IntegerType::Char) => ctx.i8_type().as_basic_type_enum(),
            &BaseType::UnsignedInteger(IntegerType::Char) => ctx.i8_type().as_basic_type_enum(),
            &BaseType::SignedInteger(IntegerType::Short) => ctx.i16_type().as_basic_type_enum(),
            &BaseType::UnsignedInteger(IntegerType::Short) => ctx.i16_type().as_basic_type_enum(),
            &BaseType::SignedInteger(IntegerType::Int) => ctx.i32_type().as_basic_type_enum(),
            &BaseType::UnsignedInteger(IntegerType::Int) => ctx.i32_type().as_basic_type_enum(),
            &BaseType::SignedInteger(IntegerType::Long) => ctx.i64_type().as_basic_type_enum(),
            &BaseType::UnsignedInteger(IntegerType::Long) => ctx.i64_type().as_basic_type_enum(),
            &BaseType::SignedInteger(IntegerType::LongLong) => ctx.i64_type().as_basic_type_enum(),
            &BaseType::UnsignedInteger(IntegerType::LongLong) => ctx.i64_type().as_basic_type_enum(),
            &BaseType::Float => ctx.f32_type().as_basic_type_enum(),
            &BaseType::Double => ctx.f64_type().as_basic_type_enum(),
            &BaseType::Pointer(ref basic_type) => {
                basic_type.base_type
                    .to_llvm_type(ctx)
                    .ptr_type(AddressSpace::Generic).as_basic_type_enum()
            }
            _ => panic!()
        }
    }

    fn cast_rank(&self) -> i32 {
        match self {
            &BaseType::Void => 0,
            &BaseType::Bool => 1,
            &BaseType::SignedInteger(IntegerType::Char) => 2,
            &BaseType::UnsignedInteger(IntegerType::Char) => 2,
            &BaseType::SignedInteger(IntegerType::Short) => 3,
            &BaseType::UnsignedInteger(IntegerType::Short) => 3,
            &BaseType::SignedInteger(IntegerType::Int) => 4,
            &BaseType::UnsignedInteger(IntegerType::Int) => 4,
            &BaseType::SignedInteger(IntegerType::Long) => 5,
            &BaseType::UnsignedInteger(IntegerType::Long) => 5,
            &BaseType::SignedInteger(IntegerType::LongLong) => 6,
            &BaseType::UnsignedInteger(IntegerType::LongLong) => 6,
            &BaseType::Float => 7,
            &BaseType::Double => 8,
            _ => panic!()
        }
    }

    // default cast
    pub fn default_cast(&self, cast_ty: &BaseType) -> Result<BaseType> {
        // same type, directly cast
        if self == cast_ty {
            return Ok(cast_ty.clone());
        }

        if self.cast_rank() < cast_ty.cast_rank() {
            return Ok(cast_ty.clone());
        }

        return Err(CompileErr::InvalidDefaultCast(self.clone(), cast_ty.clone()).into());
    }
}

impl fmt::Display for BaseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for StorageClassSpecifier {
    fn default() -> Self {
        StorageClassSpecifier::Auto
    }
}
