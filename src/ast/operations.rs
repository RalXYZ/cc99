use serde::Serialize;

#[derive(Serialize, Debug)]
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

#[derive(Serialize, Debug)]
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
}

#[derive(Serialize, Debug)]
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
    // member access
    ArraySubscript,
    // other
    CommaOperator,
}
