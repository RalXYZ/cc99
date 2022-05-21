use crate::ast::{
    AssignOperation, BaseType, BasicType, BinaryOperation, Expression, IntegerType, UnaryOperation,
};
use crate::generator::Generator;
use crate::utils::CompileErr;
use anyhow::Result;
use inkwell::types::AnyType;
use inkwell::values::{BasicMetadataValueEnum, BasicValue, BasicValueEnum, IntValue, PointerValue};
use inkwell::{AddressSpace, IntPredicate};
use std::borrow::Borrow;
use std::ops::Deref;

impl<'ctx> Generator<'ctx> {
    pub(crate) fn gen_expression(
        &mut self,
        expr: &Expression,
    ) -> Result<(BaseType, BasicValueEnum<'ctx>)> {
        match expr {
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
                let deref = self.get_variable(string_literal)?;
                let val = self.builder.build_load(deref.1, "load_val");
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
            Expression::ArraySubscript(ref id_expr, ref idx) => {
                if let Expression::Identifier(id) = id_expr.deref() {
                    let (l_t, l_pv) = self.get_variable(id)?;
                    if let BaseType::Array(arr_t, arr_l) = l_t.base_type {
                        let idx = self.gen_expression(idx).unwrap().1.into_int_value();
                        Ok((
                            arr_t.base_type,
                            self.builder.build_load(
                                unsafe {
                                    self.builder.build_gep(
                                        l_pv,
                                        vec![self.context.i32_type().const_zero(), idx].as_ref(),
                                        "arr_subscript",
                                    )
                                },
                                "load_arr_subscript",
                            ),
                        ))
                    } else {
                        unreachable!()
                    }
                } else {
                    unreachable!()
                }
            }
            _ => return Err(CompileErr::UnknownExpression(expr.to_string()).into()),
        }
    }

    fn gen_unary_expr(
        &mut self,
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
            UnaryOperation::Reference => match expr.deref().deref() {
                Expression::Identifier(ref id) => {
                    let (t, ptr) = self.get_variable(id)?;
                    Ok((BaseType::Pointer(Box::new(t)), ptr.as_basic_value_enum()))
                }
                _ => return Err(CompileErr::InvalidUnary.into()),
            },
            UnaryOperation::Dereference => match expr_type {
                BaseType::Pointer(ref t) => Ok((
                    t.base_type.clone(),
                    self.builder
                        .build_load(expr_value.into_pointer_value(), "dereference")
                        .as_basic_value_enum(),
                )),
                _ => return Err(CompileErr::InvalidUnary.into()),
            },
            _ => return Err(CompileErr::InvalidUnary.into()),
        }
    }

    fn gen_binary_expr(
        &mut self,
        op: &BinaryOperation,
        lhs: &Box<Expression>,
        rhs: &Box<Expression>,
    ) -> Result<(BaseType, BasicValueEnum<'ctx>)> {
        let (l_t, l_v) = self.gen_expression(lhs)?;
        let (r_t, r_v) = self.gen_expression(rhs)?;

        let cast_t = BaseType::upcast(&l_t, &r_t)?;
        let l_cast_v = self.cast_value(&l_t, &l_v, &cast_t)?;
        let r_cast_v = self.cast_value(&r_t, &r_v, &cast_t)?;

        match cast_t {
            BaseType::SignedInteger(_) | BaseType::UnsignedInteger(_) => Ok((
                cast_t,
                self.build_int_binary_op(op, l_cast_v.into_int_value(), r_cast_v.into_int_value())?,
            )),
            _ => unimplemented!(),
        }
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
            BinaryOperation::LogicalAnd => self.builder.build_and(
                self.builder
                    .build_int_cast(lhs, self.context.bool_type(), "cast_i32_to_i1"),
                self.builder
                    .build_int_cast(rhs, self.context.bool_type(), "cast_i32_to_i1"),
                "logical_int_and",
            ),
            BinaryOperation::LogicalOr => self.builder.build_or(
                self.builder
                    .build_int_cast(lhs, self.context.bool_type(), "cast_i32_to_i1"),
                self.builder
                    .build_int_cast(rhs, self.context.bool_type(), "cast_i32_to_i1"),
                "logical_int_or",
            ),

            _ => return Err(CompileErr::InvalidBinary.into()),
        };

        Ok(result_v.as_basic_value_enum())
    }

    fn gen_assignment(
        &mut self,
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

    fn get_lvalue(&mut self, lhs: &Box<Expression>) -> Result<(BasicType, PointerValue<'ctx>)> {
        match lhs.deref().deref() {
            Expression::Identifier(ref id) => Ok(self.get_variable(id)?),
            Expression::ArraySubscript(ref id_expr, ref idx) => {
                if let Expression::Identifier(id) = id_expr.deref() {
                    let (t, pv) = self.get_variable(id)?;

                    if let BaseType::Array(arr_t, _) = t.base_type {
                        let idx = self.gen_expression(idx).unwrap().1.into_int_value();
                        Ok((*arr_t, unsafe {
                            self.builder.build_gep(
                                pv,
                                vec![self.context.i32_type().const_zero(), idx].as_ref(),
                                "arr_subscript",
                            )
                        }))
                    } else {
                        unreachable!()
                    }
                } else {
                    unreachable!()
                }
            }
            _ => panic!(),
        }
    }

    fn gen_function_call(
        &mut self,
        name: &Box<Expression>,
        args: &Vec<Expression>,
    ) -> Result<(BaseType, BasicValueEnum<'ctx>)> {
        if let Expression::Identifier(ref id) = name.deref().deref() {
            let (ret_t, args_t) = self.function_map.get(id).unwrap().to_owned();
            let fv = self.module.get_function(id).unwrap();

            if args.len() != fv.get_type().count_param_types() as usize {
                return Err(CompileErr::ParameterCountMismatch(
                    id.to_string(),
                    fv.get_type().count_param_types() as usize,
                    args.len(),
                )
                .into());
            }

            let mut casted_args = Vec::with_capacity(args.len());

            for (i, e) in args.iter().enumerate() {
                let t = args_t.get(i).unwrap();

                let (e_t, e_v) = self.gen_expression(e)?;

                e_t.test_cast(&t.base_type)?;
                let cast_v = self.cast_value(&e_t, &e_v, &t.base_type)?;

                casted_args.push(BasicMetadataValueEnum::from(cast_v));
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
