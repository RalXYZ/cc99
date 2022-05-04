use serde::Serialize;

use super::operations::*;
use super::types::*;

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum AST {
    GlobalDeclarations(Vec<Declaration>),
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum Declaration {
    GlobalDeclaration(
        Type,
        /// identifier (if it's a struct/union declaration, it might be None)
        Option<String>,
        /// initializer
        Option<Box<Expression>>,
    ),
    FunctionDefinition(
        Type,
        /// identifier
        String,
        /// parameters
        Vec<Option<String>>,
        /// body
        Statement,
    ),
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum Statement {
    // labeled statement
    Labeled(String, Box<Statement>),
    Case(
        /// None represents default case
        Option<Box<Expression>>,
        Box<Statement>,
    ),
    // compound statement
    Compound(Vec<StatementOrDeclaration>),
    // expression statement
    Expression(Box<Expression>),
    // selection statement
    If(
        Box<Expression>,
        /// true statement
        Box<Statement>,
        /// false statement
        Option<Box<Statement>>,
    ),
    Switch(Box<Expression>, Box<Statement>),
    // iteration statement
    While(Box<Expression>, Box<Statement>),
    DoWhile(Box<Statement>, Box<Expression>),
    For(
        /// initialize clause
        Option<Box<ForInitClause>>,
        /// condition expression
        Option<Box<Expression>>,
        /// iteration expression
        Option<Box<Expression>>,
        /// loop statement
        Box<Statement>,
    ),
    // jump statement
    Break,
    Continue,
    Return(Option<Box<Expression>>),
    Goto(String),
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum Expression {
    Assignment(
        AssignOperation,
        /// left hand side
        Box<Expression>,
        /// right hand side
        Box<Expression>,
    ),
    Unary(UnaryOperation, Box<Expression>),
    Binary(
        BinaryOperation,
        /// left hand side
        Box<Expression>,
        /// right hand side
        Box<Expression>,
    ),
    FunctionCall(
        /// function
        Box<Expression>,
        /// arguments
        Vec<Expression>,
    ),
    TypeCast(BasicType, Box<Expression>),
    Conditional(
        /// condition
        Box<Expression>,
        /// true expression
        Box<Expression>,
        /// false expression
        Box<Expression>,
    ),
    SizeofType(BasicType),
    MemberOfObject(
        /// object
        Box<Expression>,
        /// member name
        String,
    ),
    MemberOfPointer(
        /// pointer
        Box<Expression>,
        /// member name
        String,
    ),
    ArraySubscript(
        /// array
        Box<Expression>,
        /// index
        Box<Expression>,
    ),

    Identifier(String),
    IntegerConstant(i32),
    UnsignedIntegerConstant(u32),
    LongConstant(i64),
    UnsignedLongConstant(u64),
    LongLongConstant(i64),
    UnsignedLongLongConstant(u64),
    CharacterConstant(char),
    FloatConstant(f32),
    DoubleConstant(f64),
    StringLiteral(String),
    Empty,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum ForInitClause {
    Expression(Expression),
    ForDeclaration(Vec<Declaration>),
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum StatementOrDeclaration {
    Statement(Statement),
    LocalDeclaration(Declaration),
}

impl Default for Statement {
    fn default() -> Self {
        Statement::Expression(Default::default())
    }
}

impl Default for Expression {
    fn default() -> Self {
        Expression::Empty
    }
}
