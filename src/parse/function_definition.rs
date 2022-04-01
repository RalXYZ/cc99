use pest::iterators::Pair;

use super::*;

pub fn build_function_definition(ast: &mut Vec<Declaration>, pair: Pair<'_, Rule>) {}

pub fn build_assignment_expression(pair: Pair<'_, Rule>) -> Expression {
    Expression::IntLiteral(0)
}
