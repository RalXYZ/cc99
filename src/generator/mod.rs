use std::collections::{HashMap, VecDeque};
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{FunctionValue, PointerValue};
use crate::ast::BasicType as BT;

pub mod gen;
mod cast_inst;
mod func_def;
mod stmt;
mod out;
mod expr;
mod utils;

pub struct Generator<'ctx> {
    module_name: &'ctx str,
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,

    //>>>>>>>>>>>>>>>>>>>>>>>>
    //      LLVM Blocks
    //<<<<<<<<<<<<<<<<<<<<<<<<

    // LLVM blocks for the current function
    val_map_block_stack: Vec<HashMap<String, (BT, PointerValue<'ctx>)>>,
    // current function block
    current_function: Option<(FunctionValue<'ctx>, BT)>,
    // break labels (in loop statements)
    break_labels: VecDeque<BasicBlock<'ctx>>,
    // continue labels (in loop statements)
    continue_labels: VecDeque<BasicBlock<'ctx>>,
    // hashset for functions
    function_map: HashMap<String, (BT, Vec<BT>)>,
    // hashset for global variable
    global_variable_map: HashMap<String, (BT, PointerValue<'ctx>)>,
}
