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
                Declaration::Declaration(
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
            Box::new(AST::GlobalDeclaration(vec![Declaration::Declaration(
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
            ),]))
        );
    }

    #[test]
    fn typedef_struct_declaration() {
        let code = r#"typedef struct Xxx{const int y; float z;} x;"#;
        let basic_type = BasicType {
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
        };
        assert_eq!(
            parse(code).unwrap(),
            Box::new(AST::GlobalDeclaration(vec![
                Declaration::Declaration(
                    Type {
                        function_specifier: vec!(),
                        storage_class_specifier: StorageClassSpecifier::Auto,
                        basic_type: basic_type.clone()
                    },
                    None,
                    None
                ),
                Declaration::Declaration(
                    Type {
                        function_specifier: vec!(),
                        storage_class_specifier: StorageClassSpecifier::Typedef,
                        basic_type
                    },
                    Some("x".to_string()),
                    None
                )
            ]))
        );
    }

    #[test]
    fn function_definition() {
        let code = r#"_Noreturn int *const bar(int x, float) {}"#;
        assert_eq!(
            parse(code).unwrap(),
            Box::new(AST::GlobalDeclaration(vec![
                Declaration::FunctionDefinition(
                    Type {
                        function_specifier: vec![FunctionSpecifier::Noreturn],
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
