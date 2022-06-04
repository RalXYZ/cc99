use crate::ast::{
    AssignOperation, AssignOperationEnum, BaseType, BasicType, BinaryOperation,
    BinaryOperationEnum, Expression, ExpressionEnum, IntegerType, Span, UnaryOperation,
    UnaryOperationEnum,
};
use crate::generator::Generator;
use crate::utils::CompileErr as CE;
use inkwell::values::{
    AnyValue, BasicMetadataValueEnum, BasicValue, BasicValueEnum, FloatValue, IntValue,
    PointerValue,
};
use inkwell::{FloatPredicate, IntPredicate};
use std::cmp::Ordering;
use std::fmt::Error;

impl<'ctx> Generator<'ctx> {
    pub(crate) fn gen_expression(
        &self,
        expr: &Expression,
    ) -> Result<(BaseType, BasicValueEnum<'ctx>), CE> {
        match expr.node {
            ExpressionEnum::Empty => Ok((
                BaseType::Void,
                self.context
                    .i8_type()
                    .const_int(0_u64, false)
                    .as_basic_value_enum(),
            )),
            ExpressionEnum::Assignment(ref op, ref lhs, ref rhs) => {
                self.gen_assignment(op, lhs, rhs, expr.span)
            }
            ExpressionEnum::Unary(ref op, ref expr) => self.gen_unary_expr(op, expr, expr.span),
            ExpressionEnum::Binary(ref op, ref lhs, ref rhs) => {
                self.gen_binary_expr(op, lhs, rhs, expr.span)
            }
            ExpressionEnum::FunctionCall(ref name, ref args) => {
                self.gen_function_call(name, args, expr.span)
            }
            ExpressionEnum::MemberOfObject(ref obj, ref member) => {
                let (t, p_v) = self.gen_member_of_object(obj, member, expr.span)?;
                Ok((
                    t.base_type,
                    self.builder.build_load(p_v, "member_of_object"),
                ))
            }
            ExpressionEnum::CharacterConstant(ref value) => Ok((
                BaseType::SignedInteger(IntegerType::Char),
                self.context
                    .i8_type()
                    .const_int(*value as u64, false)
                    .as_basic_value_enum(),
            )),
            ExpressionEnum::IntegerConstant(ref value) => Ok((
                BaseType::SignedInteger(IntegerType::Int),
                self.context
                    .i32_type()
                    .const_int(*value as u64, false)
                    .as_basic_value_enum(),
            )),
            ExpressionEnum::UnsignedIntegerConstant(ref value) => Ok((
                BaseType::UnsignedInteger(IntegerType::Int),
                self.context
                    .i32_type()
                    .const_int(*value as u64, false)
                    .as_basic_value_enum(),
            )),
            ExpressionEnum::LongConstant(ref value)
            | ExpressionEnum::LongLongConstant(ref value) => Ok((
                BaseType::SignedInteger(IntegerType::Long),
                self.context
                    .i64_type()
                    .const_int(*value as u64, false)
                    .as_basic_value_enum(),
            )),
            ExpressionEnum::UnsignedLongConstant(ref value)
            | ExpressionEnum::UnsignedLongLongConstant(ref value) => Ok((
                BaseType::UnsignedInteger(IntegerType::Long),
                self.context
                    .i64_type()
                    .const_int(*value as u64, false)
                    .as_basic_value_enum(),
            )),
            ExpressionEnum::FloatConstant(ref value) => Ok((
                BaseType::Float,
                self.context
                    .f32_type()
                    .const_float(*value as f64)
                    .as_basic_value_enum(),
            )),
            ExpressionEnum::DoubleConstant(ref value) => Ok((
                BaseType::Float,
                self.context
                    .f64_type()
                    .const_float(*value as f64)
                    .as_basic_value_enum(),
            )),
            ExpressionEnum::Identifier(ref string_literal) => {
                //if BaseType is Array, we just return Array type but don't load value!!
                let deref = self.get_variable(string_literal, expr.span)?;
                let val = if let BaseType::Array(_, _) = deref.0.base_type {
                    deref.1.as_basic_value_enum()
                } else {
                    self.builder.build_load(deref.1, "load_val")
                };
                Ok((deref.0.base_type, val))
            }
            ExpressionEnum::StringLiteral(ref string) => Ok((
                BaseType::Pointer(Box::new(BasicType {
                    qualifier: vec![],
                    base_type: BaseType::SignedInteger(IntegerType::Char),
                })),
                self.builder
                    .build_global_string_ptr(string.as_str(), "str")
                    .as_basic_value_enum(),
            )),
            ExpressionEnum::ArraySubscript(ref id_expr, ref idx_vec) => {
                let (l_t, mut l_pv) = self.get_lvalue(id_expr)?;

                //if type is pointer, get_variable will get the address of pointer, not the pointer point to!
                //So if we want get the point to address, we need extra load action!
                if let BaseType::Pointer(_) = l_t.base_type {
                    // println!("{}", l_pv.get_type().print_to_string().to_string());
                    l_pv = self
                        .builder
                        .build_load(l_pv, "dereference")
                        .into_pointer_value();
                }
                // println!("{}", l_pv.get_type().print_to_string().to_string());
                let (res_t, mut idx_int_val_vec) =
                    self.process_arr_subscript(&l_t, idx_vec.clone(), expr.span)?;
                //Pointer
                if let BaseType::Pointer(_) = l_t.base_type {
                    while idx_int_val_vec.len() > 0 {
                        l_pv = unsafe {
                            self.builder.build_gep(
                                l_pv,
                                [idx_int_val_vec[0]].as_ref(),
                                "pointer_subscript",
                            )
                        };
                        idx_int_val_vec.remove(0);
                        if idx_int_val_vec.len() > 0 {
                            l_pv = self
                                .builder
                                .build_load(l_pv, "dereference")
                                .into_pointer_value()
                        }
                    }
                    Ok((
                        res_t.base_type,
                        self.builder.build_load(l_pv, "dereference"),
                    ))
                } else {
                    //Array
                    if let BaseType::Array(_, _) = res_t.base_type {
                        Ok((res_t.base_type, unsafe {
                            self.builder
                                .build_gep(l_pv, idx_int_val_vec.as_ref(), "arr_subscript")
                                .as_basic_value_enum()
                        }))
                    } else {
                        Ok((
                            res_t.base_type,
                            self.builder.build_load(
                                unsafe {
                                    self.builder.build_gep(
                                        l_pv,
                                        idx_int_val_vec.as_ref(),
                                        "arr_subscript",
                                    )
                                },
                                "load_arr_subscript",
                            ),
                        ))
                    }
                }
            }
            _ => Err(CE::unknown_expression(expr.span)),
        }
    }

