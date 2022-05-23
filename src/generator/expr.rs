use crate::ast::{
    AssignOperation, BaseType, BasicType, BinaryOperation, Expression, IntegerType, UnaryOperation,
};
use crate::generator::Generator;
use crate::utils::CompileErr;
use anyhow::Result;
use inkwell::values::{
    AnyValue, BasicMetadataValueEnum, BasicValue, BasicValueEnum, FloatValue, IntValue,
    PointerValue,
};
use inkwell::{FloatPredicate, IntPredicate};
use std::fmt::Error;
use std::ops::Deref;

impl<'ctx> Generator<'ctx> {
    pub(crate) fn gen_expression(
        &self,
        expr: &Expression,
    ) -> Result<(BaseType, BasicValueEnum<'ctx>)> {
        match expr {
            Expression::Empty => Ok((
                BaseType::Void,
                self.context
                    .i8_type()
                    .const_int(0 as u64, false)
                    .as_basic_value_enum(),
            )),
            Expression::Assignment(ref op, ref lhs, ref rhs) => self.gen_assignment(op, lhs, rhs),
            Expression::Unary(op, expr) => self.gen_unary_expr(op, expr),
            Expression::Binary(op, lhs, rhs) => self.gen_binary_expr(op, lhs, rhs),
            Expression::FunctionCall(ref name, ref args) => self.gen_function_call(name, args),
            Expression::CharacterConstant(ref value) => Ok((
                BaseType::SignedInteger(IntegerType::Char),
                self.context
                    .i8_type()
                    .const_int(*value as u64, false)
                    .as_basic_value_enum(),
            )),
            Expression::IntegerConstant(ref value) => Ok((
                BaseType::SignedInteger(IntegerType::Int),
                self.context
                    .i32_type()
                    .const_int(*value as u64, false)
                    .as_basic_value_enum(),
            )),
            Expression::UnsignedIntegerConstant(ref value) => Ok((
                BaseType::UnsignedInteger(IntegerType::Int),
                self.context
                    .i32_type()
                    .const_int(*value as u64, false)
                    .as_basic_value_enum(),
            )),
            Expression::LongConstant(ref value) | Expression::LongLongConstant(ref value) => Ok((
                BaseType::SignedInteger(IntegerType::Long),
                self.context
                    .i64_type()
                    .const_int(*value as u64, false)
                    .as_basic_value_enum(),
            )),
            Expression::UnsignedLongConstant(ref value)
            | Expression::UnsignedLongLongConstant(ref value) => Ok((
                BaseType::UnsignedInteger(IntegerType::Long),
                self.context
                    .i64_type()
                    .const_int(*value as u64, false)
                    .as_basic_value_enum(),
            )),
            Expression::FloatConstant(ref value) => Ok((
                BaseType::Float,
                self.context
                    .f32_type()
                    .const_float(*value as f64)
                    .as_basic_value_enum(),
            )),
            Expression::DoubleConstant(ref value) => Ok((
                BaseType::Float,
                self.context
                    .f64_type()
                    .const_float(*value as f64)
                    .as_basic_value_enum(),
            )),
            Expression::Identifier(ref string_literal) => {
                //if BaseType is Array, we just return Array type but don't load value!!
                let deref = self.get_variable(string_literal)?;
                let val = if let BaseType::Array(_, _) = deref.0.base_type {
                    deref.1.as_basic_value_enum()
                } else {
                    self.builder.build_load(deref.1, "load_val")
                };
                Ok((deref.0.base_type, val))
            }
            Expression::StringLiteral(ref string) => Ok((
                BaseType::Pointer(Box::new(BasicType {
                    qualifier: vec![],
                    base_type: BaseType::SignedInteger(IntegerType::Char),
                })),
                self.builder
                    .build_global_string_ptr(string.as_str(), "str")
                    .as_basic_value_enum(),
            )),
            Expression::ArraySubscript(ref id_expr, ref idx_vec) => {
                if let Expression::Identifier(id) = id_expr.deref() {
                    let (l_t, mut l_pv) = self.get_variable(id)?;

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
                    let (res_t, idx_int_val_vec) =
                        self.process_arr_subscript(l_t, idx_vec.clone())?;
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
                } else {
                    unreachable!()
                }
            }
            _ => return Err(CompileErr::UnknownExpression(expr.to_string()).into()),
        }
    }

