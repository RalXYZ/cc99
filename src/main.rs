extern crate pest;
#[macro_use]
extern crate pest_derive;
use clap::{ArgGroup, Parser};
use inkwell::context::Context;
use std::fs;
use std::path::Path;
use std::process::Command;

mod ast;
mod generator;
mod parse;
mod preprocess;
mod utils;

use generator::*;
use parse::*;
use preprocess::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(group(
    ArgGroup::new("stage")
        .args(&["expand", "parse", "bitcode", "compile", "assemble"])
))]
struct Args {
    /// Source code
    #[clap()]
    file: String,

    /// Place the output into <OUTPUT>
    #[clap(short, long)]
    output: Option<String>,

    /// Preprocess only; do not parse, compile, assemble or link
    #[clap(short = 'E', long)]
    expand: bool,

    /// Preprocess and parse; do not compile, assemble or link
    #[clap(short, long)]
    parse: bool,

    /// Generate LLVM Bitcode only
    #[clap(short, long)]
    bitcode: bool,

    /// Compile only; do not assemble or link
    #[clap(short = 'S', long)]
    compile: bool,

    /// Compile and assemble, but do not link
    #[clap(short = 'c', long)]
    assemble: bool,

    /// Add the directory <dir>,<dir>,<dir>(from left to right) to the list of directories to be searched for header files during preprocessing
    #[clap(short, long)]
    include: Option<String>,
}

fn main() {
    let args = Args::parse();
    let include_dirs: Vec<&str> = match args.include {
        Some(ref includes) => includes.split(',').collect(),
        None => Default::default(),
    };
    let basename = Path::new(&args.file).file_stem().unwrap().to_str().unwrap();
    let output_file = match args.output {
        Some(output) => output,
        None => {
            if args.expand {
                format!("{}.expand.c", basename)
            } else if args.parse {
                format!("{}.json", basename)
            } else if args.bitcode {
                format!("{}.bc", basename)
            } else if args.compile {
                format!("{}.s", basename)
            } else if args.assemble {
                format!("{}.o", basename)
            } else {
                basename.to_string()
            }
        }
    };

    // preprocess
    let code = preprocess_file(&args.file, &include_dirs)
        .unwrap_or_else(|e| panic!("Preprocess failed:\n{}", e));

    if args.expand {
        fs::write(&output_file, &code)
            .unwrap_or_else(|_| panic!("Unable to write file {}", output_file));
    } else {
        // parse
        let ast = parse(&code).unwrap_or_else(|e| panic!("Parse failed:\n{}", e));

        if args.parse {
            fs::write(&output_file, &serde_json::to_string(&ast).unwrap())
                .unwrap_or_else(|_| panic!("Unable to write file {}", output_file));
        } else {
            // code_gen
            let context = Context::create();
            let mut code_gen = Generator::new(&context, &args.file);
            let gen_result = code_gen.gen(&ast);
            if let Err(e) = gen_result {
                panic!("{}", e);
            }

            if args.bitcode {
                // generate LLVM bitcode
                if !code_gen.out_bc(Some(output_file)) {
                    panic!("Unable to generate bitcode");
                }
            } else if args.compile {
                // generate assembly code
                if let Err(e) = code_gen.out_asm_or_obj(false, Some(output_file)) {
                    panic!("{}", e);
                }
            } else {
                // generate object code
                if let Err(e) = code_gen.out_asm_or_obj(
                    true,
                    Some(match args.assemble {
                        true => output_file.clone(),
                        false => basename.to_string() + ".o",
                    }),
                ) {
                    panic!("{}", e);
                }
                if !args.assemble {
                    // generate binary
                    let clang_result = Command::new("clang")
                        .arg(basename.to_string() + ".o")
                        .arg("-o")
                        .arg(output_file.as_str())
                        .arg("-no-pie")
                        .output()
                        .expect("Unable to generate binary");
                    if !clang_result.status.success() {
                        panic!("{}", String::from_utf8_lossy(&clang_result.stderr));
                    }

                    // remove tmp files
                    fs::remove_file(basename.to_string() + ".o").unwrap_or_else(|_| {
                        panic!("Unable to remove file {}", basename.to_string() + ".o");
                    });
                }
            }
        }
    }
}
