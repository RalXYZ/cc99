use pest::iterators::Pair;

use super::*;

pub fn build_declaration(ast: &mut Vec<Declaration>, pair: Pair<'_, Rule>) {
    let mut basic_type: Type = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::declaration_specifiers => {
                basic_type = build_declaration_specifiers(token);
            }
            Rule::declarator_and_initializer_list => {
                for list_entry in token.into_inner() {
                    match list_entry.as_rule() {
                        Rule::declarator_and_initializer => {
                            build_declarator_and_initializer(ast, list_entry, &basic_type);
                        }
                        _ => unreachable!(),
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}

pub fn build_declaration_specifiers(pair: Pair<'_, Rule>) -> Type {
    let mut qualifier: Vec<TypeQualifier> = Default::default();
    let mut storage_class_specifier: Vec<StorageClassSpecifier> = Default::default();
    let mut function_specifier: Vec<FunctionSpecifier> = Default::default();
    let mut base_type: BaseType = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::storage_class_specifier => match token.into_inner().next().unwrap().as_rule() {
                Rule::typedef_ => {
                    storage_class_specifier.push(StorageClassSpecifier::Typedef);
                }
                Rule::extern_ => {
                    storage_class_specifier.push(StorageClassSpecifier::Extern);
                }
                Rule::static_ => {
                    storage_class_specifier.push(StorageClassSpecifier::Static);
                }
                Rule::auto_ => {
                    storage_class_specifier.push(StorageClassSpecifier::Auto);
                }
                Rule::register_ => {
                    storage_class_specifier.push(StorageClassSpecifier::Register);
                }
                _ => unreachable!(),
            },
            Rule::function_specifier => match token.into_inner().next().unwrap().as_rule() {
                Rule::inline_ => {
                    function_specifier.push(FunctionSpecifier::Inline);
                }
                Rule::noreturn_ => {
                    function_specifier.push(FunctionSpecifier::Noreturn);
                }
                _ => unreachable!(),
            },
            Rule::type_qualifier => qualifier.push(build_type_qualifier(token)),
            Rule::type_specifier => {
                base_type = build_type_specifier(token);
            }
            _ => unreachable!(),
        }
    }
    assert!(storage_class_specifier.len() <= 1);
    Type {
        function_specifier,
        storage_class_specifier: if !storage_class_specifier.is_empty() {
            storage_class_specifier[0].to_owned()
        } else {
            StorageClassSpecifier::Auto
        },
        basic_type: BasicType {
            qualifier,
            base_type,
        },
    }
}

pub fn build_type_specifier(pair: Pair<'_, Rule>) -> BaseType {
    let token = pair.into_inner().next().unwrap();
    match token.as_rule() {
        Rule::void_ => BaseType::Void,
        Rule::char_ => BaseType::Char,
        Rule::int_ => BaseType::Int,
        Rule::bool_ => BaseType::Bool,
        Rule::float_ => BaseType::Float,
        Rule::double_ => BaseType::Double,
        Rule::identifier => BaseType::Identifier(token.as_str().to_string()),
        Rule::struct_specifier => {
            // TODO(TO/GA)
            unreachable!()
        }
        _ => unreachable!(),
    }
}

pub fn build_declarator_and_initializer(
    ast: &mut Vec<Declaration>,
    pair: Pair<'_, Rule>,
    basic_type: &Type,
) {
    let mut derived_type = (*basic_type).clone();
    let mut identifier: String = Default::default();
    let mut initializer: Option<Box<Expression>> = None;
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::declarator => {
                for sub_token in token.into_inner() {
                    match sub_token.as_rule() {
                        Rule::pointer => {
                            build_pointer(&mut derived_type, sub_token);
                        }
                        Rule::raw_declarator => {
                            build_raw_declarator(&mut derived_type, &mut identifier, sub_token);
                        }
                        _ => unreachable!(),
                    }
                }
            }
            Rule::assignment_expression => {
                initializer = Some(Box::new(build_assignment_expression(token)));
            }
            _ => unreachable!(),
        }
    }
    ast.push(Declaration::Declaration(
        derived_type,
        identifier,
        initializer,
    ));
}

pub fn build_pointer(derived_type: &mut Type, pair: Pair<'_, Rule>) {
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::star_ => {
                derived_type.basic_type.base_type =
                    BaseType::Pointer(Box::new(derived_type.basic_type.to_owned()));
                derived_type.basic_type.qualifier = Default::default();
            }
            Rule::type_qualifier => {
                derived_type
                    .basic_type
                    .qualifier
                    .push(build_type_qualifier(token));
            }
            _ => unreachable!(),
        }
    }
}

pub fn build_raw_declarator(
    derived_type: &mut Type,
    identifier: &mut String,
    pair: Pair<'_, Rule>,
) {
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::identifier => {
                *identifier = token.as_str().to_string();
            }
            Rule::assignment_expression => {
                // TODO(TO/GA)
                unreachable!()
            }
            Rule::function_parameter_list => {
                // TODO(TO/GA)
                unreachable!()
            }
            _ => unreachable!(),
        }
    }
}

pub fn build_type_qualifier(pair: Pair<'_, Rule>) -> TypeQualifier {
    let token = pair.into_inner().next().unwrap();
    match token.as_rule() {
        Rule::const_ => TypeQualifier::Const,
        Rule::volatile_ => TypeQualifier::Volatile,
        Rule::restrict_ => TypeQualifier::Restrict,
        Rule::atomic_ => TypeQualifier::Atomic,
        _ => unreachable!(),
    }
}
