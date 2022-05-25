#[cfg(not(feature = "web"))]
use super::super::utils::CompileErr;
use super::*;
#[cfg(not(feature = "web"))]
use anyhow::Result;
use std::fmt;

use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Clone, Default)]
pub struct Type<'a> {
    pub function_specifier: Vec<FunctionSpecifier>,
    pub storage_class_specifier: StorageClassSpecifier,
    pub basic_type: BasicType<'a>,
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
pub struct BasicType<'a> {
    pub qualifier: Vec<TypeQualifier>,
    pub base_type: BaseType<'a>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum BaseType<'a> {
    Void,
    SignedInteger(IntegerType),
    UnsignedInteger(IntegerType),
    Bool,
    Float,
    Double,
    Pointer(Box<BasicType<'a>>),
    Array(
        /// element type
        Box<BasicType<'a>>,
        /// array length, from high-dimension to low-dimension
        Vec<Expression<'a>>,
    ),
    Function(
        /// return type
        Box<BasicType<'a>>,
        /// parameters' types
        Vec<BasicType<'a>>,
        /// is variadic or not
        bool,
    ),
    Struct(
        /// struct name
        Option<String>,
        /// struct members
        Option<Vec<StructMember<'a>>>,
    ),
    Union(
        /// union name
        Option<String>,
        /// union members
        Option<Vec<StructMember<'a>>>,
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
pub struct StructMember<'a> {
    pub member_name: String,
    pub member_type: BasicType<'a>,
}

impl Default for BaseType<'_> {
    fn default() -> Self {
        BaseType::SignedInteger(IntegerType::Int)
    }
}

impl<'ctx> BasicType<'_> {
    pub fn is_const(&self) -> bool {
        self.qualifier
            .iter()
            .any(|x| matches!(x, TypeQualifier::Const))
    }
}

#[cfg(not(feature = "web"))]
impl<'ctx> BaseType<'ctx> {
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
            _ => panic!(),
        }
    }

    pub(crate) fn upcast(lhs: &BaseType<'ctx>, rhs: &BaseType<'ctx>) -> Result<BaseType<'ctx>> {
        if lhs.cast_rank() >= rhs.cast_rank() {
            Ok(lhs.clone())
        } else {
            Ok(rhs.clone())
        }
    }

    pub(crate) fn equal_discarding_qualifiers(&self, rhs: &BaseType) -> bool {
        if self == rhs {
            return true;
        }

        if let BaseType::Pointer(lhs_inner) = self {
            if let BaseType::Pointer(rhs_inner) = rhs {
                return lhs_inner
                    .base_type
                    .equal_discarding_qualifiers(&rhs_inner.base_type);
            }
        }

        false
    }

    pub(crate) fn test_cast(&self, dest: &BaseType<'ctx>) -> Result<()> {
        // same type, directly cast
        if self == dest {
            return Ok(());
        }

        if let (BaseType::Pointer(lhs_ptr), BaseType::Pointer(rhs_ptr)) = (self, dest) {
            // TODO: handle const
            if lhs_ptr
                .base_type
                .equal_discarding_qualifiers(&rhs_ptr.base_type)
            {
                return Ok(());
            };
            // return Err(CompileErr::InvalidDefaultCast(self.clone(), dest.clone()).into());
            unimplemented!()
        }

        if let (BaseType::Array(lhs_type, lhs_expr), BaseType::Pointer(rhs_ptr)) = (self, dest) {
            if lhs_expr.len() != 1 {
                // return Err(CompileErr::InvalidDefaultCast(self.clone(), dest.clone()).into());
                unimplemented!()
            }
            //make sure they are both basic type(not pointer or array)
            lhs_type.base_type.cast_rank();
            rhs_ptr.base_type.cast_rank();
            return Ok(());
        }

        if let BaseType::Pointer(_) = self {
            // return Err(CompileErr::InvalidDefaultCast(self.clone(), dest.clone()).into());
            unimplemented!()
        }
        if let BaseType::Pointer(_) = dest {
            // return Err(CompileErr::InvalidDefaultCast(self.clone(), dest.clone()).into());
            unimplemented!()
        }

        if self.cast_rank() < dest.cast_rank() {
            return Ok(());
        }

        // Err(CompileErr::InvalidDefaultCast(self.clone(), dest.clone()).into())
        unimplemented!()
    }
}

impl fmt::Display for BaseType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for StorageClassSpecifier {
    fn default() -> Self {
        StorageClassSpecifier::Auto
    }
}
