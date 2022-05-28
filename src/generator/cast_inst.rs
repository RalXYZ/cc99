use crate::ast::{BaseType as Ty, IntegerType as IT, Span};
use crate::generator::Generator;
use crate::utils::CompileErr as CE;
use inkwell::values::InstructionOpcode as Op;

impl<'ctx> Generator<'ctx> {
    pub fn gen_cast_llvm_instruction(&self, curr: &Ty, dest: &Ty, span: Span) -> Result<Op, CE> {
        let instruction = match curr {
            // char
            Ty::UnsignedInteger(IT::Char) => match dest {
                Ty::SignedInteger(_) => Op::SExt,
                Ty::UnsignedInteger(_) => Op::ZExt,
                Ty::Float | Ty::Double => Op::SIToFP,
                Ty::Pointer(_) => Op::IntToPtr,
                _ => return Err(CE::invalid_cast(curr.to_string(), dest.to_string(), span).into()),
            },
            Ty::SignedInteger(IT::Char) => match dest {
                Ty::SignedInteger(IT::Char) => Op::BitCast,
                Ty::SignedInteger(_) | Ty::UnsignedInteger(_) => Op::ZExt,
                Ty::Float | Ty::Double => Op::UIToFP,
                Ty::Pointer(_) => Op::IntToPtr,
                _ => return Err(CE::invalid_cast(curr.to_string(), dest.to_string(), span).into()),
            },

            // short
            Ty::SignedInteger(IT::Short) => match dest {
                Ty::SignedInteger(IT::Char) | Ty::UnsignedInteger(IT::Char) => Op::Trunc,
                Ty::UnsignedInteger(IT::Short) => Op::BitCast,
                Ty::SignedInteger(_) => Op::SExt,
                Ty::UnsignedInteger(_) => Op::ZExt,
                Ty::Float | Ty::Double => Op::SIToFP,
                Ty::Pointer(_) => Op::IntToPtr,
                _ => return Err(CE::invalid_cast(curr.to_string(), dest.to_string(), span).into()),
            },
            Ty::UnsignedInteger(IT::Short) => match dest {
                Ty::SignedInteger(IT::Char) | Ty::UnsignedInteger(IT::Char) => Op::Trunc,
                Ty::SignedInteger(IT::Short) => Op::BitCast,
                Ty::SignedInteger(_) | Ty::UnsignedInteger(_) => Op::ZExt,
                Ty::Float | Ty::Double => Op::UIToFP,
                Ty::Pointer(_) => Op::IntToPtr,
                _ => return Err(CE::invalid_cast(curr.to_string(), dest.to_string(), span).into()),
            },

            // int
            Ty::SignedInteger(IT::Int) => match dest {
                Ty::SignedInteger(IT::Char)
                | Ty::UnsignedInteger(IT::Char)
                | Ty::SignedInteger(IT::Short)
                | Ty::UnsignedInteger(IT::Short) => Op::Trunc,
                Ty::UnsignedInteger(IT::Int) => Op::BitCast,
                Ty::SignedInteger(_) => Op::SExt,
                Ty::UnsignedInteger(_) => Op::ZExt,
                Ty::Float | Ty::Double => Op::SIToFP,
                Ty::Pointer(_) => Op::IntToPtr,
                _ => return Err(CE::invalid_cast(curr.to_string(), dest.to_string(), span).into()),
            },
            Ty::UnsignedInteger(IT::Int) => match dest {
                Ty::SignedInteger(IT::Char)
                | Ty::UnsignedInteger(IT::Char)
                | Ty::SignedInteger(IT::Short)
                | Ty::UnsignedInteger(IT::Short) => Op::Trunc,
                Ty::SignedInteger(IT::Int) => Op::BitCast,
                Ty::SignedInteger(_) | Ty::UnsignedInteger(_) => Op::ZExt,
                Ty::Float | Ty::Double => Op::UIToFP,
                Ty::Pointer(_) => Op::IntToPtr,
                _ => return Err(CE::invalid_cast(curr.to_string(), dest.to_string(), span).into()),
            },

            // long
            Ty::SignedInteger(IT::Long) => match dest {
                Ty::SignedInteger(IT::Char)
                | Ty::UnsignedInteger(IT::Char)
                | Ty::SignedInteger(IT::Short)
                | Ty::UnsignedInteger(IT::Short)
                | Ty::SignedInteger(IT::Int)
                | Ty::UnsignedInteger(IT::Int) => Op::Trunc,
                Ty::UnsignedInteger(IT::Long) => Op::BitCast,
                Ty::SignedInteger(_) => Op::SExt,
                Ty::UnsignedInteger(_) => Op::ZExt,
                Ty::Float | Ty::Double => Op::SIToFP,
                Ty::Pointer(_) => Op::IntToPtr,
                _ => return Err(CE::invalid_cast(curr.to_string(), dest.to_string(), span).into()),
            },
            Ty::UnsignedInteger(IT::Long) => match dest {
                Ty::SignedInteger(IT::Char)
                | Ty::UnsignedInteger(IT::Char)
                | Ty::SignedInteger(IT::Short)
                | Ty::UnsignedInteger(IT::Short)
                | Ty::SignedInteger(IT::Int)
                | Ty::UnsignedInteger(IT::Int) => Op::Trunc,
                Ty::SignedInteger(IT::Long) => Op::BitCast,
                Ty::SignedInteger(_) | Ty::UnsignedInteger(_) => Op::ZExt,
                Ty::Float | Ty::Double => Op::UIToFP,
                Ty::Pointer(_) => Op::IntToPtr,
                _ => return Err(CE::invalid_cast(curr.to_string(), dest.to_string(), span).into()),
            },

            // long long
            Ty::SignedInteger(IT::LongLong) => match dest {
                Ty::SignedInteger(IT::Char)
                | Ty::UnsignedInteger(IT::Char)
                | Ty::SignedInteger(IT::Short)
                | Ty::UnsignedInteger(IT::Short)
                | Ty::SignedInteger(IT::Int)
                | Ty::UnsignedInteger(IT::Int)
                | Ty::SignedInteger(IT::Long)
                | Ty::UnsignedInteger(IT::Long) => Op::Trunc,
                Ty::UnsignedInteger(IT::LongLong) => Op::BitCast,
                Ty::Float | Ty::Double => Op::SIToFP,
                Ty::Pointer(_) => Op::IntToPtr,
                _ => return Err(CE::invalid_cast(curr.to_string(), dest.to_string(), span).into()),
            },
            Ty::UnsignedInteger(IT::LongLong) => match dest {
                Ty::SignedInteger(IT::Char)
                | Ty::UnsignedInteger(IT::Char)
                | Ty::SignedInteger(IT::Short)
                | Ty::UnsignedInteger(IT::Short)
                | Ty::SignedInteger(IT::Int)
                | Ty::UnsignedInteger(IT::Int)
                | Ty::SignedInteger(IT::Long)
                | Ty::UnsignedInteger(IT::Long) => Op::Trunc,
                Ty::SignedInteger(IT::LongLong) => Op::BitCast,
                Ty::Float | Ty::Double => Op::UIToFP,
                Ty::Pointer(_) => Op::IntToPtr,
                _ => return Err(CE::invalid_cast(curr.to_string(), dest.to_string(), span).into()),
            },

            Ty::Bool => match dest {
                Ty::SignedInteger(_) | Ty::UnsignedInteger(_) => Op::ZExt,
                Ty::Float | Ty::Double => Op::UIToFP,
                _ => return Err(CE::invalid_cast(curr.to_string(), dest.to_string(), span).into()),
            },

            Ty::Float => match dest {
                Ty::SignedInteger(_) => Op::FPToSI,
                Ty::UnsignedInteger(_) => Op::FPToUI,
                Ty::Double => Op::FPExt,
                _ => return Err(CE::invalid_cast(curr.to_string(), dest.to_string(), span).into()),
            },
            Ty::Double => match dest {
                Ty::SignedInteger(_) => Op::FPToSI,
                Ty::UnsignedInteger(_) => Op::FPToUI,
                Ty::Float => Op::FPTrunc,
                _ => return Err(CE::invalid_cast(curr.to_string(), dest.to_string(), span).into()),
            },

            Ty::Pointer(_) => match dest {
                Ty::SignedInteger(_) | Ty::UnsignedInteger(_) => Op::PtrToInt,
                Ty::Pointer(_) => Op::BitCast,
                _ => return Err(CE::invalid_cast(curr.to_string(), dest.to_string(), span).into()),
            },
            Ty::Array(_, _) => match dest {
                Ty::Pointer(_) => Op::BitCast,
                _ => return Err(CE::invalid_cast(curr.to_string(), dest.to_string(), span).into()),
            },
            _ => return Err(CE::invalid_cast(curr.to_string(), dest.to_string(), span).into()),
        };
        Ok(instruction)
    }
}
