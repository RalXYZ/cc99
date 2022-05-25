use serde::{Serialize, Serializer};
use std::fmt;

use super::operations::*;
use super::span::*;
use super::types::*;

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum AST {
    GlobalDeclaration(Vec<Declaration>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Declaration {
    pub node: DeclarationEnum,
    pub span: Span,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum DeclarationEnum {
    Declaration(
        Type,
        /// identifier (if it's a struct/union declaration, it might be None)
        Option<String>,
        /// initializer
        Option<Box<Expression>>,
    ),
    FunctionDefinition(
        Vec<FunctionSpecifier>,
        StorageClassSpecifier,
        /// return type
        Box<BasicType>,
        /// identifier
        String,
        /// parameters and their names
        Vec<(BasicType, Option<String>)>,
        /// is variadic
        bool,
        /// body
        Statement,
    ),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Statement {
    pub node: StatementEnum,
    pub span: Span,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum StatementEnum {
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

#[derive(Debug, PartialEq, Clone)]
pub struct Expression {
    pub node: ExpressionEnum,
    pub span: Span,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum ExpressionEnum {
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
        Vec<Expression>,
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

#[derive(Debug, PartialEq, Clone)]
pub struct ForInitClause {
    pub node: ForInitClauseEnum,
    pub span: Span,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum ForInitClauseEnum {
    Expression(Expression),
    ForDeclaration(Vec<Declaration>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct StatementOrDeclaration {
    pub node: StatementOrDeclarationEnum,
    pub span: Span,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum StatementOrDeclarationEnum {
    Statement(Statement),
    LocalDeclaration(Declaration),
}

impl Default for Statement {
    fn default() -> Self {
        Statement {
            node: StatementEnum::Expression(Box::new(Default::default())),
            span: Default::default(),
        }
    }
}

impl Default for Expression {
    fn default() -> Self {
        Expression {
            node: ExpressionEnum::Empty,
            span: Default::default(),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Serialize for Declaration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.node.serialize(serializer)
    }
}

impl Serialize for Statement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.node.serialize(serializer)
    }
}

impl Serialize for Expression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.node.serialize(serializer)
    }
}

impl Serialize for ForInitClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.node.serialize(serializer)
    }
}

impl Serialize for StatementOrDeclaration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.node.serialize(serializer)
    }
}
