use std::collections::{HashMap, VecDeque};
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{FunctionValue, PointerValue};
use crate::ast::BasicType as CC99BasicTYpe;

pub mod generator;
pub mod cast_instruction;

pub struct Generator<'ctx> {
    source_path: &'ctx str,
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,

    //>>>>>>>>>>>>>>>>>>>>>>>>
    //      LLVM Blocks
    //<<<<<<<<<<<<<<<<<<<<<<<<

    addr_map_stack: Vec<HashMap<String, (CC99BasicTYpe, PointerValue<'ctx>)>>,
    // current function block
    current_function: Option<(FunctionValue<'ctx>, Option<CC99BasicTYpe>)>,
    // break labels (in loop statements)
    break_labels: VecDeque<BasicBlock<'ctx>>,
    // continue labels (in loop statements)
    continue_labels: VecDeque<BasicBlock<'ctx>>,
    // hashset for functions
    function_map: HashMap<String, (Option<CC99BasicTYpe>, Vec<CC99BasicTYpe>)>,
    // hashset for global variable
    global_variable_map: HashMap<String, (CC99BasicTYpe, PointerValue<'ctx>)>,
}
