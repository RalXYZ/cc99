use pest::Parser;
use serde::Serialize;

use super::ast::*;

#[derive(Parser, Serialize)]
#[grammar = "./parse/parse.pest"]
pub struct CC99Parser;

pub fn parse(code: &str) -> Box<AST> {
    Box::new(AST::GlobalDeclaration(vec![]))
}

#[cfg(test)]
mod tests {
    use super::*;
}