    fn process_arr_subscript(
        &self,
        l_t: &BasicType,
        idx_vec: Vec<Expression>,
        span: Span,
    ) -> Result<(BasicType, Vec<IntValue<'ctx>>), CE> {
        let true_l_t = match l_t.base_type {
            BaseType::Identifier(ref name) => {
                if let Some(typedef) = self.typedef_map.get(name) {
                    typedef
                } else {
                    unreachable!()
                }
            }
            _ => l_t,
        };
        if let BaseType::Array(ref arr_t, arr_len_vec) = &true_l_t.base_type {
            let res_t = match idx_vec.len().cmp(&arr_len_vec.len()) {
                Ordering::Less => BaseType::Array(
                    arr_t.clone(),
                    (idx_vec.len()..arr_len_vec.len()).fold(vec![], |mut acc, i| {
                        acc.push(arr_len_vec[i].clone());
                        acc
                    }),
                ),
                Ordering::Equal => arr_t.base_type.clone(),
                Ordering::Greater => {
                    return Err(CE::array_dimension_mismatch(
                        arr_len_vec.len(),
                        idx_vec.len(),
                        span,
                    ));
                }
            };

            let mut idx_int_val_vec = vec![self.context.i32_type().const_zero()];
            idx_int_val_vec.extend(
                idx_vec
                    .iter()
                    .map(|expr| self.gen_expression(expr).unwrap().1.into_int_value()),
            );
            Ok((
                BasicType {
                    qualifier: true_l_t.qualifier.clone(),
                    base_type: res_t,
                },
                idx_int_val_vec,
            ))
        } else if let BaseType::Pointer(_) = true_l_t.base_type {
            let mut res_t = &true_l_t.base_type;

            let mut idx_int_val_vec = vec![];

            let mut not_match = false;
            idx_int_val_vec.extend(idx_vec.iter().map(|expr| {
                match res_t {
                    BaseType::Pointer(p) => {
                        res_t = &p.base_type;
                    }
                    _ => {
                        not_match = true;
                    }
                }
                self.gen_expression(expr).unwrap().1.into_int_value()
            }));
            if not_match {
                //NOT found the expect number now!
                return Err(CE::pointer_dimension_mismatch(0, idx_vec.len(), span));
            }
            Ok((
                BasicType {
                    qualifier: true_l_t.qualifier.clone(),
                    base_type: res_t.clone(),
                },
                idx_int_val_vec,
            ))
        } else {
            unreachable!()
        }
    }

