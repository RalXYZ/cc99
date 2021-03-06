use pest::error::ErrorVariant;
use pest::iterators::Pair;

use super::*;

impl Parse {
    pub fn build_function_definition(
        &mut self,
        ast: &mut Vec<Declaration>,
        pair: Pair<'_, Rule>,
    ) -> Result<(), Box<dyn Error>> {
        let span = pair.as_span();
        let mut derived_type: Type = Default::default();
        let mut identifier: String = Default::default();
        let mut parameter_names: Vec<Option<String>> = Default::default();
        let mut function_body = Default::default();
        for token in pair.into_inner() {
            match token.as_rule() {
                Rule::declaration_specifiers => {
                    derived_type = self.build_declaration_specifiers(ast, token)?;
                }
                Rule::pointer => {
                    self.build_pointer(&mut derived_type, token)?;
                }
                Rule::identifier => {
                    identifier = token.as_str().to_string();
                }
                Rule::function_parameter_list => {
                    parameter_names =
                        self.build_function_parameter_list(ast, &mut derived_type, token)?;
                }
                Rule::compound_statement => {
                    function_body = self.build_compound_statement(token)?;
                }
                _ => unreachable!(),
            }
        }

        // throw error if derived_type is a function that return sth. but has noreturn specifier
        match &derived_type.basic_type.base_type {
            BaseType::Function(return_type, _, _) => {
                if return_type.base_type != BaseType::Void
                    && derived_type
                        .function_specifier
                        .contains(&FunctionSpecifier::Noreturn)
                {
                    return Err(Box::new(pest::error::Error::<Rule>::new_from_span(
                        ErrorVariant::CustomError {
                            message: "function with return value is marked as _Noreturn"
                                .to_string(),
                        },
                        span,
                    )));
                }
            }
            _ => unreachable!(),
        }

        if let BaseType::Function(mut return_type, parameter_types, is_variadic) =
            derived_type.basic_type.base_type
        {
            return_type
                .qualifier
                .extend(derived_type.basic_type.qualifier);
            let parameters = parameter_types
                .into_iter()
                .zip(parameter_names.into_iter())
                .collect::<Vec<_>>();

            ast.push(Declaration {
                node: DeclarationEnum::FunctionDefinition(
                    derived_type.function_specifier,
                    derived_type.storage_class_specifier,
                    return_type,
                    identifier,
                    parameters,
                    is_variadic,
                    function_body,
                ),
                span: Span::from(span),
            });
            Ok(())
        } else {
            unreachable!()
        }
    }

