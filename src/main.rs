extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate core;

use cc99::compile_result;
use clap::{ArgGroup, Parser};
use inkwell::{context::Context, OptimizationLevel};
use std::fs;
use std::io::{stdin, Read};
use std::path::Path;
use std::process::{Command, Output};

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
    #[clap(short, long, display_order = 0)]
    output: Option<String>,

    /// Preprocess only; do not parse, compile, assemble or link
    #[clap(short = 'E', long, display_order = 3)]
    expand: bool,

    /// Preprocess and parse; do not compile, assemble or link
    #[clap(short, long, display_order = 4)]
    parse: bool,

    /// Generate LLVM Bitcode only
    #[clap(short, long, display_order = 5)]
    bitcode: bool,

    /// Compile only; do not assemble or link
    #[clap(short = 'S', long, display_order = 6)]
    compile: bool,

    /// Compile and assemble, but do not link
    #[clap(short = 'c', long, display_order = 7)]
    assemble: bool,

    /// AST Visualization
    #[clap(short = 'V', long, display_order = 8)]
    visual: bool,

    /// Optimization level, from 0 to 3
    #[clap(short = 'O', long, default_value = "0", display_order = 2)]
    opt_level: u32,

    /// Add the directory <dir>,<dir>,<dir>(from left to right) to the list of directories to be searched for header files during preprocessing
    #[clap(short, long, display_order = 1)]
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
        let size = stdin().read_to_string(&mut buffer);
        match size {
            Ok(_) => {
                let res = compile_result(buffer.as_str());
                print!("{}", res);
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("Unable to read stdin: {}", e);
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
        }
    };

    // preprocess
    let code = preprocess_file(&args.file, &include_dirs).unwrap_or_else(|e| {
        eprintln!("Preprocess failed:\n{}", e);
        std::process::exit(1);
    });

    if args.expand {
        fs::write(&output_file, &code).unwrap_or_else(|_| {
            eprintln!("Unable to write file {}", output_file);
            std::process::exit(1);
        });
    } else {
        // parse
        let ast = Parse::new().parse(&code).unwrap_or_else(|e| {
            eprintln!("Parse failed:\n{}", e);
            std::process::exit(1);
        });

        if args.parse {
            fs::write(&output_file, &serde_json::to_string(&ast).unwrap()).unwrap_or_else(|_| {
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
                    let object_file = basename.to_string() + ".o";
                    link(&object_file, &output_file, Compiler::Clang)
                        .map_err(|_| link(&object_file, &output_file, Compiler::GNU))
                        .map_or_else(
                            |e| {
                                if let Err(e) = e {
                                    eprintln!("{}", e);
                                    std::process::exit(1);
                                }
                            },
                            |output| {
                                if !output.status.success() {
                                    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                                    std::process::exit(1);
                                }
                            },
                        );
                    // remove tmp files
                    fs::remove_file(&object_file).unwrap_or_else(|_| {
                        eprintln!("Unable to remove file {}", object_file);
                        std::process::exit(1);
                    });
                }
            }
        }
    }
}

#[derive(Debug)]
enum Compiler {
    GNU,
    Clang,
}

fn link(source_file: &str, target_file: &str, compiler: Compiler) -> Result<Output, String> {
    let args = vec!["-o", target_file, source_file];
    let output = match compiler {
        Compiler::GNU => Command::new("gcc").args(args).output(),
        Compiler::Clang => Command::new("clang").arg("-no-pie").args(args).output(),
    };
    match output {
        Ok(output) => Ok(output),
        Err(e) => Err(format!(
            "Failed to link using {:?}: {}",
            compiler,
            e.to_string()
        )),
    }
}