    fn gen_unary_expr(
        &self,
        op: &UnaryOperation,
        expr: &Expression,
        span: Span,
    ) -> Result<(BaseType, BasicValueEnum<'ctx>), CE> {
        let (expr_type, expr_value) = self.gen_expression(expr)?;

        match op.node {
            UnaryOperationEnum::UnaryPlus => match expr_type {
                BaseType::Bool
                | BaseType::SignedInteger(_)
                | BaseType::UnsignedInteger(_)
                | BaseType::Float
                | BaseType::Double => Ok((expr_type, expr_value)),
                _ => Err(CE::invalid_unary(span)),
            },
            UnaryOperationEnum::UnaryMinus => match expr_type {
                BaseType::Bool | BaseType::SignedInteger(_) | BaseType::UnsignedInteger(_) => Ok((
                    expr_type,
                    self.builder
                        .build_int_neg(expr_value.into_int_value(), "int_neg")
                        .as_basic_value_enum(),
                )),
                BaseType::Float | BaseType::Double => Ok((
                    expr_type,
                    self.builder
                        .build_float_neg(expr_value.into_float_value(), "float_neg")
                        .as_basic_value_enum(),
                )),
                _ => Err(CE::invalid_unary(span)),
            },
            UnaryOperationEnum::BitwiseNot => match expr_type {
                BaseType::SignedInteger(_) | BaseType::UnsignedInteger(_) => Ok((
                    expr_type,
                    self.builder
                        .build_not(expr_value.into_int_value(), "bitwise_not")
                        .as_basic_value_enum(),
                )),
                _ => Err(CE::invalid_unary(span)),
            },
            UnaryOperationEnum::LogicalNot => match expr_type {
                BaseType::SignedInteger(_) => {
                    let llvm_type = self.convert_llvm_type(&expr_type);
                    let result_int = self.builder.build_int_compare(
                        IntPredicate::EQ,
                        llvm_type.into_int_type().const_int(0_u64, true),
                        expr_value.into_int_value(),
                        "logical_not_result_int",
                    );

                    let llvm_type = self.convert_llvm_type(&BaseType::Bool);
                    Ok((
                        BaseType::Bool,
                        self.builder
                            .build_int_cast(
                                result_int,
                                llvm_type.into_int_type(),
                                "cast_logical_not_result_to_bool",
                            )
                            .as_basic_value_enum(),
                    ))
                }
                _ => Err(CE::invalid_unary(span)),
            },
            UnaryOperationEnum::Reference => {
                let (t, ptr) = self.get_lvalue(expr)?;
                Ok((BaseType::Pointer(Box::new(t)), ptr.as_basic_value_enum()))
            }
            UnaryOperationEnum::Dereference => match expr_type {
                BaseType::Pointer(ref t) => Ok((
                    t.base_type.clone(),
                    self.builder
                        .build_load(expr_value.into_pointer_value(), "dereference")
                        .as_basic_value_enum(),
                )),
                _ => Err(CE::invalid_unary(span)),
            },
            UnaryOperationEnum::PostfixDecrement | UnaryOperationEnum::PostfixIncrement => {
                match expr_type {
                    BaseType::SignedInteger(_)
                    | BaseType::UnsignedInteger(_)
                    | BaseType::Double
                    | BaseType::Float
                    | BaseType::Pointer(_) => {
                        let assign_op = if op.node == UnaryOperationEnum::PostfixIncrement {
                            AssignOperation {
                                node: AssignOperationEnum::Addition,
                                span: op.span,
                            }
                        } else {
                            AssignOperation {
                                node: AssignOperationEnum::Subtraction,
                                span: op.span,
                            }
                        };
                        let _ = self.gen_assignment(
                            &assign_op,
                            expr,
                            &Box::new(Expression {
                                node: ExpressionEnum::IntegerConstant(1),
                                span: op.span,
                            }),
                            span,
                        )?;
                        Ok((expr_type, expr_value))
                    }
                    _ => Err(CE::invalid_unary(span)),
                }
            }
            UnaryOperationEnum::PrefixIncrement | UnaryOperationEnum::PrefixDecrement => {
                match expr_type {
                    BaseType::SignedInteger(_)
                    | BaseType::UnsignedInteger(_)
                    | BaseType::Double
                    | BaseType::Float
                    | BaseType::Pointer(_) => {
                        let assign_op = if op.node == UnaryOperationEnum::PostfixIncrement {
                            AssignOperation {
                                node: AssignOperationEnum::Addition,
                                span: op.span,
                            }
                        } else {
                            AssignOperation {
                                node: AssignOperationEnum::Subtraction,
                                span: op.span,
                            }
                        };
                        self.gen_assignment(
                            &assign_op,
                            expr,
                            &Box::new(Expression {
                                node: ExpressionEnum::IntegerConstant(1),
                                span: op.span,
                            }),
                            span,
                        )
                    }
                    _ => Err(CE::invalid_unary(span)),
                }
            }
            _ => Err(CE::invalid_unary(span)),
        }
    }

