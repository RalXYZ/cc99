use std::collections::{HashMap, VecDeque};
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{FunctionValue, PointerValue};
use crate::ast::BasicType as BT;

pub mod generator;
pub mod cast_instruction;
pub mod func_def;

pub struct Generator<'ctx> {
    source_path: &'ctx str,
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,

    //>>>>>>>>>>>>>>>>>>>>>>>>
    //      LLVM Blocks
    //<<<<<<<<<<<<<<<<<<<<<<<<

    // LLVM blocks for the current function
    val_map_block_stack: Vec<HashMap<String, (BT, PointerValue<'ctx>)>>,
    // current function block
    current_function: Option<(FunctionValue<'ctx>, Option<BT>)>,
    // break labels (in loop statements)
    break_labels: VecDeque<BasicBlock<'ctx>>,
    // continue labels (in loop statements)
    continue_labels: VecDeque<BasicBlock<'ctx>>,
    // hashset for functions
    function_map: HashMap<String, (Option<BT>, Vec<BT>)>,
    // hashset for global variable
    global_variable_map: HashMap<String, (BT, PointerValue<'ctx>)>,
}