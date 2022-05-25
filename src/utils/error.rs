use crate::ast::BaseType;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompileErr {
    #[error("there are duplicated global variables: {}", .0)]
    DuplicatedGlobalVariable(String),

    #[error("there are duplicated symbols: {}", .0)]
    DuplicatedSymbol(String),

    #[error("unknown expression: {}", .0)]
    UnknownExpression(String),

    #[error("invalid default cast between {} and {}", .0.to_string(), .1.to_string())]
    InvalidDefaultCast(BaseType, BaseType),

    #[error("invalid cast from {} to {}", .0.to_string(), .1.to_string())]
    InvalidCast(BaseType, BaseType),

    #[error("invalid unary operator")]
    InvalidUnary,

    #[error("invalid binary operator")]
    InvalidBinary,

    #[error("there are duplicate functions: {}", .0.as_str())]
    DuplicateFunction(String),

    #[error("redefinition of symbol: {}", .0.as_str())]
    Redefinition(String),

    #[error("missing variable: {}", .0.as_str())]
    MissingVariable(String),

    #[error("there are duplicate local variables: {}", .0.as_str())]
    DuplicatedVariable(String),

    #[error("keyword {} is not in a loop", .0.as_str())]
    KeywordNotInLoop(String),

    #[error("array dimension not match, expect {}, found {}", .0.to_string(), .1.to_string())]
    ArrayDimensionNotMatch(usize, usize),

    #[error("point dimension not match, expect {}, found {}", .0.to_string(), .1.to_string())]
    PointDimensionNotMatch(usize, usize),

    #[error("Invalid left value for a not addressable variable: {}", .0.as_str())]
    InvalidLeftValue(String),

    #[error("Invalid dereference for a no pointer variable: {}", .0.as_str())]
    InvalidDereference(String),

    #[error("error: {}", .0.as_str())]
    Error(String),

    #[error("parameter count of {} mismatch: expect {}, got {}", .0.as_str(), .1.to_string(), .2.to_string())]
    ParameterCountMismatch(String, usize, usize),
}
