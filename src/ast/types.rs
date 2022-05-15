use std::fmt;
use super::*;
use anyhow::Result;
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

    pub(crate) fn upcast(lhs: &BaseType, rhs: &BaseType) -> Result<BaseType> {
        if lhs.cast_rank() >= rhs.cast_rank() {
            Ok(lhs.clone())
        } else {
            Ok(rhs.clone())
        }
    }

    pub(crate) fn test_cast(&self, dest: &BaseType) -> Result<()> {
        // same type, directly cast
        if self == dest {
            return Ok(());
        }

        if self.cast_rank() < dest.cast_rank() {
            return Ok(());
        }

        Err(CompileErr::InvalidDefaultCast(self.clone(), dest.clone()).into())
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
