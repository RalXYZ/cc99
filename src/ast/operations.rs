use pest::Span;
use serde::{Serialize, Serializer};

#[derive(Debug, PartialEq, Clone)]
pub struct AssignOperation<'a> {
    pub node: AssignOperationEnum,
    pub span: Span<'a>,
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
pub struct UnaryOperation<'a> {
    pub node: UnaryOperationEnum,
    pub span: Span<'a>,
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
pub struct BinaryOperation<'a> {
    pub node: BinaryOperationEnum,
    pub span: Span<'a>,
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

impl<'a> AssignOperation<'a> {
    pub fn default(code: &'a str) -> Self {
        AssignOperation {
            node: AssignOperationEnum::Naive,
            span: Span::new(code, 0, 0).unwrap(),
        }
    }
}

impl<'a> UnaryOperation<'a> {
    pub fn default(code: &'a str) -> Self {
        UnaryOperation {
            node: UnaryOperationEnum::PrefixIncrement,
            span: Span::new(code, 0, 0).unwrap(),
        }
    }
}

impl<'a> BinaryOperation<'a> {
    pub fn default(code: &'a str) -> Self {
        BinaryOperation {
            node: BinaryOperationEnum::Comma,
            span: Span::new(code, 0, 0).unwrap(),
        }
    }
}

impl Serialize for AssignOperation<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.node.serialize(serializer)
    }
}

impl Serialize for UnaryOperation<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.node.serialize(serializer)
    }
}

impl Serialize for BinaryOperation<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.node.serialize(serializer)
    }
}
