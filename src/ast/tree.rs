use serde::Serialize;

use super::operations::*;
use super::types::*;

#[derive(Serialize, Debug)]
pub enum AST {
    GlobalDeclaration(Vec<Declaration>),
}

#[derive(Serialize, Debug)]
pub enum Declaration {
    Declaration(Type, Vec<DeclaratorAndInitializer>),
    FunctionDefinition(
        Type,
        String,
        /// parameters
        Vec<Option<String>>,
        Vec<Statement>,
    ),
}

#[derive(Serialize, Debug)]
pub struct DeclaratorAndInitializer {
    pub declarator: String,
    pub initializer: Option<Box<Expression>>,
}

#[derive(Serialize, Debug)]
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
    Switch(Box<Expression>, Vec<Statement>),
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

#[derive(Serialize, Debug)]
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
    FunctionCall(String, Vec<Expression>),
    TypeCast(Type, Box<Expression>),
    Conditional(
        /// condition
        Box<Expression>,
        /// true expression
        Box<Expression>,
        /// false expression
        Box<Expression>,
    ),
    Sizeof(Box<Expression>),
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

    Identifier(String),
    BoolLiteral(bool),
    IntLiteral(i32),
    FloatLiteral(f32),
    StringLiteral(String),
}

#[derive(Serialize, Debug)]
pub enum ForInitClause {
    Expression(Expression),
    Declaration(Declaration),
}

#[derive(Serialize, Debug)]
pub enum StatementOrDeclaration {
    Statement(Statement),
    Declaration(Declaration),
}
