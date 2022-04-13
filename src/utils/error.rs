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
}