extern crate pest;
#[macro_use]
extern crate pest_derive;
use clap::{ArgGroup, Parser};
use std::fs;
use inkwell::context::Context;

mod ast;
mod parse;
mod preprocess;
mod generator;

use parse::*;
use preprocess::*;
use generator::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(group(
    ArgGroup::new("stage")
        .args(&["expand", "parse"])
))]
struct Args {
    /// Source code
    #[clap()]
    file: String,

    /// Place the output into <file>
    #[clap(short, long, default_value_t = String::from("a.out"))]
    output: String,

    /// Preprocess only; do not parse, compile, assemble or link
    #[clap(short, long)]
    expand: bool,

    /// Preprocess and parse; do not compile, assemble or link
    #[clap(short, long)]
    parse: bool,

    /// Add the directory <dir>,<dir>,<dir>(from left to right) to the list of directories to be searched for header files during preprocessing
    #[clap(short, long, default_value_t = String::from(""))]
    include: String,
}

fn main() {
    let args = Args::parse();
    let include_dirs: Vec<&str> = args.include.split(',').collect();
    if args.expand {
        let code = preprocess_file(&args.file, &include_dirs)
            .unwrap_or_else(|e| panic!("Preprocess failed:\n{}", e));
        fs::write(&args.output, &code)
            .unwrap_or_else(|_| panic!("Unable to write file {}", args.output));
    } else if args.parse {
        let code = preprocess_file(&args.file, &include_dirs)
            .unwrap_or_else(|e| panic!("Preprocess failed:\n{}", e));
        let ast = parse(&code).unwrap_or_else(|e| panic!("Parse failed:\n{}", e));
        fs::write(&args.output, &serde_json::to_string(&ast).unwrap())
            .unwrap_or_else(|_| panic!("Unable to write file {}", args.output));
    } else {
        let code = preprocess_file(&args.file, &include_dirs).unwrap();
        let ast = parse(&code).unwrap();

        let context = Context::create();
        let mut code_gen = Generator::new(&context, &args.file);
        code_gen.gen(&ast);

        unimplemented!();
    }
}