    pub fn build_declaration(
        &mut self,
        ast: &mut Vec<Declaration>,
        pair: Pair<'_, Rule>,
    ) -> Result<(), Box<dyn Error>> {
        let mut basic_type: Type = Default::default();
        for token in pair.into_inner() {
            match token.as_rule() {
                Rule::declaration_specifiers => {
                    basic_type = self.build_declaration_specifiers(ast, token)?;
                }
                Rule::declarator_and_initializer_list => {
                    for list_entry in token.into_inner() {
                        match list_entry.as_rule() {
                            Rule::declarator_and_initializer => {
                                self.build_declarator_and_initializer(
                                    ast,
                                    list_entry,
                                    &basic_type,
                                )?;
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        Ok(())
    }

    pub fn build_declaration_specifiers(
        &mut self,
        ast: &mut Vec<Declaration>,
        pair: Pair<'_, Rule>,
    ) -> Result<Type, Box<dyn Error>> {
        let span = pair.as_span();
        let mut qualifier: Vec<TypeQualifier> = Default::default();
        let mut storage_class_specifier: Vec<StorageClassSpecifier> = Default::default();
        let mut function_specifier: Vec<FunctionSpecifier> = Default::default();
        let mut base_type: BaseType = Default::default();
        for token in pair.into_inner() {
            match token.as_rule() {
                Rule::storage_class_specifier => match token.into_inner().next().unwrap().as_rule()
                {
                    Rule::typedef_ => {
                        storage_class_specifier.push(StorageClassSpecifier::Typedef);
                    }
                    Rule::extern_ => {
                        storage_class_specifier.push(StorageClassSpecifier::Extern);
                    }
                    Rule::static_ => {
                        storage_class_specifier.push(StorageClassSpecifier::Static);
                    }
                    Rule::thread_local_ => {
                        storage_class_specifier.push(StorageClassSpecifier::ThreadLocal);
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
                Rule::type_qualifier => qualifier.push(self.build_type_qualifier(token)?),
                Rule::type_specifier => {
                    base_type = self.build_type_specifier(ast, token)?;
                }
                _ => unreachable!(),
            }
        }

        assert!(storage_class_specifier.len() <= 1);
        if let BaseType::Function(_, _, _) = base_type {
            if storage_class_specifier[0] == StorageClassSpecifier::Register {
                return Err(Box::new(pest::error::Error::<Rule>::new_from_span(
                    ErrorVariant::CustomError {
                        message: "register storage class specifier is not allowed for function"
                            .to_string(),
                    },
                    span,
                )));
            }
            if storage_class_specifier[0] == StorageClassSpecifier::ThreadLocal {
                return Err(Box::new(pest::error::Error::<Rule>::new_from_span(
                    ErrorVariant::CustomError {
                        message: "thread local storage class specifier is not allowed for function"
                            .to_string(),
                    },
                    span,
                )));
            }
        }

        Ok(Type {
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
        })
    }

    fn build_type_specifier(
        &mut self,
        ast: &mut Vec<Declaration>,
        pair: Pair<'_, Rule>,
    ) -> Result<BaseType, Box<dyn Error>> {
        let mut is_signed = true;
        let mut integer_type = IntegerType::Int;
        let token = pair.into_inner().next().unwrap();
        match token.as_rule() {
            Rule::void_ => return Ok(BaseType::Void),
            Rule::signed_ => is_signed = true,
            Rule::unsigned_ => is_signed = false,
            Rule::char_ => integer_type = IntegerType::Char,
            Rule::short_ => integer_type = IntegerType::Short,
            Rule::int_ => integer_type = IntegerType::Int,
            Rule::long_ => {
                integer_type = match integer_type {
                    IntegerType::Int => IntegerType::Long,
                    IntegerType::Long => IntegerType::LongLong,
                    _ => unreachable!(),
                }
            }
            Rule::bool_ => return Ok(BaseType::Bool),
            Rule::float_ => return Ok(BaseType::Float),
            Rule::double_ => return Ok(BaseType::Double),
            Rule::identifier => return Ok(BaseType::Identifier(token.as_str().to_string())),
            Rule::struct_specifier => return self.build_struct_specifier(ast, token),
            _ => unreachable!(),
        }
        if is_signed {
            Ok(BaseType::SignedInteger(integer_type))
        } else {
            Ok(BaseType::UnsignedInteger(integer_type))
        }
    }

    pub fn build_declarator_and_initializer(
        &mut self,
        ast: &mut Vec<Declaration>,
        pair: Pair<'_, Rule>,
        basic_type: &Type,
    ) -> Result<(), Box<dyn Error>> {
        let span = pair.as_span();
        let mut derived_type = (*basic_type).clone();
        let mut identifier: String = Default::default();
        let mut initializer: Option<Box<Expression>> = None;
        for token in pair.into_inner() {
            match token.as_rule() {
                Rule::declarator => {
                    for sub_token in token.into_inner() {
                        match sub_token.as_rule() {
                            Rule::pointer => {
                                self.build_pointer(&mut derived_type, sub_token)?;
                            }
                            Rule::raw_declarator => {
                                self.build_raw_declarator(
                                    ast,
                                    &mut derived_type,
                                    &mut identifier,
                                    sub_token,
                                )?;
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                Rule::assignment_expression => {
                    initializer = Some(Box::new(self.build_assignment_expression(token)?));
                }
                _ => unreachable!(),
            }
        }

        match &derived_type.basic_type.base_type {
            BaseType::Function(return_type, _, _) => {
                // throw error if derived_type is a function that return sth. but has noreturn specifier
                if return_type.base_type != BaseType::Void
                    && derived_type
                        .function_specifier
                        .contains(&FunctionSpecifier::Noreturn)
                {
                    return Err(Box::new(pest::error::Error::<Rule>::new_from_span(
                        ErrorVariant::CustomError {
                            message: "function with return value is marked as _Noreturn"
                                .to_string(),
                        },
                        span,
                    )));
                }
            }
            _ => {
                // throw error if derived_type is not a function but have function specifiers
                if !derived_type.function_specifier.is_empty() {
                    return Err(Box::new(pest::error::Error::<Rule>::new_from_span(
                        ErrorVariant::CustomError {
                            message: "non-function type can't have function specifiers".to_string(),
                        },
                        span,
                    )));
                }
            }
        }

        ast.push(Declaration {
            node: DeclarationEnum::Declaration(derived_type, Some(identifier), initializer),
            span: Span::from(span),
        });
        Ok(())
    }

    pub fn build_pointer(
        &mut self,
        derived_type: &mut Type,
        pair: Pair<'_, Rule>,
    ) -> Result<(), Box<dyn Error>> {
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
                        .push(self.build_type_qualifier(token)?);
                }
                _ => unreachable!(),
            }
        }
        Ok(())
    }

    fn build_raw_declarator(
        &mut self,
        ast: &mut Vec<Declaration>,
        derived_type: &mut Type,
        identifier: &mut String,
        pair: Pair<'_, Rule>,
    ) -> Result<(), Box<dyn Error>> {
        let mut dimensions: Vec<Expression> = Default::default();
        for token in pair.into_inner() {
            match token.as_rule() {
                Rule::identifier => {
                    *identifier = token.as_str().to_string();
                }
                Rule::assignment_expression => {
                    dimensions.push(self.build_assignment_expression(token)?);
                }
                Rule::function_parameter_list => {
                    self.build_function_parameter_list(ast, derived_type, token)?;
                }
                _ => unreachable!(),
            }
        }
        if !dimensions.is_empty() {
            derived_type.basic_type.base_type =
                BaseType::Array(Box::new(derived_type.basic_type.to_owned()), dimensions);
            derived_type.basic_type.qualifier = Default::default();
        }
        Ok(())
    }

    pub fn build_function_parameter_list(
        &mut self,
        ast: &mut Vec<Declaration>,
        derived_type: &mut Type,
        pair: Pair<'_, Rule>,
    ) -> Result<Vec<Option<String>>, Box<dyn Error>> {
        let mut is_variadic = false;
        let mut parameter_list: Vec<BasicType> = Default::default();
        let mut parameter_name: Vec<Option<String>> = Default::default();
        for token in pair.into_inner() {
            match token.as_rule() {
                Rule::function_parameter => {
                    let parameter = self.build_function_parameter(ast, token)?;
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
        Ok(parameter_name)
    }

    fn build_function_parameter(
        &mut self,
        ast: &mut Vec<Declaration>,
        pair: Pair<'_, Rule>,
    ) -> Result<(BasicType, Option<String>), Box<dyn Error>> {
        let mut basic_type: BasicType = Default::default();
        let mut identifier: Option<String> = None;
        for token in pair.into_inner() {
            match token.as_rule() {
                Rule::declaration_specifiers => {
                    basic_type = self.build_declaration_specifiers(ast, token)?.basic_type;
                }
                Rule::function_parameter_declarator => {
                    self.build_function_parameter_declarator(
                        ast,
                        &mut basic_type,
                        &mut identifier,
                        token,
                    )?;
                }
                _ => unreachable!(),
            }
        }
        Ok((basic_type, identifier))
    }

    fn build_function_parameter_declarator(
        &mut self,
        ast: &mut Vec<Declaration>,
        basic_type: &mut BasicType,
        identifier: &mut Option<String>,
        pair: Pair<'_, Rule>,
    ) -> Result<(), Box<dyn Error>> {
        let mut derived_type = Type {
            function_specifier: Default::default(),
            storage_class_specifier: Default::default(),
            basic_type: basic_type.to_owned(),
        };
        for token in pair.into_inner() {
            match token.as_rule() {
                Rule::pointer => {
                    self.build_pointer(&mut derived_type, token)?;
                }
                Rule::function_parameter_raw_declarator => {
                    let mut identifier_: String = Default::default();
                    self.build_raw_declarator(ast, &mut derived_type, &mut identifier_, token)?;
                    if !identifier_.is_empty() {
                        *identifier = Some(identifier_.to_string());
                    }
                }
                _ => unreachable!(),
            }
        }
        *basic_type = derived_type.basic_type;
        Ok(())
    }

    fn build_struct_specifier(
        &mut self,
        ast: &mut Vec<Declaration>,
        pair: Pair<'_, Rule>,
    ) -> Result<BaseType, Box<dyn Error>> {
        let span = pair.as_span();
        let mut is_struct = true;
        let mut identifier: Option<String> = None;
        let mut struct_declaration = false;
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
                    struct_declaration = true;
                    for sub_token in token.into_inner() {
                        let sub_span = sub_token.as_span();
                        match sub_token.as_rule() {
                            Rule::declaration => {
                                let mut sub_ast = Vec::new();
                                self.build_declaration(&mut sub_ast, sub_token)?;
                                for declaration in sub_ast {
                                    match declaration.node {
                                        DeclarationEnum::Declaration(
                                            member_type,
                                            member_name,
                                            member_initializer,
                                        ) => {
                                            let member_name = match member_name {
                                                Some(name) => name,
                                                None => {
                                                    return Err(Box::new(
                                                        pest::error::Error::<Rule>::new_from_span(
                                                            ErrorVariant::CustomError {
                                                                message:
                                                                    "expected struct member name"
                                                                        .to_string(),
                                                            },
                                                            sub_span,
                                                        ),
                                                    ));
                                                }
                                            };
                                            if member_initializer.is_some() {
                                                return Err(Box::new(
                                                pest::error::Error::<Rule>::new_from_span(
                                                    ErrorVariant::CustomError {
                                                        message:
                                                            "struct member can't have initializer"
                                                                .to_string(),
                                                    },
                                                    sub_span,
                                                ),
                                            ));
                                            }
                                            if member_type.storage_class_specifier
                                                != StorageClassSpecifier::Auto
                                            {
                                                return Err(Box::new(
                                                pest::error::Error::<Rule>::new_from_span(
                                                    ErrorVariant::CustomError {
                                                        message:
                                                            "struct member can't have storage class specifiers"
                                                                .to_string(),
                                                    },
                                                    sub_span,
                                                ),
                                            ));
                                            }
                                            if let BaseType::Function(_, _, _) =
                                                member_type.basic_type.base_type
                                            {
                                                return Err(Box::new(
                                                pest::error::Error::<Rule>::new_from_span(
                                                    ErrorVariant::CustomError {
                                                        message:
                                                            "struct member can't be function type"
                                                                .to_string(),
                                                    },
                                                    sub_span,
                                                ),
                                            ));
                                            }
                                            struct_members.push(StructMember {
                                                member_type: member_type.basic_type,
                                                member_name,
                                            });
                                        }
                                        DeclarationEnum::FunctionDefinition(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => {
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

        let struct_definition = match is_struct {
            true => BaseType::Struct(
                identifier.clone(),
                match struct_declaration {
                    false => None,
                    true => Some(struct_members),
                },
            ),
            false => BaseType::Union(
                identifier.clone(),
                match struct_declaration {
                    false => None,
                    true => Some(struct_members),
                },
            ),
        };

        if identifier.is_none() {
            return Ok(struct_definition);
        }

        if struct_declaration {
            ast.push(Declaration {
                node: DeclarationEnum::Declaration(
                    Type {
                        function_specifier: Default::default(),
                        storage_class_specifier: Default::default(),
                        basic_type: BasicType {
                            qualifier: Default::default(),
                            base_type: struct_definition,
                        },
                    },
                    None,
                    None,
                ),
                span: Span::from(span),
            });
        }

        Ok(match is_struct {
            true => BaseType::Struct(identifier, None),
            false => BaseType::Union(identifier, None),
        })
    }

    fn build_type_qualifier(
        &mut self,
        pair: Pair<'_, Rule>,
    ) -> Result<TypeQualifier, Box<dyn Error>> {
        let token = pair.into_inner().next().unwrap();
        Ok(match token.as_rule() {
            Rule::const_ => TypeQualifier::Const,
            Rule::volatile_ => TypeQualifier::Volatile,
            Rule::restrict_ => TypeQualifier::Restrict,
            Rule::atomic_ => TypeQualifier::Atomic,
            _ => unreachable!(),
        })
    }
}
