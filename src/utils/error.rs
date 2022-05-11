use thiserror::Error;
use crate::ast::BaseType;

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

    #[error("invalid cast between {} and {}", .0.to_string(), .1.to_string())]
    InvalidCast(BaseType, BaseType),

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

    #[error("error: {}", .0.as_str())]
    Error(String),
}