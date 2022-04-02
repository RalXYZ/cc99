use pest::iterators::Pair;

use super::*;

pub fn build_function_definition(ast: &mut Vec<Declaration>, pair: Pair<'_, Rule>) {
    let mut derived_type: Type = Default::default();
    let mut identifier: String = Default::default();
    let mut parameter_names: Vec<Option<String>> = Default::default();
    let mut function_body: Statement = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::declaration_specifiers => {
                derived_type = build_declaration_specifiers(ast, token);
            }
            Rule::pointer => {
                build_pointer(&mut derived_type, token);
            }
            Rule::identifier => {
                identifier = token.as_str().to_string();
            }
            Rule::function_parameter_list => {
                parameter_names = build_function_parameter_list(ast, &mut derived_type, token);
            }
            Rule::compound_statement => {
                function_body = build_compound_statement(token);
            }
            _ => unreachable!(),
        }
    }
    ast.push(Declaration::FunctionDefinition(
        derived_type,
        identifier,
        parameter_names,
        function_body,
    ));
}

pub fn build_compound_statement(pair: Pair<'_, Rule>) -> Statement {
    Default::default()
}

pub fn build_assignment_expression(pair: Pair<'_, Rule>) -> Expression {
    Default::default()
}
