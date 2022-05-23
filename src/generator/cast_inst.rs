use crate::ast::{BaseType as Ty, IntegerType as IT};
use crate::generator::Generator;
use crate::utils::CompileErr::InvalidCast;
use anyhow::Result;
use inkwell::values::InstructionOpcode as Op;

impl<'ctx> Generator<'ctx> {
    pub fn gen_cast_llvm_instruction(&self, curr: &Ty, dest: &Ty) -> Result<Op> {
        let instruction = match curr {
            // char
            Ty::UnsignedInteger(IT::Char) => match dest {
                Ty::SignedInteger(_) => Op::SExt,
                Ty::UnsignedInteger(_) => Op::ZExt,
                Ty::Float | Ty::Double => Op::SIToFP,
                Ty::Pointer(_) => Op::IntToPtr,
                _ => return Err(InvalidCast(curr.clone(), dest.clone()).into()),
            },
            Ty::SignedInteger(IT::Char) => match dest {
                Ty::SignedInteger(IT::Char) => Op::BitCast,
                Ty::SignedInteger(_) | Ty::UnsignedInteger(_) => Op::ZExt,
                Ty::Float | Ty::Double => Op::UIToFP,
                Ty::Pointer(_) => Op::IntToPtr,
                _ => return Err(InvalidCast(curr.clone(), dest.clone()).into()),
            },

            // short
            Ty::SignedInteger(IT::Short) => match dest {
                Ty::SignedInteger(IT::Char) | Ty::UnsignedInteger(IT::Char) => Op::Trunc,
                Ty::UnsignedInteger(IT::Short) => Op::BitCast,
                Ty::SignedInteger(_) => Op::SExt,
                Ty::UnsignedInteger(_) => Op::ZExt,
                Ty::Float | Ty::Double => Op::SIToFP,
                Ty::Pointer(_) => Op::IntToPtr,
                _ => return Err(InvalidCast(curr.clone(), dest.clone()).into()),
            },
            Ty::UnsignedInteger(IT::Short) => match dest {
                Ty::SignedInteger(IT::Char) | Ty::UnsignedInteger(IT::Char) => Op::Trunc,
                Ty::SignedInteger(IT::Short) => Op::BitCast,
                Ty::SignedInteger(_) | Ty::UnsignedInteger(_) => Op::ZExt,
                Ty::Float | Ty::Double => Op::UIToFP,
                Ty::Pointer(_) => Op::IntToPtr,
                _ => return Err(InvalidCast(curr.clone(), dest.clone()).into()),
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
                _ => return Err(InvalidCast(curr.clone(), dest.clone()).into()),
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
                _ => return Err(InvalidCast(curr.clone(), dest.clone()).into()),
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
                _ => return Err(InvalidCast(curr.clone(), dest.clone()).into()),
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
                _ => return Err(InvalidCast(curr.clone(), dest.clone()).into()),
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
                _ => return Err(InvalidCast(curr.clone(), dest.clone()).into()),
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
                _ => return Err(InvalidCast(curr.clone(), dest.clone()).into()),
            },

            Ty::Bool => match dest {
                Ty::SignedInteger(_) | Ty::UnsignedInteger(_) => Op::ZExt,
                Ty::Float | Ty::Double => Op::UIToFP,
                _ => return Err(InvalidCast(curr.clone(), dest.clone()).into()),
            },

            Ty::Float => match dest {
                Ty::SignedInteger(_) => Op::FPToSI,
                Ty::UnsignedInteger(_) => Op::FPToUI,
                Ty::Double => Op::FPExt,
                _ => return Err(InvalidCast(curr.clone(), dest.clone()).into()),
            },
            Ty::Double => match dest {
                Ty::SignedInteger(_) => Op::FPToSI,
                Ty::UnsignedInteger(_) => Op::FPToUI,
                Ty::Float => Op::FPTrunc,
                _ => return Err(InvalidCast(curr.clone(), dest.clone()).into()),
            },

            Ty::Pointer(_) => match dest {
                Ty::SignedInteger(_) | Ty::UnsignedInteger(_) => Op::PtrToInt,
                Ty::Pointer(_) => Op::BitCast,
                _ => return Err(InvalidCast(curr.clone(), dest.clone()).into()),
            },
            Ty::Array(_, _) => match dest {
                Ty::Pointer(_) => Op::BitCast,
                _ => return Err(InvalidCast(curr.clone(), dest.clone()).into()),
            },
            _ => return Err(InvalidCast(curr.clone(), dest.clone()).into()),
        };
        Ok(instruction)
    }
}
