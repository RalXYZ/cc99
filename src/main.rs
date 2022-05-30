extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate core;

use clap::{ArgGroup, Parser};
use inkwell::{context::Context, OptimizationLevel};
use std::fs;
use std::io::{Read, stdin};
use std::path::Path;
use std::process::Command;
use cc99::compile_result;

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
        .args(&["expand", "parse", "bitcode", "compile", "assemble","visual"])
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

    /// Optimization level
    #[clap(short = 'O', long, default_value = "0")]
    opt_level: u32,

    /// Visual Compile AST
    #[clap(short = 'V', long)]
    visual: bool,

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
    if args.visual {
        let mut buffer = String::new();
        let size=stdin().read_to_string(&mut buffer);
        match size {
            Ok(_)=>{
                let res=compile_result(buffer.as_str());
                print!("{}",res);
                std::process::exit(0);
            }
            Err(e)=>{
                eprintln!("Unable to read stdin: {}",e);
                std::process::exit(1);
            }
        }
    }
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
    let opt_level = match args.opt_level {
        0 => OptimizationLevel::None,
        1 => OptimizationLevel::Less,
        2 => OptimizationLevel::Default,
        3 => OptimizationLevel::Aggressive,
        _ => {
            eprintln!("Invalid optimization level");
            std::process::exit(1);
        },
    };

    // preprocess
    let code = preprocess_file(&args.file, &include_dirs)
        .unwrap_or_else(|e|  {
            eprintln!("Preprocess failed:\n{}", e);
            std::process::exit(1);
        });

    if args.expand {
        fs::write(&output_file, &code)
            .unwrap_or_else(|_| {
                eprintln!("Unable to write file {}", output_file);
                std::process::exit(1);
            });
    } else {
        // parse
        let ast = Parse::new()
            .parse(&code)
            .unwrap_or_else(|e| {
                eprintln!("Parse failed:\n{}", e);
                std::process::exit(1);
            });

        if args.parse {
            fs::write(&output_file, &serde_json::to_string(&ast).unwrap())
                .unwrap_or_else(|_| {
                    eprintln!("Unable to write file {}", output_file);
                    std::process::exit(1);
                });
        } else {
            // code_gen
            let context = Context::create();
            let mut code_gen = Generator::new(&context, &args.file, &code);
            code_gen.gen(&ast);

            if args.bitcode {
                // generate LLVM bitcode
                if !code_gen.out_bc(Some(output_file)) {
                    eprintln!("Unable to generate bitcode");
                    std::process::exit(1);
                }
            } else if args.compile {
                // generate assembly code
                if let Err(e) = code_gen.out_asm_or_obj(false, Some(output_file), opt_level) {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            } else {
                // generate object code
                if let Err(e) = code_gen.out_asm_or_obj(
                    true,
                    Some(match args.assemble {
                        true => output_file.clone(),
                        false => basename.to_string() + ".o",
                    }),
                    opt_level,
                ) {
                    eprintln!("{}", e);
                    std::process::exit(1);
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
                        eprintln!("{}", String::from_utf8_lossy(&clang_result.stderr));
                        std::process::exit(1);
                    }

                    // remove tmp files
                    fs::remove_file(basename.to_string() + ".o").unwrap_or_else(|_| {
                        eprintln!("Unable to remove file {}", basename.to_string() + ".o");
                        std::process::exit(1);
                    });
                }
            }
        }
    }
}
