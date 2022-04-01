use pest::iterators::Pair;

use super::*;

pub fn build_declaration(ast: &mut Vec<Declaration>, pair: Pair<'_, Rule>) {
    let mut basic_type: Type = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::declaration_specifiers => {
                basic_type = build_declaration_specifiers(ast, token);
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

pub fn build_declaration_specifiers(ast: &mut Vec<Declaration>, pair: Pair<'_, Rule>) -> Type {
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
                base_type = build_type_specifier(ast, token);
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
            Default::default()
        },
        basic_type: BasicType {
            qualifier,
            base_type,
        },
    }
}

pub fn build_type_specifier(ast: &mut Vec<Declaration>, pair: Pair<'_, Rule>) -> BaseType {
    let token = pair.into_inner().next().unwrap();
    match token.as_rule() {
        Rule::void_ => BaseType::Void,
        Rule::char_ => BaseType::Char,
        Rule::int_ => BaseType::Int,
        Rule::bool_ => BaseType::Bool,
        Rule::float_ => BaseType::Float,
        Rule::double_ => BaseType::Double,
        Rule::identifier => BaseType::Identifier(token.as_str().to_string()),
        Rule::struct_specifier => build_struct_specifier(ast, token),
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
                            build_raw_declarator(
                                ast,
                                &mut derived_type,
                                &mut identifier,
                                sub_token,
                            );
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
    // TODO(TO/GA): throw error if derived_type is not a function but have a function specifier
    // TODO(TO/GA): throw error if derived_type is a function that return sth. but has noreturn specifier
    ast.push(Declaration::Declaration(
        derived_type,
        Some(identifier),
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
    ast: &mut Vec<Declaration>,
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
                derived_type.basic_type.base_type = BaseType::Array(
                    Box::new(derived_type.basic_type.to_owned()),
                    Box::new(build_assignment_expression(token)),
                );
                derived_type.basic_type.qualifier = Default::default();
            }
            Rule::function_parameter_list => {
                build_function_parameter_list(ast, derived_type, token);
            }
            _ => unreachable!(),
        }
    }
}

pub fn build_function_parameter_list(
    ast: &mut Vec<Declaration>,
    derived_type: &mut Type,
    pair: Pair<'_, Rule>,
) -> Vec<String> {
    let mut is_variadic = false;
    let mut parameter_list: Vec<BasicType> = Default::default();
    let mut parameter_name: Vec<String> = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::function_parameter => {
                let parameter = build_function_parameter(ast, token);
                parameter_list.push(parameter.0);
                parameter_name.push(parameter.1);
            }
            Rule::variadic_argument_ => {
                is_variadic = true;
            }
            _ => unreachable!(),
        }
    }
    derived_type.basic_type.base_type = BaseType::Function(
        Box::new(derived_type.basic_type.to_owned()),
        parameter_list,
        is_variadic,
    );
    derived_type.basic_type.qualifier = Default::default();
    parameter_name
}

pub fn build_function_parameter(
    ast: &mut Vec<Declaration>,
    pair: Pair<'_, Rule>,
) -> (BasicType, String) {
    let mut basic_type: BasicType = Default::default();
    let mut identifier: String = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::declaration_specifiers => {
                basic_type = build_declaration_specifiers(ast, token).basic_type;
            }
            Rule::function_parameter_declarator => {
                build_function_parameter_declarator(ast, &mut basic_type, &mut identifier, token);
            }
            _ => unreachable!(),
        }
    }
    (basic_type, identifier)
}

pub fn build_function_parameter_declarator(
    ast: &mut Vec<Declaration>,
    basic_type: &mut BasicType,
    identifier: &mut String,
    pair: Pair<'_, Rule>,
) {
    let mut derived_type = Type {
        function_specifier: Default::default(),
        storage_class_specifier: Default::default(),
        basic_type: basic_type.to_owned(),
    };
    let mut identifier: String = Default::default();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::pointer => {
                build_pointer(&mut derived_type, token);
            }
            Rule::function_parameter_raw_declarator => {
                build_raw_declarator(ast, &mut derived_type, &mut identifier, token);
            }
            _ => unreachable!(),
        }
    }
    *basic_type = derived_type.basic_type;
}

pub fn build_struct_specifier(ast: &mut Vec<Declaration>, pair: Pair<'_, Rule>) -> BaseType {
    let mut is_struct = true;
    let mut identifier: Option<String> = None;
    let mut struct_members: Vec<StructMember> = Default::default();

    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::struct_ => {
                is_struct = true;
            }
            Rule::union_ => {
                is_struct = false;
            }
            Rule::identifier => {
                identifier = Some(token.as_str().to_string());
            }
            Rule::struct_declaration => {
                for sub_token in token.into_inner() {
                    match sub_token.as_rule() {
                        Rule::declaration => {
                            let mut sub_ast = Vec::new();
                            build_declaration(&mut sub_ast, sub_token);
                            for declaration in sub_ast {
                                match declaration {
                                    Declaration::Declaration(
                                        member_type,
                                        member_name,
                                        member_initializer,
                                    ) => {
                                        let member_name = match member_name {
                                            Some(name) => name,
                                            None => panic!("struct member name is None"), // TODO(TO/GA): throw error
                                        };
                                        if member_initializer.is_some() {
                                            panic!("struct member initializer is not None");
                                            // TODO(TO/GA): throw error
                                        }
                                        struct_members.push(StructMember {
                                            member_type: member_type.basic_type, // TODO(TO/GA): throw error if it has StorageClassSpecifier
                                            member_name,
                                        });
                                    }
                                    Declaration::FunctionDefinition(_, _, _, _) => {
                                        // TODO(TO/GA): throw error
                                        unreachable!();
                                    }
                                }
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    let ret = match is_struct {
        true => BaseType::Struct(
            identifier,
            match struct_members.is_empty() {
                true => None,
                false => Some(struct_members),
            },
        ),
        false => BaseType::Union(
            identifier,
            match struct_members.is_empty() {
                true => None,
                false => Some(struct_members),
            },
        ),
    };

    ast.push(Declaration::Declaration(
        Type {
            function_specifier: Default::default(),
            storage_class_specifier: Default::default(),
            basic_type: BasicType {
                qualifier: Default::default(),
                base_type: ret.clone(),
            },
        },
        None,
        None,
    ));
    ret
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
