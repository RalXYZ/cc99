use inkwell::values::{BasicValue, BasicValueEnum};
use anyhow::Result;
use crate::{BaseType, Expression, IntegerType};
use crate::generator::Generator;
use crate::utils::CompileErr;

impl<'ctx> Generator<'ctx> {
    pub(crate) fn gen_expression(&self, expr: &Expression) -> Result<(BaseType, BasicValueEnum<'ctx>)> {
        match expr {
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
                    BaseType::Pointer(Box::new(BT{
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
}
