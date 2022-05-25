use super::span::*;
use serde::{Serialize, Serializer};

#[derive(Debug, PartialEq, Clone)]
pub struct AssignOperation {
    pub node: AssignOperationEnum,
    pub span: Span,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum AssignOperationEnum {
    Naive,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryOperation {
    pub node: UnaryOperationEnum,
    pub span: Span,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum UnaryOperationEnum {
    // increment, decrement
    PrefixIncrement,
    PrefixDecrement,
    PostfixIncrement,
    PostfixDecrement,
    // arithmetic
    UnaryPlus,
    UnaryMinus,
    BitwiseNot,
    // logical
    LogicalNot,
    // member access
    Reference,
    Dereference,
    // other
    SizeofExpr,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryOperation {
    pub node: BinaryOperationEnum,
    pub span: Span,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum BinaryOperationEnum {
    // arithmetic
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
    // logical
    LogicalAnd,
    LogicalOr,
    // comparison
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    // other
    Comma,
}

impl Default for AssignOperation {
    fn default() -> Self {
        AssignOperation {
            node: AssignOperationEnum::Naive,
            span: Default::default(),
        }
    }
}

impl Default for UnaryOperation {
    fn default() -> Self {
        UnaryOperation {
            node: UnaryOperationEnum::PrefixIncrement,
            span: Default::default(),
        }
    }
}

impl Default for BinaryOperation {
    fn default() -> Self {
        BinaryOperation {
            node: BinaryOperationEnum::Comma,
            span: Default::default(),
        }
    }
}

impl Serialize for AssignOperation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.node.serialize(serializer)
    }
}

impl Serialize for UnaryOperation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.node.serialize(serializer)
    }
}

impl Serialize for BinaryOperation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.node.serialize(serializer)
    }
}
