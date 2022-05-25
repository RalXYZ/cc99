use pest::Span;
use serde::{Serialize, Serializer};
use std::fmt;

use super::operations::*;
use super::types::*;

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum AST<'a> {
    GlobalDeclaration(Vec<Declaration<'a>>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Declaration<'a> {
    pub node: DeclarationEnum<'a>,
    pub span: Span<'a>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum DeclarationEnum<'a> {
    Declaration(
        Type<'a>,
        /// identifier (if it's a struct/union declaration, it might be None)
        Option<String>,
        /// initializer
        Option<Box<Expression<'a>>>,
    ),
    FunctionDefinition(
        Vec<FunctionSpecifier>,
        StorageClassSpecifier,
        /// return type
        Box<BasicType<'a>>,
        /// identifier
        String,
        /// parameters and their names
        Vec<(BasicType<'a>, Option<String>)>,
        /// is variadic
        bool,
        /// body
        Statement<'a>,
    ),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Statement<'a> {
    pub node: StatementEnum<'a>,
    pub span: Span<'a>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum StatementEnum<'a> {
    // labeled statement
    Labeled(String, Box<Statement<'a>>),
    Case(
        /// None represents default case
        Option<Box<Expression<'a>>>,
        Box<Statement<'a>>,
    ),
    // compound statement
    Compound(Vec<StatementOrDeclaration<'a>>),
    // expression statement
    Expression(Box<Expression<'a>>),
    // selection statement
    If(
        Box<Expression<'a>>,
        /// true statement
        Box<Statement<'a>>,
        /// false statement
        Option<Box<Statement<'a>>>,
    ),
    Switch(Box<Expression<'a>>, Box<Statement<'a>>),
    // iteration statement
    While(Box<Expression<'a>>, Box<Statement<'a>>),
    DoWhile(Box<Statement<'a>>, Box<Expression<'a>>),
    For(
        /// initialize clause
        Option<Box<ForInitClause<'a>>>,
        /// condition expression
        Option<Box<Expression<'a>>>,
        /// iteration expression
        Option<Box<Expression<'a>>>,
        /// loop statement
        Box<Statement<'a>>,
    ),
    // jump statement
    Break,
    Continue,
    Return(Option<Box<Expression<'a>>>),
    Goto(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Expression<'a> {
    pub node: ExpressionEnum<'a>,
    pub span: Span<'a>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum ExpressionEnum<'a> {
    Assignment(
        AssignOperation<'a>,
        /// left hand side
        Box<Expression<'a>>,
        /// right hand side
        Box<Expression<'a>>,
    ),
    Unary(UnaryOperation<'a>, Box<Expression<'a>>),
    Binary(
        BinaryOperation<'a>,
        /// left hand side
        Box<Expression<'a>>,
        /// right hand side
        Box<Expression<'a>>,
    ),
    FunctionCall(
        /// function
        Box<Expression<'a>>,
        /// arguments
        Vec<Expression<'a>>,
    ),
    TypeCast(BasicType<'a>, Box<Expression<'a>>),
    Conditional(
        /// condition
        Box<Expression<'a>>,
        /// true expression
        Box<Expression<'a>>,
        /// false expression
        Box<Expression<'a>>,
    ),
    SizeofType(BasicType<'a>),
    MemberOfObject(
        /// object
        Box<Expression<'a>>,
        /// member name
        String,
    ),
    MemberOfPointer(
        /// pointer
        Box<Expression<'a>>,
        /// member name
        String,
    ),
    ArraySubscript(
        /// array
        Box<Expression<'a>>,
        /// index
        Vec<Expression<'a>>,
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
pub struct ForInitClause<'a> {
    pub node: ForInitClauseEnum<'a>,
    pub span: Span<'a>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum ForInitClauseEnum<'a> {
    Expression(Expression<'a>),
    ForDeclaration(Vec<Declaration<'a>>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct StatementOrDeclaration<'a> {
    pub node: StatementOrDeclarationEnum<'a>,
    pub span: Span<'a>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum StatementOrDeclarationEnum<'a> {
    Statement(Statement<'a>),
    LocalDeclaration(Declaration<'a>),
}

impl<'a> Statement<'a> {
    pub fn default(code: &'a str) -> Self {
        Statement {
            node: StatementEnum::Expression(Box::new(Expression::default(code))),
            span: Span::new(code, 0, 0).unwrap(),
        }
    }
}

impl<'a> Expression<'a> {
    pub fn default(code: &'a str) -> Self {
        Expression {
            node: ExpressionEnum::Empty,
            span: Span::new(code, 0, 0).unwrap(),
        }
    }
}

impl fmt::Display for Expression<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Serialize for Declaration<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.node.serialize(serializer)
    }
}

impl Serialize for Statement<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.node.serialize(serializer)
    }
}

impl Serialize for Expression<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.node.serialize(serializer)
    }
}

impl Serialize for ForInitClause<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.node.serialize(serializer)
    }
}

impl Serialize for StatementOrDeclaration<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.node.serialize(serializer)
    }
}
