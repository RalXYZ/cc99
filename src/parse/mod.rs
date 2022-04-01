mod declaration;
mod function_definition;

use pest::Parser;
use serde::Serialize;
use std::error::Error;

use super::ast::*;
use declaration::*;
use function_definition::*;

#[derive(Parser, Serialize)]
#[grammar = "./parse/parse.pest"]
pub struct CC99Parser;

// TODO(TO/GA): error handling
pub fn parse(code: &str) -> Result<Box<AST>, Box<dyn Error>> {
    let tokens = match CC99Parser::parse(Rule::cc99, code)
        .unwrap_or_else(|e| panic!("{}", e))
        .next()
    {
        Some(p) => p.into_inner(),
        None => panic!("Fail to parse an empty file"),
    };
    let mut ast = Vec::new();
    for token in tokens {
        match token.as_rule() {
            Rule::declaration => {
                build_declaration(&mut ast, token);
            }
            Rule::function_definition => {
                build_function_definition(&mut ast, token);
            }
            Rule::EOI => {}
            _ => unreachable!(),
        }
    }
    Ok(Box::new(AST::GlobalDeclaration(ast)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variable_declaration() {
        let code = r#"static int const *const x, y;"#;
        assert_eq!(
            parse(code).unwrap(),
            Box::new(AST::GlobalDeclaration(vec![
                Declaration::Declaration(
                    Type {
                        qualifier: vec![TypeQualifier::Const],
                        function_specifier: vec![],
                        storage_class_specifier: StorageClassSpecifier::Static,
                        basic_type: BasicType::Pointer(Box::new(Type {
                            qualifier: vec![TypeQualifier::Const],
                            function_specifier: vec![],
                            storage_class_specifier: StorageClassSpecifier::Auto,
                            basic_type: BasicType::Int,
                        })),
                    },
                    "x".to_string(),
                    None,
                ),
                Declaration::Declaration(
                    Type {
                        qualifier: vec![TypeQualifier::Const],
                        function_specifier: vec![],
                        storage_class_specifier: StorageClassSpecifier::Static,
                        basic_type: BasicType::Int,
                    },
                    "y".to_string(),
                    None,
                ),
            ]))
        );
    }
}
