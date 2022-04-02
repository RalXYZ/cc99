use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum AssignOperation {
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

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum UnaryOperation {
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

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum BinaryOperation {
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
        AssignOperation::Naive
    }
}

impl Default for UnaryOperation {
    fn default() -> Self {
        UnaryOperation::PrefixIncrement
    }
}

impl Default for BinaryOperation {
    fn default() -> Self {
        BinaryOperation::Comma
    }
}