    fn gen_binary_expr(
        &self,
        op: &BinaryOperation,
        lhs: &Expression,
        rhs: &Expression,
        span: Span,
    ) -> Result<(BaseType, BasicValueEnum<'ctx>), CE> {
        let (ref l_t, l_v) = self.gen_expression(lhs)?;
        let (ref r_t, r_v) = self.gen_expression(rhs)?;

        //TODO FIXME!! so dirty!
        let mut visit = 0;
        let tmp = match l_t {
            BaseType::SignedInteger(_) | BaseType::UnsignedInteger(_) => {
                if let BaseType::Pointer(_) = r_t {
                    visit = 1;
                    self.build_point_binary_op(op, r_v.into_pointer_value(), l_v.into_int_value())
                } else if let BaseType::Array(array_type, array_vec) = r_t {
                    visit = 1;
                    if array_vec.len() != 1 {
                        Err(CE::array_dimension_mismatch(1, array_vec.len(), span))
                    } else {
                        let point_v = self.cast_value(
                            r_t,
                            &r_v,
                            &BaseType::Pointer(array_type.clone()),
                            rhs.span,
                        )?;
                        self.build_point_binary_op(
                            op,
                            point_v.into_pointer_value(),
                            l_v.into_int_value(),
                        )
                    }
                } else {
                    Err(CE::plain_error("".to_string(), span))
                }
            }
            BaseType::Pointer(_) => match r_t {
                BaseType::SignedInteger(_) => {
                    visit = 2;
                    self.build_point_binary_op(op, l_v.into_pointer_value(), r_v.into_int_value())
                }
                BaseType::UnsignedInteger(_) => {
                    visit = 2;
                    self.build_point_binary_op(op, l_v.into_pointer_value(), r_v.into_int_value())
                }
                _ => Err(CE::plain_error("".to_string(), span)),
            },
            BaseType::Array(array_type, array_vec) => {
                visit = 2;
                if array_vec.len() != 1 {
                    Err(CE::array_dimension_mismatch(1, array_vec.len(), span))
                } else {
                    let point_v = self.cast_value(
                        l_t,
                        &l_v,
                        &BaseType::Pointer(array_type.clone()),
                        lhs.span,
                    )?;
                    self.build_point_binary_op(
                        op,
                        point_v.into_pointer_value(),
                        r_v.into_int_value(),
                    )
                }
            }
            _ => Err(CE::plain_error("".to_string(), span)),
        };
        if visit > 0 {
            return match tmp {
                Ok(t) => {
                    if visit == 1 {
                        Ok((r_t.clone(), t))
                    } else {
                        Ok((l_t.clone(), t))
                    }
                }
                Err(e) => Err(e),
            };
        }

        let cast_t = BaseType::upcast(l_t, r_t, &self.typedef_map)?;
        let l_cast_v = self.cast_value(l_t, &l_v, &cast_t, lhs.span)?;
        let r_cast_v = self.cast_value(r_t, &r_v, &cast_t, lhs.span)?;

        match cast_t {
            BaseType::Void => Err(CE::plain_error(
                "Invalid void type for binary operation".to_string(),
                span,
            )),
            BaseType::SignedInteger(_) | BaseType::UnsignedInteger(_) | BaseType::Bool => {
                let int_or_bool_value = self.build_int_binary_op(
                    op,
                    l_cast_v.into_int_value(),
                    r_cast_v.into_int_value(),
                    span,
                )?;
                // If value is bool
                if int_or_bool_value
                    .into_int_value()
                    .get_type()
                    .get_bit_width()
                    == 1
                {
                    Ok((BaseType::Bool, int_or_bool_value))
                } else {
                    Ok((cast_t, int_or_bool_value))
                }
            }
            BaseType::Float | BaseType::Double => {
                let float_or_bool_value = self.build_float_binary_op(
                    op,
                    l_cast_v.into_float_value(),
                    r_cast_v.into_float_value(),
                )?;
                // If value is bool(always float or int1)
                if float_or_bool_value.is_int_value() {
                    return Ok((BaseType::Bool, float_or_bool_value));
                }
                Ok((cast_t, float_or_bool_value))
            }
            _ => unimplemented!(),
        }
    }

