use super::super::utils::CompileErr as CE;
use super::*;
use std::fmt;

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
        /// array length, from high-dimension to low-dimension
        Vec<Expression>,
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

impl<'ctx> BasicType {
    pub fn is_const(&self) -> bool {
        self.qualifier
            .iter()
            .any(|x| matches!(x, TypeQualifier::Const))
    }
}

impl<'ctx> BaseType {
    fn cast_rank(&self) -> i32 {
        match *self {
            BaseType::Void => 0,
            BaseType::Bool => 1,
            BaseType::SignedInteger(IntegerType::Char) => 2,
            BaseType::UnsignedInteger(IntegerType::Char) => 2,
            BaseType::SignedInteger(IntegerType::Short) => 3,
            BaseType::UnsignedInteger(IntegerType::Short) => 3,
            BaseType::SignedInteger(IntegerType::Int) => 4,
            BaseType::UnsignedInteger(IntegerType::Int) => 4,
            BaseType::SignedInteger(IntegerType::Long) => 5,
            BaseType::UnsignedInteger(IntegerType::Long) => 5,
            BaseType::SignedInteger(IntegerType::LongLong) => 6,
            BaseType::UnsignedInteger(IntegerType::LongLong) => 6,
            BaseType::Float => 7,
            BaseType::Double => 8,
            _ => panic!(),
        }
    }

    pub(crate) fn upcast(lhs: &BaseType, rhs: &BaseType) -> Result<BaseType, CE> {
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

    pub fn test_cast(&self, dest: &BaseType, span: Span) -> Result<(), CE> {
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
            return Err(CE::invalid_default_cast(
                self.to_string(),
                dest.to_string(),
                span,
            ));
        }

        if let (BaseType::Array(lhs_type, lhs_expr), BaseType::Pointer(rhs_ptr)) = (self, dest) {
            if lhs_expr.len() != 1 {
                return Err(CE::invalid_default_cast(
                    self.to_string(),
                    dest.to_string(),
                    span,
                ));
            }
            //make sure they are both basic type(not pointer or array)
            lhs_type.base_type.cast_rank();
            rhs_ptr.base_type.cast_rank();
            return Ok(());
        }

        if let BaseType::Pointer(_) = self {
            return Err(CE::invalid_default_cast(
                self.to_string(),
                dest.to_string(),
                span,
            ));
        }
        if let BaseType::Pointer(_) = dest {
            return Err(CE::invalid_default_cast(
                self.to_string(),
                dest.to_string(),
                span,
            ));
        }

        if self.cast_rank() < dest.cast_rank() {
            return Ok(());
        }

        Err(CE::invalid_default_cast(
            self.to_string(),
            dest.to_string(),
            span,
        ))
    }
}

impl fmt::Display for BaseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BaseType::Void => write!(f, "void"),
            BaseType::SignedInteger(IntegerType::Char) => write!(f, "char"),
            BaseType::UnsignedInteger(IntegerType::Char) => write!(f, "unsigned char"),
            BaseType::SignedInteger(IntegerType::Short) => write!(f, "short"),
            BaseType::UnsignedInteger(IntegerType::Short) => write!(f, "unsigned short"),
            BaseType::SignedInteger(IntegerType::Int) => write!(f, "int"),
            BaseType::UnsignedInteger(IntegerType::Int) => write!(f, "unsigned int"),
            BaseType::SignedInteger(IntegerType::Long) => write!(f, "long"),
            BaseType::UnsignedInteger(IntegerType::Long) => write!(f, "unsigned long"),
            BaseType::SignedInteger(IntegerType::LongLong) => write!(f, "long long"),
            BaseType::UnsignedInteger(IntegerType::LongLong) => write!(f, "unsigned long long"),
            BaseType::Bool => write!(f, "_Bool"),
            BaseType::Float => write!(f, "float"),
            BaseType::Double => write!(f, "double"),
            BaseType::Pointer(inner) => {
                write!(f, "{}", inner.base_type)?;
                write!(f, "*")
            }
            _ => write!(f, "{:?}", self),
        }
    }
}

impl Default for StorageClassSpecifier {
    fn default() -> Self {
        StorageClassSpecifier::Auto
    }
}