    fn process_arr_subscript(
        &self,
        l_t: BasicType,
        idx_vec: Vec<Expression>,
    ) -> Result<(BasicType, Vec<IntValue<'ctx>>)> {
        if let BaseType::Array(ref arr_t, arr_len_vec) = l_t.base_type {
            let res_t: BaseType;
            if idx_vec.len() > arr_len_vec.len() {
                return Err(
                    CompileErr::ArrayDimensionNotMatch(arr_len_vec.len(), idx_vec.len()).into(),
                );
            } else if idx_vec.len() == arr_len_vec.len() {
                res_t = arr_t.base_type.clone();
            } else {
                res_t = BaseType::Array(
                    arr_t.clone(),
                    (idx_vec.len()..arr_len_vec.len()).fold(vec![], |mut acc, i| {
                        acc.push(arr_len_vec[i].clone());
                        acc
                    }),
                );
            }

            let mut idx_int_val_vec = vec![self.context.i32_type().const_zero()];
            idx_int_val_vec.extend(
                idx_vec
                    .iter()
                    .map(|expr| self.gen_expression(expr).unwrap().1.into_int_value()),
            );
            Ok((
                BasicType {
                    qualifier: l_t.qualifier,
                    base_type: res_t,
                },
                idx_int_val_vec,
            ))
        } else if let BaseType::Pointer(ref point_t) = l_t.base_type {
            if idx_vec.len() != 1 {
                return Err(CompileErr::PointDimensionNotMatch(1, idx_vec.len()).into());
            }
            // we don't support more than 1 dimension pointer
            if let BaseType::Pointer(_) = point_t.base_type {
                return Err(
                    CompileErr::Error("unsupported multidimensional pointer".to_string()).into(),
                );
            }
            //pointer doesn't need extra 0
            let mut idx_int_val_vec = vec![];
            idx_int_val_vec.extend(
                idx_vec
                    .iter()
                    .map(|expr| self.gen_expression(expr).unwrap().1.into_int_value()),
            );
            Ok((
                BasicType {
                    qualifier: l_t.qualifier,
                    base_type: point_t.base_type.clone(),
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
        expr: &Box<Expression>,
    ) -> Result<(BaseType, BasicValueEnum<'ctx>)> {
        let (expr_type, expr_value) = self.gen_expression(expr)?;

        match op {
            UnaryOperation::UnaryPlus => match expr_type {
                BaseType::Bool
                | BaseType::SignedInteger(_)
                | BaseType::UnsignedInteger(_)
                | BaseType::Float
                | BaseType::Double => Ok((expr_type, expr_value)),
                _ => return Err(CompileErr::InvalidUnary.into()),
            },
            UnaryOperation::UnaryMinus => match expr_type {
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
                _ => return Err(CompileErr::InvalidUnary.into()),
            },
            UnaryOperation::BitwiseNot => match expr_type {
                BaseType::SignedInteger(_) | BaseType::UnsignedInteger(_) => Ok((
                    expr_type,
                    self.builder
                        .build_not(expr_value.into_int_value(), "bitwise_not")
                        .as_basic_value_enum(),
                )),
                _ => return Err(CompileErr::InvalidUnary.into()),
            },
            UnaryOperation::LogicalNot => match expr_type {
                BaseType::SignedInteger(_) => {
                    let llvm_type = self.convert_llvm_type(&expr_type);
                    let result_int = self.builder.build_int_compare(
                        IntPredicate::EQ,
                        llvm_type.into_int_type().const_int(0 as u64, true),
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
                _ => return Err(CompileErr::InvalidUnary.into()),
            },
            UnaryOperation::Reference => {
                let (t, ptr) = self.get_lvalue(expr)?;
                Ok((BaseType::Pointer(Box::new(t)), ptr.as_basic_value_enum()))
            }
            UnaryOperation::Dereference => match expr_type {
                BaseType::Pointer(ref t) => Ok((
                    t.base_type.clone(),
                    self.builder
                        .build_load(expr_value.into_pointer_value(), "dereference")
                        .as_basic_value_enum(),
                )),
                _ => return Err(CompileErr::InvalidUnary.into()),
            },
            UnaryOperation::PostfixDecrement | UnaryOperation::PostfixIncrement => {
                match expr_type {
                    BaseType::SignedInteger(_)
                    | BaseType::UnsignedInteger(_)
                    | BaseType::Double
                    | BaseType::Float
                    | BaseType::Pointer(_) => {
                        let _ = self.gen_assignment(
                            if *op == UnaryOperation::PostfixIncrement {
                                &AssignOperation::Addition
                            } else {
                                &AssignOperation::Subtraction
                            },
                            expr,
                            &Box::new(Expression::IntegerConstant(1)),
                        )?;
                        Ok((expr_type, expr_value))
                    }
                    _ => return Err(CompileErr::InvalidUnary.into()),
                }
            }
            UnaryOperation::PrefixIncrement | UnaryOperation::PrefixDecrement => match expr_type {
                BaseType::SignedInteger(_)
                | BaseType::UnsignedInteger(_)
                | BaseType::Double
                | BaseType::Float
                | BaseType::Pointer(_) => self.gen_assignment(
                    if *op == UnaryOperation::PrefixIncrement {
                        &AssignOperation::Addition
                    } else {
                        &AssignOperation::Subtraction
                    },
                    expr,
                    &Box::new(Expression::IntegerConstant(1)),
                ),
                _ => return Err(CompileErr::InvalidUnary.into()),
            },
            _ => return Err(CompileErr::InvalidUnary.into()),
        }
    }

    fn gen_binary_expr(
        &self,
        op: &BinaryOperation,
        lhs: &Box<Expression>,
        rhs: &Box<Expression>,
    ) -> Result<(BaseType, BasicValueEnum<'ctx>)> {
        let (l_t, l_v) = self.gen_expression(lhs)?;
        let (r_t, r_v) = self.gen_expression(rhs)?;

        //TODO FIXME!! so dirty!
        let mut visit = 0;
        let tmp = match l_t {
            BaseType::SignedInteger(_) | BaseType::UnsignedInteger(_) => {
                if let BaseType::Pointer(_) = r_t {
                    visit = 1;
                    self.build_point_binary_op(op, r_v.into_pointer_value(), l_v.into_int_value())
                } else {
                    Err(CompileErr::Error("".to_string()).into())
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
                _ => Err(CompileErr::Error("".to_string()).into()),
            },
            _ => Err(CompileErr::Error("".to_string()).into()),
        };
        if visit > 0 {
            return match tmp {
                Ok(t) => {
                    if visit == 1 {
                        Ok((r_t, t))
                    } else {
                        Ok((l_t, t))
                    }
                }
                Err(e) => Err(e),
            };
        }

        let cast_t = BaseType::upcast(&l_t, &r_t)?;
        let l_cast_v = self.cast_value(&l_t, &l_v, &cast_t)?;
        let r_cast_v = self.cast_value(&r_t, &r_v, &cast_t)?;

        match cast_t {
            BaseType::Void => {
                Err(CompileErr::Error("Invalid void type for binary operation".to_string()).into())
            }
            BaseType::SignedInteger(_) | BaseType::UnsignedInteger(_) | BaseType::Bool => {
                let int_or_bool_value = self.build_int_binary_op(
                    op,
                    l_cast_v.into_int_value(),
                    r_cast_v.into_int_value(),
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
    ) -> Result<BasicValueEnum<'ctx>> {
        let result_v = match op {
            BinaryOperation::Addition => unsafe {
                let idx_int_val_vec = vec![rhs];
                self.builder
                    .build_gep(lhs, idx_int_val_vec.as_ref(), "pointer add")
            },
            BinaryOperation::Subtraction => unsafe {
                let idx_int_val_vec = vec![rhs.const_neg()];
                self.builder
                    .build_gep(lhs, idx_int_val_vec.as_ref(), "pointer add")
            },

            // logical
            //TODO 先不管logical操作了QAQ
            _ => return Err(CompileErr::InvalidBinary.into()),
        };
        Ok(result_v.as_basic_value_enum())
    }
    fn build_int_binary_op(
        &self,
        op: &BinaryOperation,
        lhs: IntValue<'ctx>,
        rhs: IntValue<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>> {
        let result_v = match op {
            // arithmetic
            BinaryOperation::Addition => self.builder.build_int_add(lhs, rhs, "int_add"),
            BinaryOperation::Subtraction => self.builder.build_int_sub(lhs, rhs, "int_sub"),
            BinaryOperation::Multiplication => self.builder.build_int_mul(lhs, rhs, "int_mul"),
            BinaryOperation::Division => self.builder.build_int_signed_div(lhs, rhs, "int_div"),
            BinaryOperation::Modulo => self.builder.build_int_signed_rem(lhs, rhs, "int_mod"),
            BinaryOperation::BitwiseAnd => self.builder.build_and(lhs, rhs, "int_and"),
            BinaryOperation::BitwiseOr => self.builder.build_or(lhs, rhs, "int_or"),
            BinaryOperation::BitwiseXor => self.builder.build_xor(lhs, rhs, "int_xor"),
            BinaryOperation::LeftShift => self.builder.build_left_shift(lhs, rhs, "int_shl"),
            BinaryOperation::RightShift => {
                self.builder.build_right_shift(lhs, rhs, true, "int_shr")
            }
            // comparison
            BinaryOperation::LessThan => {
                self.builder
                    .build_int_compare(IntPredicate::SLT, lhs, rhs, "int_lt")
            }
            BinaryOperation::LessThanOrEqual => {
                self.builder
                    .build_int_compare(IntPredicate::SLE, lhs, rhs, "int_le")
            }
            BinaryOperation::GreaterThan => {
                self.builder
                    .build_int_compare(IntPredicate::SGT, lhs, rhs, "int_gt")
            }
            BinaryOperation::GreaterThanOrEqual => {
                self.builder
                    .build_int_compare(IntPredicate::SGE, lhs, rhs, "int_ge")
            }
            BinaryOperation::Equal => {
                self.builder
                    .build_int_compare(IntPredicate::EQ, lhs, rhs, "int_eq")
            }
            BinaryOperation::NotEqual => {
                self.builder
                    .build_int_compare(IntPredicate::NE, lhs, rhs, "int_ne")
            }
            // logical
            //TODO FIXME 这里and 和 or 是完全错误的，这个build_int_cast只是截取了最后一位！！
            BinaryOperation::LogicalAnd => self.builder.build_and(
                self.builder
                    .build_int_cast(lhs, self.context.bool_type(), "cast i32 to i1_1"),
                self.builder
                    .build_int_cast(rhs, self.context.bool_type(), "cast i32 to i1_2"),
                "logical int and",
            ),
            BinaryOperation::LogicalOr => self.builder.build_or(
                self.builder
                    .build_int_cast(lhs, self.context.bool_type(), "cast i32 to i1_1"),
                self.builder
                    .build_int_cast(rhs, self.context.bool_type(), "cast i32 to i1_2"),
                "logical int or",
            ),

            _ => return Err(CompileErr::InvalidBinary.into()),
        };

        Ok(result_v.as_basic_value_enum())
    }
    fn build_float_binary_op(
        &self,
        op: &BinaryOperation,
        lhs: FloatValue<'ctx>,
        rhs: FloatValue<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>> {
        let result_f = match op {
            // arithmetic
            BinaryOperation::Addition => Ok(self.builder.build_float_add(lhs, rhs, "float add")),
            BinaryOperation::Subtraction => Ok(self.builder.build_float_sub(lhs, rhs, "float sub")),
            BinaryOperation::Multiplication => {
                Ok(self.builder.build_float_mul(lhs, rhs, "float mul"))
            }
            BinaryOperation::Division => Ok(self.builder.build_float_div(lhs, rhs, "float div")),
            BinaryOperation::Modulo => Ok(self.builder.build_float_rem(lhs, rhs, "float mod")),
            _ => Err(Error),
        };
        //return FloatValue
        if result_f.is_ok() {
            return Ok(result_f.unwrap().as_basic_value_enum());
        }
        let result_i = match op {
            BinaryOperation::LessThan => {
                self.builder
                    .build_float_compare(FloatPredicate::OLT, lhs, rhs, "float lt")
            }
            BinaryOperation::LessThanOrEqual => {
                self.builder
                    .build_float_compare(FloatPredicate::OLE, lhs, rhs, "float le")
            }
            BinaryOperation::GreaterThan => {
                self.builder
                    .build_float_compare(FloatPredicate::OGT, lhs, rhs, "float gt")
            }
            BinaryOperation::GreaterThanOrEqual => {
                self.builder
                    .build_float_compare(FloatPredicate::OGE, lhs, rhs, "float ge")
            }
            BinaryOperation::Equal => {
                self.builder
                    .build_float_compare(FloatPredicate::OEQ, lhs, rhs, "float eq")
            }
            BinaryOperation::NotEqual => {
                self.builder
                    .build_float_compare(FloatPredicate::ONE, lhs, rhs, "float ne")
            }
            // logical
            //TODO FIXME 这里and 和 or 是完全错误的，这个build_int_cast只是截取了最后一位！！
            BinaryOperation::LogicalAnd => self.builder.build_and(
                self.builder.build_int_cast(
                    self.builder
                        .build_cast(
                            self.gen_cast_llvm_instruction(
                                &BaseType::Double,
                                &BaseType::SignedInteger(IntegerType::Int),
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
            BinaryOperation::LogicalOr => self.builder.build_or(
                self.builder.build_int_cast(
                    self.builder
                        .build_cast(
                            self.gen_cast_llvm_instruction(
                                &BaseType::Double,
                                &BaseType::SignedInteger(IntegerType::Int),
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
            _ => return Err(CompileErr::InvalidBinary.into()),
        };
        //return IntValue
        Ok(result_i.as_basic_value_enum())
    }
    pub(crate) fn gen_assignment(
        &self,
        op: &AssignOperation,
        lhs: &Box<Expression>,
        rhs: &Box<Expression>,
    ) -> Result<(BaseType, BasicValueEnum<'ctx>)> {
        let (l_t, l_pv) = self.get_lvalue(lhs)?;

        let (r_t, r_v) = if let AssignOperation::Naive = op {
            self.gen_expression(rhs)?
        } else {
            self.gen_binary_expr(
                &match op {
                    AssignOperation::Addition => BinaryOperation::Addition,
                    AssignOperation::Subtraction => BinaryOperation::Subtraction,
                    AssignOperation::Multiplication => BinaryOperation::Multiplication,
                    AssignOperation::Division => BinaryOperation::Division,
                    AssignOperation::Modulo => BinaryOperation::Modulo,
                    AssignOperation::BitwiseAnd => BinaryOperation::BitwiseAnd,
                    AssignOperation::BitwiseOr => BinaryOperation::BitwiseOr,
                    AssignOperation::BitwiseXor => BinaryOperation::BitwiseXor,
                    AssignOperation::LeftShift => BinaryOperation::LeftShift,
                    AssignOperation::RightShift => BinaryOperation::RightShift,
                    AssignOperation::Naive => unreachable!(),
                },
                lhs,
                rhs,
            )?
        };

        r_t.test_cast(&l_t.base_type)?;

        let cast_v = self.cast_value(&r_t, &r_v, &l_t.base_type)?;

        self.builder.build_store(l_pv, cast_v);

        Ok((l_t.base_type, cast_v))
    }

    fn get_lvalue(&self, lhs: &Box<Expression>) -> Result<(BasicType, PointerValue<'ctx>)> {
        match lhs.deref().deref() {
            Expression::Identifier(ref id) => Ok(self.get_variable(id)?),
            Expression::Unary(ref op, ref unary_operation) => {
                // we need lhs expression's type!!! But now we don't have a function only get the type
                //TODO FIXME! It's really dirty
                let (t, _) = self.gen_expression(lhs)?;
                let (l_t, l_v) = self.gen_expression(unary_operation)?;
                if let UnaryOperation::Dereference = op {
                    if let BaseType::Pointer(_) = l_t {
                        Ok((
                            BasicType {
                                qualifier: vec![],
                                base_type: t,
                            },
                            l_v.into_pointer_value(),
                        ))
                    } else {
                        Err(
                            CompileErr::InvalidDereference(l_v.print_to_string().to_string())
                                .into(),
                        )
                    }
                } else {
                    if l_v.is_pointer_value() {
                        Ok((
                            BasicType {
                                qualifier: vec![],
                                base_type: t,
                            },
                            l_v.into_pointer_value(),
                        ))
                    } else {
                        Err(CompileErr::InvalidLeftValue(l_v.print_to_string().to_string()).into())
                    }
                }
            }
            Expression::ArraySubscript(ref id_expr, ref idx_vec) => {
                if let Expression::Identifier(id) = id_expr.deref() {
                    let (t, mut pv) = self.get_variable(id)?;
                    //if type is pointer, get_variable will get the address of pointer, not the pointer point to!
                    //So if we want get the point to address, we need extra load action!
                    if let BaseType::Pointer(_) = t.base_type {
                        pv = self
                            .builder
                            .build_load(pv, "dereference")
                            .into_pointer_value()
                    }
                    let (res_t, idx_int_val_vec) =
                        self.process_arr_subscript(t, idx_vec.clone())?;
                    Ok((res_t, unsafe {
                        self.builder
                            .build_gep(pv, idx_int_val_vec.as_ref(), "arr_subscript")
                    }))
                } else {
                    unreachable!()
                }
            }
            _ => panic!(),
        }
    }

    fn gen_function_call(
        &self,
        name: &Box<Expression>,
        args: &Vec<Expression>,
    ) -> Result<(BaseType, BasicValueEnum<'ctx>)> {
        if let Expression::Identifier(ref id) = name.deref().deref() {
            let (ret_t, args_t, is_variadic) = self.function_map.get(id).unwrap().to_owned();
            let fv = self.module.get_function(id).unwrap();

            if args.len() != fv.get_type().count_param_types() as usize {
                if !(is_variadic && args.len() > fv.get_type().count_param_types() as usize) {
                    return Err(CompileErr::ParameterCountMismatch(
                        id.to_string(),
                        fv.get_type().count_param_types() as usize,
                        args.len(),
                    )
                    .into());
                }
            }

            let mut casted_args = Vec::with_capacity(args.len());

            for (i, e) in args.iter().enumerate() {
                let t = args_t.get(i);

                match t {
                    Some(t) => {
                        let (e_t, e_v) = self.gen_expression(e)?;

                        e_t.test_cast(&t.base_type)?;
                        let cast_v = self.cast_value(&e_t, &e_v, &t.base_type)?;

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
                    ret_v.unwrap_or(self.context.i32_type().const_zero().as_basic_value_enum()),
                ))
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }
}
