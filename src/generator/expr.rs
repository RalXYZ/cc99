use std::ops::Deref;
use inkwell::values::{BasicValue, BasicValueEnum};
use anyhow::Result;
use inkwell::IntPredicate;
use crate::ast::{BaseType, BasicType, Expression, IntegerType, UnaryOperation};
use crate::generator::Generator;
use crate::utils::CompileErr;

impl<'ctx> Generator<'ctx> {
    pub(crate) fn gen_expression(&mut self, expr: &Expression) -> Result<(BaseType, BasicValueEnum<'ctx>)> {
        match expr {
            Expression::Unary(op, expr) =>
                self.gen_unary_expr(op, expr),
            Expression::CharacterConstant(ref value) =>
                Ok((
                    BaseType::SignedInteger(IntegerType::Char),
                    self.context.i8_type().const_int(*value as u64, false).as_basic_value_enum(),
                )),
            Expression::IntegerConstant(ref value) =>
                Ok((
                    BaseType::SignedInteger(IntegerType::Int),
                    self.context.i32_type().const_int(*value as u64, false).as_basic_value_enum(),
                )),
            Expression::UnsignedIntegerConstant(ref value) =>
                Ok((
                    BaseType::UnsignedInteger(IntegerType::Int),
                    self.context.i32_type().const_int(*value as u64, false).as_basic_value_enum(),
                )),
            Expression::LongConstant(ref value) |
            Expression::LongLongConstant(ref value) =>
                Ok((
                    BaseType::SignedInteger(IntegerType::Long),
                    self.context.i64_type().const_int(*value as u64, false).as_basic_value_enum(),
                )),
            Expression::UnsignedLongConstant(ref value) |
            Expression::UnsignedLongLongConstant(ref value) =>
                Ok((
                    BaseType::UnsignedInteger(IntegerType::Long),
                    self.context.i64_type().const_int(*value as u64, false).as_basic_value_enum(),
                )),
            Expression::FloatConstant(ref value) =>
                Ok((
                    BaseType::Float,
                    self.context.f32_type().const_float(*value as f64).as_basic_value_enum(),
                )),
            Expression::DoubleConstant(ref value) =>
                Ok((
                    BaseType::Float,
                    self.context.f64_type().const_float(*value as f64).as_basic_value_enum(),
                )),
            Expression::Identifier(ref string_literal) => {
                let deref = self.get_variable(string_literal)?;
                let val = self.builder.build_load(deref.1, "load val");
                Ok((deref.0.base_type, val))
            },
            Expression::StringLiteral(ref string) => {
                // let i32_type = self.context.i32_type();
                // let i32_ptr_type = i32_type.ptr_type(AddressSpace::Generic);
                // let fn_type = i32_type.fn_type(&[i32_ptr_type.into()], false);
                // let fn_value = self.module.add_function("ret", fn_type, None);
                // let entry = self.context.append_basic_block(fn_value, "entry");
                // self.builder.position_at_end(entry);
                // self.builder.build_return(None);

                Ok((
                    BaseType::Pointer(Box::new(BasicType{
                        qualifier: vec![],
                        base_type: BaseType::SignedInteger(IntegerType::Char),
                    })),
                    self.builder
                        .build_global_string_ptr(string.as_str(), "str")
                        .as_basic_value_enum(),
                ))
            },
            _ => return Err(CompileErr::UnknownExpression(expr.to_string()).into()),
        }
    }

    fn gen_unary_expr(
        &mut self, op: &UnaryOperation, expr: &Box<Expression>
    ) -> Result<(BaseType, BasicValueEnum<'ctx>)> {
        let (expr_type, expr_value) = self.gen_expression(expr)?;

        match op {
            UnaryOperation::UnaryPlus =>
                match expr_type {
                    BaseType::Bool |
                    BaseType::SignedInteger(_) |
                    BaseType::UnsignedInteger(_) |
                    BaseType::Float |
                    BaseType::Double =>
                        Ok((expr_type, expr_value)),
                    _ => return Err(CompileErr::InvalidUnary.into()),
                },
            UnaryOperation::UnaryMinus => {
                match expr_type {
                    BaseType::Bool |
                    BaseType::SignedInteger(_) |
                    BaseType::UnsignedInteger(_) =>
                        Ok((
                            expr_type,
                            self.builder.build_int_neg(
                                expr_value.into_int_value(), "int neg"
                            ).as_basic_value_enum()
                        )),
                    BaseType::Float |
                    BaseType::Double =>
                        Ok((
                            expr_type,
                            self.builder.build_float_neg(
                                expr_value.into_float_value(), "float neg"
                            ).as_basic_value_enum()
                        )),
                    _ => return Err(CompileErr::InvalidUnary.into()),
                }
            },
            UnaryOperation::BitwiseNot => {
                match expr_type {
                    BaseType::SignedInteger(_) |
                    BaseType::UnsignedInteger(_) =>
                        Ok((
                            expr_type,
                            self.builder.build_not(
                                expr_value.into_int_value(), "bitwise not"
                            ).as_basic_value_enum()
                        )),
                    _ => return Err(CompileErr::InvalidUnary.into()),
                }
            },
            UnaryOperation::LogicalNot => {
                match expr_type {
                    BaseType::SignedInteger(_) => {
                        let llvm_type = self.convert_llvm_type(&expr_type);
                        let result_int = self.builder.build_int_compare(
                            IntPredicate::EQ,
                            llvm_type.into_int_type().const_int(0 as u64, true),
                            expr_value.into_int_value(),
                            "logical not result int",
                        );

                        let llvm_type = self.convert_llvm_type(&BaseType::Bool);
                        Ok((
                            BaseType::Bool, self.builder.build_int_cast(
                                result_int,
                                llvm_type.into_int_type(),
                                "cast logical not result to bool",
                            ).as_basic_value_enum()
                        ))
                    },
                    _ => return Err(CompileErr::InvalidUnary.into()),
                }
            },
            UnaryOperation::Reference => {
                match expr.deref().deref() {
                    Expression::Identifier(ref id) => {
                        let (t, ptr) = self.get_variable(id)?;
                        Ok((
                            BaseType::Pointer(Box::new(t)),
                            ptr.as_basic_value_enum(),
                        ))
                    },
                    _ => return Err(CompileErr::InvalidUnary.into()),

                }
            },
            UnaryOperation::Dereference => {
                match expr_type {
                    BaseType::Pointer(ref t) => {
                        Ok((
                            t.base_type.clone(),
                            self.builder.build_load(
                                expr_value.into_pointer_value(),
                                "dereference",
                            ).as_basic_value_enum(),
                        ))
                    },
                    _ => return Err(CompileErr::InvalidUnary.into()),
                }
            },
            _ => unimplemented!()
        }

    }
}
