use super::*;

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

impl Default for StorageClassSpecifier {
    fn default() -> Self {
        StorageClassSpecifier::Auto
    }
}