    // left is PointerValue, right is IntValue
    fn build_point_binary_op(
        &self,
        op: &BinaryOperation,
        lhs: PointerValue<'ctx>,
        rhs: IntValue<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, CE> {
        let result_v = match op.node {
            BinaryOperationEnum::Addition => unsafe {
                let idx_int_val_vec = vec![rhs];
                self.builder
                    .build_gep(lhs, idx_int_val_vec.as_ref(), "pointer add")
            },
            BinaryOperationEnum::Subtraction => unsafe {
                let idx_int_val_vec = vec![rhs.const_neg()];
                self.builder
                    .build_gep(lhs, idx_int_val_vec.as_ref(), "pointer add")
            },

            // logical
            //TODO 先不管logical操作了QAQ
            _ => unimplemented!(),
        };
        Ok(result_v.as_basic_value_enum())
    }
    fn build_int_binary_op(
        &self,
        op: &BinaryOperation,
        lhs: IntValue<'ctx>,
        rhs: IntValue<'ctx>,
        span: Span,
    ) -> Result<BasicValueEnum<'ctx>, CE> {
        let result_v = match op.node {
            // arithmetic
            BinaryOperationEnum::Addition => self.builder.build_int_add(lhs, rhs, "int_add"),
            BinaryOperationEnum::Subtraction => self.builder.build_int_sub(lhs, rhs, "int_sub"),
            BinaryOperationEnum::Multiplication => self.builder.build_int_mul(lhs, rhs, "int_mul"),
            BinaryOperationEnum::Division => self.builder.build_int_signed_div(lhs, rhs, "int_div"),
            BinaryOperationEnum::Modulo => self.builder.build_int_signed_rem(lhs, rhs, "int_mod"),
            BinaryOperationEnum::BitwiseAnd => self.builder.build_and(lhs, rhs, "int_and"),
            BinaryOperationEnum::BitwiseOr => self.builder.build_or(lhs, rhs, "int_or"),
            BinaryOperationEnum::BitwiseXor => self.builder.build_xor(lhs, rhs, "int_xor"),
            BinaryOperationEnum::LeftShift => self.builder.build_left_shift(lhs, rhs, "int_shl"),
            BinaryOperationEnum::RightShift => {
                self.builder.build_right_shift(lhs, rhs, true, "int_shr")
            }
            // comparison
            BinaryOperationEnum::LessThan => {
                self.builder
                    .build_int_compare(IntPredicate::SLT, lhs, rhs, "int_lt")
            }
            BinaryOperationEnum::LessThanOrEqual => {
                self.builder
                    .build_int_compare(IntPredicate::SLE, lhs, rhs, "int_le")
            }
            BinaryOperationEnum::GreaterThan => {
                self.builder
                    .build_int_compare(IntPredicate::SGT, lhs, rhs, "int_gt")
            }
            BinaryOperationEnum::GreaterThanOrEqual => {
                self.builder
                    .build_int_compare(IntPredicate::SGE, lhs, rhs, "int_ge")
            }
            BinaryOperationEnum::Equal => {
                self.builder
                    .build_int_compare(IntPredicate::EQ, lhs, rhs, "int_eq")
            }
            BinaryOperationEnum::NotEqual => {
                self.builder
                    .build_int_compare(IntPredicate::NE, lhs, rhs, "int_ne")
            }
            // logical
            //TODO FIXME 这里and 和 or 是完全错误的，这个build_int_cast只是截取了最后一位！！
            BinaryOperationEnum::LogicalAnd => self.builder.build_and(
                self.builder
                    .build_int_cast(lhs, self.context.bool_type(), "cast i32 to i1_1"),
                self.builder
                    .build_int_cast(rhs, self.context.bool_type(), "cast i32 to i1_2"),
                "logical int and",
            ),
            BinaryOperationEnum::LogicalOr => self.builder.build_or(
                self.builder
                    .build_int_cast(lhs, self.context.bool_type(), "cast i32 to i1_1"),
                self.builder
                    .build_int_cast(rhs, self.context.bool_type(), "cast i32 to i1_2"),
                "logical int or",
            ),

            _ => return Err(CE::invalid_binary(span)),
        };

        Ok(result_v.as_basic_value_enum())
    }
    fn build_float_binary_op(
        &self,
        op: &BinaryOperation,
        lhs: FloatValue<'ctx>,
        rhs: FloatValue<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, CE> {
        let result_f = match op.node {
            // arithmetic
            BinaryOperationEnum::Addition => {
                Ok(self.builder.build_float_add(lhs, rhs, "float add"))
            }
            BinaryOperationEnum::Subtraction => {
                Ok(self.builder.build_float_sub(lhs, rhs, "float sub"))
            }
            BinaryOperationEnum::Multiplication => {
                Ok(self.builder.build_float_mul(lhs, rhs, "float mul"))
            }
            BinaryOperationEnum::Division => {
                Ok(self.builder.build_float_div(lhs, rhs, "float div"))
            }
            BinaryOperationEnum::Modulo => Ok(self.builder.build_float_rem(lhs, rhs, "float mod")),
            _ => Err(Error),
        };
        //return FloatValue
        if let Ok(result_f) = result_f {
            return Ok(result_f.as_basic_value_enum());
        }
        let result_i = match op.node {
            BinaryOperationEnum::LessThan => {
                self.builder
                    .build_float_compare(FloatPredicate::OLT, lhs, rhs, "float lt")
            }
            BinaryOperationEnum::LessThanOrEqual => {
                self.builder
                    .build_float_compare(FloatPredicate::OLE, lhs, rhs, "float le")
            }
            BinaryOperationEnum::GreaterThan => {
                self.builder
                    .build_float_compare(FloatPredicate::OGT, lhs, rhs, "float gt")
            }
            BinaryOperationEnum::GreaterThanOrEqual => {
                self.builder
                    .build_float_compare(FloatPredicate::OGE, lhs, rhs, "float ge")
            }
            BinaryOperationEnum::Equal => {
                self.builder
                    .build_float_compare(FloatPredicate::OEQ, lhs, rhs, "float eq")
            }
            BinaryOperationEnum::NotEqual => {
                self.builder
                    .build_float_compare(FloatPredicate::ONE, lhs, rhs, "float ne")
            }
            // logical
            //TODO FIXME 这里and 和 or 是完全错误的，这个build_int_cast只是截取了最后一位！！
            BinaryOperationEnum::LogicalAnd => self.builder.build_and(
                self.builder.build_int_cast(
                    self.builder
                        .build_cast(
                            self.gen_cast_llvm_instruction(
                                &BaseType::Double,
                                &BaseType::SignedInteger(IntegerType::Int),
                                op.span,
                            )?,
                            lhs,
                            self.context.i32_type(),
                            "cast float to i32",
                        )
                        .into_int_value(),
                    self.context.bool_type(),
                    "cast float to i1",
                ),
                self.builder.build_int_cast(
                    self.builder
                        .build_cast(
                            self.gen_cast_llvm_instruction(
                                &BaseType::Double,
                                &BaseType::SignedInteger(IntegerType::Int),
                                op.span,
                            )?,
                            rhs,
                            self.context.i32_type(),
                            "cast float to i32",
                        )
                        .into_int_value(),
                    self.context.bool_type(),
                    "cast float to i1",
                ),
                "logical float and",
            ),
            BinaryOperationEnum::LogicalOr => self.builder.build_or(
                self.builder.build_int_cast(
                    self.builder
                        .build_cast(
                            self.gen_cast_llvm_instruction(
                                &BaseType::Double,
                                &BaseType::SignedInteger(IntegerType::Int),
                                op.span,
                            )?,
                            lhs,
                            self.context.i32_type(),
                            "cast float to i32",
                        )
                        .into_int_value(),
                    self.context.bool_type(),
                    "cast float to i1",
                ),
                self.builder.build_int_cast(
                    self.builder
                        .build_cast(
                            self.gen_cast_llvm_instruction(
                                &BaseType::Double,
                                &BaseType::SignedInteger(IntegerType::Int),
                                op.span,
                            )?,
                            rhs,
                            self.context.i32_type(),
                            "cast float to i32",
                        )
                        .into_int_value(),
                    self.context.bool_type(),
                    "cast float to i1",
                ),
                "logical float or",
            ),
            _ => return Err(CE::invalid_binary(op.span)),
        };
        //return IntValue
        Ok(result_i.as_basic_value_enum())
    }
    pub(crate) fn gen_assignment(
        &self,
        op: &AssignOperation,
        lhs: &Expression,
        rhs: &Expression,
        span: Span,
    ) -> Result<(BaseType, BasicValueEnum<'ctx>), CE> {
        let (l_t, l_pv) = self.get_lvalue(lhs)?;

        let (r_t, r_v) = if let AssignOperationEnum::Naive = op.node {
            self.gen_expression(rhs)?
        } else {
            self.gen_binary_expr(
                &BinaryOperation {
                    node: match op.node {
                        AssignOperationEnum::Addition => BinaryOperationEnum::Addition,
                        AssignOperationEnum::Subtraction => BinaryOperationEnum::Subtraction,
                        AssignOperationEnum::Multiplication => BinaryOperationEnum::Multiplication,
                        AssignOperationEnum::Division => BinaryOperationEnum::Division,
                        AssignOperationEnum::Modulo => BinaryOperationEnum::Modulo,
                        AssignOperationEnum::BitwiseAnd => BinaryOperationEnum::BitwiseAnd,
                        AssignOperationEnum::BitwiseOr => BinaryOperationEnum::BitwiseOr,
                        AssignOperationEnum::BitwiseXor => BinaryOperationEnum::BitwiseXor,
                        AssignOperationEnum::LeftShift => BinaryOperationEnum::LeftShift,
                        AssignOperationEnum::RightShift => BinaryOperationEnum::RightShift,
                        AssignOperationEnum::Naive => unreachable!(),
                    },
                    span: op.span,
                },
                lhs,
                rhs,
                span,
            )?
        };

        r_t.test_cast(&l_t.base_type, rhs.span, &self.typedef_map)?;

        let cast_v = self.cast_value(&r_t, &r_v, &l_t.base_type, rhs.span)?;

        self.builder.build_store(l_pv, cast_v);

        Ok((l_t.base_type, cast_v))
    }

    fn get_lvalue(&self, lhs: &Expression) -> Result<(BasicType, PointerValue<'ctx>), CE> {
        match lhs.node {
            ExpressionEnum::Identifier(ref id) => Ok(self.get_variable(id, lhs.span)?),
            ExpressionEnum::Unary(ref op, ref unary_operation) => {
                // we need lhs expression's type!!! But now we don't have a function only get the type
                //TODO FIXME! It's really dirty
                let (t, _) = self.gen_expression(lhs)?;
                let (l_t, l_v) = self.gen_expression(unary_operation)?;
                if let UnaryOperationEnum::Dereference = op.node {
                    if let BaseType::Pointer(_) = l_t {
                        Ok((
                            BasicType {
                                qualifier: vec![],
                                base_type: t,
                            },
                            l_v.into_pointer_value(),
                        ))
                    } else {
                        Err(CE::invalid_dereference(
                            l_v.print_to_string().to_string(),
                            lhs.span,
                        ))
                    }
                } else if l_v.is_pointer_value() {
                    Ok((
                        BasicType {
                            qualifier: vec![],
                            base_type: t,
                        },
                        l_v.into_pointer_value(),
                    ))
                } else {
                    Err(CE::invalid_left_value(
                        l_v.print_to_string().to_string(),
                        lhs.span,
                    ))
                }
            }
            ExpressionEnum::ArraySubscript(ref id_expr, ref idx_vec) => {
                if let ExpressionEnum::Identifier(ref id) = id_expr.node {
                    let (t, mut pv) = self.get_variable(id, lhs.span)?;
                    //if type is pointer, get_variable will get the address of pointer, not the pointer point to!
                    //So if we want get the point to address, we need extra load action!
                    if let BaseType::Pointer(_) = t.base_type {
                        pv = self
                            .builder
                            .build_load(pv, "dereference")
                            .into_pointer_value()
                    }
                    let (res_t, mut idx_int_val_vec) =
                        self.process_arr_subscript(&t, idx_vec.clone(), lhs.span)?;
                    //Pointer
                    if let BaseType::Pointer(_) = t.base_type {
                        while idx_int_val_vec.len() > 0 {
                            pv = unsafe {
                                self.builder.build_gep(
                                    pv,
                                    [idx_int_val_vec[0]].as_ref(),
                                    "ptr_subscript",
                                )
                            };
                            idx_int_val_vec.remove(0);
                            if idx_int_val_vec.len() > 0 {
                                pv = self
                                    .builder
                                    .build_load(pv, "dereference")
                                    .into_pointer_value()
                            }
                        }
                        Ok((res_t, pv))
                    } else {
                        //array
                        Ok((res_t, unsafe {
                            self.builder
                                .build_gep(pv, idx_int_val_vec.as_ref(), "arr_subscript")
                        }))
                    }
                } else {
                    unreachable!()
                }
            }
            ExpressionEnum::MemberOfObject(ref id_expr, ref member_id) => {
                let (t, p_v) = self.gen_member_of_object(id_expr, member_id, lhs.span)?;
                Ok((t, p_v))
            }
            _ => panic!(),
        }
    }

    fn gen_function_call(
        &self,
        name: &Expression,
        args: &[Expression],
        span: Span,
    ) -> Result<(BaseType, BasicValueEnum<'ctx>), CE> {
        if let ExpressionEnum::Identifier(ref id) = name.node {
            let (ret_t, args_t, is_variadic) = self.function_map.get(id).unwrap().to_owned();
            let fv = self.module.get_function(id).unwrap();

            if args.len() != fv.get_type().count_param_types() as usize
                && !(is_variadic && args.len() > fv.get_type().count_param_types() as usize)
            {
                return Err(CE::parameter_count_mismatch(
                    id.to_string(),
                    fv.get_type().count_param_types() as usize,
                    args.len(),
                    span,
                ));
            }

            let mut casted_args = Vec::with_capacity(args.len());

            for (i, e) in args.iter().enumerate() {
                let t = args_t.get(i);

                match t {
                    Some(t) => {
                        let (e_t, e_v) = self.gen_expression(e)?;

                        e_t.test_cast(&t.base_type, e.span, &self.typedef_map)?;
                        let cast_v = self.cast_value(&e_t, &e_v, &t.base_type, e.span)?;

                        casted_args.push(BasicMetadataValueEnum::from(cast_v));
                    }
                    None => {
                        // variadic
                        let (_, e_v) = self.gen_expression(e)?;

                        casted_args.push(BasicMetadataValueEnum::from(e_v));
                    }
                }
            }

            let ret_v = self
                .builder
                .build_call(fv, casted_args.as_slice(), id)
                .try_as_basic_value()
                .left();

            if ret_t.base_type == BaseType::Void && ret_v.is_none()
                || ret_t.base_type != BaseType::Void && ret_v.is_some()
            {
                Ok((
                    ret_t.base_type,
                    ret_v.unwrap_or_else(|| {
                        self.context.i32_type().const_zero().as_basic_value_enum()
                    }),
                ))
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }

    pub(crate) fn gen_member_of_object(
        &self,
        obj: &Expression,
        member: &String,
        span: Span,
    ) -> Result<(BasicType, PointerValue<'ctx>), CE> {
        let (t, p_v) = self.get_lvalue(obj)?;
        if let BaseType::Struct(ref name, _) = t.base_type {
            let members = self
                .global_struct_map
                .get(name.clone().unwrap().as_str())
                .unwrap();
            let idx = members
                .iter()
                .map(|x| x.clone().member_name)
                .position(|x| x == *member);
            if idx.is_none() {
                Err(CE::struct_member_not_found(
                    name.clone().unwrap().to_string(),
                    member.to_string(),
                    span,
                ))
            } else {
                Ok((
                    members.get(idx.unwrap()).unwrap().member_type.clone(),
                    self.builder
                        .build_struct_gep(p_v, idx.unwrap() as u32, "member_of_object")
                        .unwrap(),
                ))
            }
        } else {
            unimplemented!()
        }
    }
}
