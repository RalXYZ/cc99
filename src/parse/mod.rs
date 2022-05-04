mod declaration;
mod expression;
mod literal;
mod statement;

use pest::Parser;
use serde::Serialize;
use std::error::Error;

use super::ast::*;
use declaration::*;
use expression::*;
use literal::*;
use statement::*;

#[derive(Parser, Serialize)]
#[grammar = "./parse/parse.pest"]
struct CC99Parser;

pub fn parse(code: &str) -> Result<Box<AST>, Box<dyn Error>> {
    let tokens = match CC99Parser::parse(Rule::cc99, code)?.next() {
        Some(p) => p.into_inner(),
        None => unreachable!(),
    };
    let mut ast = Vec::new();
    for token in tokens {
        match token.as_rule() {
            Rule::declaration => {
                build_declaration(&mut ast, token)?;
            }
            Rule::function_definition => {
                build_function_definition(&mut ast, token)?;
            }
            Rule::EOI => {}
            _ => unreachable!(),
        }
    }
    Ok(Box::new(AST::GlobalDeclarations(ast)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variable_declaration() {
        let code = r#"static int const *const x, y;"#;
        assert_eq!(
            parse(code).unwrap(),
            Box::new(AST::GlobalDeclarations(vec![
                Declaration::GlobalDeclaration(
                    Type {
                        function_specifier: vec![],
                        storage_class_specifier: StorageClassSpecifier::Static,
                        basic_type: BasicType {
                            qualifier: vec![TypeQualifier::Const],
                            base_type: BaseType::Pointer(Box::new(BasicType {
                                qualifier: vec![TypeQualifier::Const],
                                base_type: Default::default(),
                            })),
                        },
                    },
                    Some("x".to_string()),
                    None,
                ),
                Declaration::GlobalDeclaration(
                    Type {
                        function_specifier: vec![],
                        storage_class_specifier: StorageClassSpecifier::Static,
                        basic_type: BasicType {
                            qualifier: vec![TypeQualifier::Const],
                            base_type: Default::default(),
                        },
                    },
                    Some("y".to_string()),
                    None,
                ),
            ]))
        );
    }

    #[test]
    fn function_declaration() {
        let code = r#"inline static const int foo(const int x, float *, ...);"#;
        assert_eq!(
            parse(code).unwrap(),
            Box::new(AST::GlobalDeclarations(vec![
                Declaration::GlobalDeclaration(
                    Type {
                        function_specifier: vec!(FunctionSpecifier::Inline),
                        storage_class_specifier: StorageClassSpecifier::Static,
                        basic_type: BasicType {
                            qualifier: vec![],
                            base_type: BaseType::Function(
                                Box::new(BasicType {
                                    qualifier: vec![TypeQualifier::Const],
                                    base_type: Default::default(),
                                }),
                                vec![
                                    BasicType {
                                        qualifier: vec![TypeQualifier::Const],
                                        base_type: Default::default(),
                                    },
                                    BasicType {
                                        qualifier: vec![],
                                        base_type: BaseType::Pointer(Box::new(BasicType {
                                            qualifier: vec![],
                                            base_type: BaseType::Float,
                                        })),
                                    },
                                ],
                                true
                            ),
                        },
                    },
                    Some("foo".to_string()),
                    None
                ),
            ]))
        );
    }

    #[test]
    fn typedef_struct_declaration() {
        let code = r#"typedef struct Xxx{const int y; float z;} x;"#;
        assert_eq!(
            parse(code).unwrap(),
            Box::new(AST::GlobalDeclarations(vec![
                Declaration::GlobalDeclaration(
                    Type {
                        function_specifier: vec!(),
                        storage_class_specifier: StorageClassSpecifier::Auto,
                        basic_type: BasicType {
                            qualifier: vec![],
                            base_type: BaseType::Struct(
                                Some("Xxx".to_string()),
                                Some(vec![
                                    StructMember {
                                        member_name: "y".to_string(),
                                        member_type: BasicType {
                                            qualifier: vec![TypeQualifier::Const],
                                            base_type: Default::default(),
                                        },
                                    },
                                    StructMember {
                                        member_name: "z".to_string(),
                                        member_type: BasicType {
                                            qualifier: vec![],
                                            base_type: BaseType::Float,
                                        },
                                    },
                                ]),
                            ),
                        }
                    },
                    None,
                    None
                ),
                Declaration::GlobalDeclaration(
                    Type {
                        function_specifier: vec!(),
                        storage_class_specifier: StorageClassSpecifier::Typedef,
                        basic_type: BasicType {
                            qualifier: vec![],
                            base_type: BaseType::Struct(Some("Xxx".to_string()), None),
                        }
                    },
                    Some("x".to_string()),
                    None
                )
            ]))
        );
    }

    #[test]
    fn function_definition() {
        let code = r#"inline int *const bar(int x, float) {}"#;
        assert_eq!(
            parse(code).unwrap(),
            Box::new(AST::GlobalDeclarations(vec![
                Declaration::FunctionDefinition(
                    Type {
                        function_specifier: vec![FunctionSpecifier::Inline],
                        storage_class_specifier: StorageClassSpecifier::Auto,
                        basic_type: BasicType {
                            qualifier: vec![],
                            base_type: BaseType::Function(
                                Box::new(BasicType {
                                    qualifier: vec![TypeQualifier::Const],
                                    base_type: BaseType::Pointer(Box::new(BasicType {
                                        qualifier: vec![],
                                        base_type: Default::default(),
                                    })),
                                }),
                                vec![
                                    BasicType {
                                        qualifier: vec![],
                                        base_type: Default::default(),
                                    },
                                    BasicType {
                                        qualifier: vec![],
                                        base_type: BaseType::Float,
                                    },
                                ],
                                false
                            ),
                        },
                    },
                    "bar".to_string(),
                    vec![Some("x".to_string()), None],
                    Statement::Compound(vec![]),
                ),
            ]))
        );
    }
}
