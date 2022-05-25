mod declaration;
mod expression;
mod literal;
mod statement;

use pest::{Parser, Span};
use serde::Serialize;
use std::error::Error;

use super::ast::*;

#[derive(Parser, Serialize)]
#[grammar = "./parse/parse.pest"]
struct CC99Parser;

pub struct Parse<'ctx> {
    code: &'ctx str,
}

impl<'ctx> Parse<'ctx> {
    pub fn new(code: &'ctx str) -> Parse<'ctx> {
        Parse { code }
    }

    pub fn parse(&mut self) -> Result<Box<AST<'ctx>>, Box<dyn Error>> {
        let tokens = match CC99Parser::parse(Rule::cc99, self.code)?.next() {
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
            Parse::new(code).parse().unwrap(),
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
                    span: Span::new(code, 17, 25).unwrap()
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
                    span: Span::new(code, 27, 28).unwrap()
                },
            ]))
        );
    }

    #[test]
    fn function_declaration() {
        let code = r#"inline static const int foo(const int x, float *, ...);"#;
        assert_eq!(
            Parse::new(code).parse().unwrap(),
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
                span: Span::new(code, 24, 54).unwrap()
            },]))
        );
    }

    #[test]
    fn typedef_struct_declaration() {
        let code = r#"typedef struct Xxx{const int y; float z;} x;"#;
        assert_eq!(
            Parse::new(code).parse().unwrap(),
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
                    span: Span::new(code, 8, 41).unwrap()
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
                    span: Span::new(code, 42, 43).unwrap()
                }
            ]))
        );
    }

    #[test]
    fn function_definition() {
        let code = r#"inline int *const bar(int x, float) {}"#;
        assert_eq!(
            Parse::new(code).parse().unwrap(),
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
                        span: Span::new(code, 36, 38).unwrap()
                    },
                ),
                span: Span::new(code, 0, 38).unwrap()
            },]))
        );
    }

    #[test]
    fn array() {
        let code = r#"const int x[10][9][8];"#;
        assert_eq!(
            Parse::new(code).parse().unwrap(),
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
                                        span: Span::new(code, 12, 14).unwrap()
                                    },
                                    Expression {
                                        node: ExpressionEnum::IntegerConstant(9),
                                        span: Span::new(code, 16, 17).unwrap()
                                    },
                                    Expression {
                                        node: ExpressionEnum::IntegerConstant(8),
                                        span: Span::new(code, 19, 20).unwrap()
                                    },
                                ],
                            ),
                        },
                    },
                    Some("x".to_string()),
                    None
                ),
                span: Span::new(code, 10, 21).unwrap()
            },]))
        );
    }
}
