mod declaration;
mod expression;
mod literal;
mod statement;

use pest::Parser;
use serde::Serialize;
use std::error::Error;

use super::ast::*;

#[derive(Parser, Serialize)]
#[grammar = "./parse/parse.pest"]
struct CC99Parser;

#[derive(Default)]
pub struct Parse {}

impl Parse {
    pub fn new() -> Parse {
        Default::default()
    }

    pub fn parse(&mut self, code: &str) -> Result<Box<AST>, Box<dyn Error>> {
        let tokens = match CC99Parser::parse(Rule::cc99, code)?.next() {
            Some(p) => p.into_inner(),
            None => unreachable!(),
        };
        let mut ast = Vec::new();
        for token in tokens {
            match token.as_rule() {
                Rule::declaration => {
                    self.build_declaration(&mut ast, token)?;
                }
                Rule::function_definition => {
                    self.build_function_definition(&mut ast, token)?;
                }
                Rule::EOI => {}
                _ => unreachable!(),
            }
        }
        Ok(Box::new(AST::GlobalDeclaration(ast)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variable_declaration() {
        let code = r#"static int const *const x, y;"#;
        assert_eq!(
            Parse::new().parse(code).unwrap(),
            Box::new(AST::GlobalDeclaration(vec![
                Declaration {
                    node: DeclarationEnum::Declaration(
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
                    span: Span::new(17, 25)
                },
                Declaration {
                    node: DeclarationEnum::Declaration(
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
                    span: Span::new(27, 28)
                },
            ]))
        );
    }

    #[test]
    fn function_declaration() {
        let code = r#"inline static const int foo(const int x, float *, ...);"#;
        assert_eq!(
            Parse::new().parse(code).unwrap(),
            Box::new(AST::GlobalDeclaration(vec![Declaration {
                node: DeclarationEnum::Declaration(
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
                span: Span::new(24, 54)
            },]))
        );
    }

    #[test]
    fn typedef_struct_declaration() {
        let code = r#"typedef struct Xxx{const int y; float z;} x;"#;
        assert_eq!(
            Parse::new().parse(code).unwrap(),
            Box::new(AST::GlobalDeclaration(vec![
                Declaration {
                    node: DeclarationEnum::Declaration(
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
                    span: Span::new(8, 41)
                },
                Declaration {
                    node: DeclarationEnum::Declaration(
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
                    ),
                    span: Span::new(42, 43)
                }
            ]))
        );
    }

    #[test]
    fn function_definition() {
        let code = r#"inline int *const bar(int x, float) {}"#;
        assert_eq!(
            Parse::new().parse(code).unwrap(),
            Box::new(AST::GlobalDeclaration(vec![Declaration {
                node: DeclarationEnum::FunctionDefinition(
                    vec![FunctionSpecifier::Inline],
                    StorageClassSpecifier::Auto,
                    Box::new(BasicType {
                        qualifier: vec![TypeQualifier::Const],
                        base_type: BaseType::Pointer(Box::new(BasicType {
                            qualifier: vec![],
                            base_type: BaseType::SignedInteger(IntegerType::Int),
                        })),
                    }),
                    "bar".to_string(),
                    vec![
                        (
                            BasicType {
                                qualifier: vec![],
                                base_type: Default::default(),
                            },
                            Some("x".to_string())
                        ),
                        (
                            BasicType {
                                qualifier: vec![],
                                base_type: BaseType::Float,
                            },
                            None
                        ),
                    ],
                    false,
                    Statement {
                        node: StatementEnum::Compound(vec![]),
                        span: Span::new(36, 38)
                    },
                ),
                span: Span::new(0, 38)
            },]))
        );
    }

    #[test]
    fn array() {
        let code = r#"const int x[10][9][8];"#;
        assert_eq!(
            Parse::new().parse(code).unwrap(),
            Box::new(AST::GlobalDeclaration(vec![Declaration {
                node: DeclarationEnum::Declaration(
                    Type {
                        function_specifier: vec![],
                        storage_class_specifier: StorageClassSpecifier::Auto,
                        basic_type: BasicType {
                            qualifier: vec![],
                            base_type: BaseType::Array(
                                Box::new(BasicType {
                                    qualifier: vec![TypeQualifier::Const],
                                    base_type: BaseType::SignedInteger(IntegerType::Int),
                                }),
                                vec![
                                    Expression {
                                        node: ExpressionEnum::IntegerConstant(10),
                                        span: Span::new(12, 14)
                                    },
                                    Expression {
                                        node: ExpressionEnum::IntegerConstant(9),
                                        span: Span::new(16, 17)
                                    },
                                    Expression {
                                        node: ExpressionEnum::IntegerConstant(8),
                                        span: Span::new(19, 20)
                                    },
                                ],
                            ),
                        },
                    },
                    Some("x".to_string()),
                    None
                ),
                span: Span::new(10, 21)
            },]))
        );
    }
}
