extern crate cc99;

#[cfg(test)]
mod tests {
    use crate::*;
    use cc99::generator::Generator;
    use cc99::*;
    use inkwell::context::Context;
    use walkdir::WalkDir;

    #[test]
    fn parse_test_file() {
        let include_dirs = vec![];
        for entry in WalkDir::new("./tests")
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir())
        {
            let raw_path = entry.path().to_str();
            if raw_path.is_none() {
                continue;
            }

            let source_path = raw_path.unwrap();
            if !source_path.ends_with(".c") {
                continue;
            }
            println!(">>> Start compiling {} <<<", source_path);

            let res = preprocess_file(source_path, &include_dirs).unwrap();
            let ast = Parse::new().parse(&res).unwrap();
            println!("{}", serde_json::to_string(&ast).unwrap());
            println!(">>> Finish Parsing <<<");
        }
    }

    #[test]
    fn test_gen() {
        let code = preprocess_file("./tests/global/decl2.c", vec![].as_slice()).unwrap();
        let ast = Parse::new()
            .parse(&code)
            .unwrap_or_else(|e| panic!("Parse failed:\n{}", e));

        let context = Context::create();
        let mut code_gen = Generator::new(&context, "./tests/global/decl2.c", &code);
        code_gen.gen(&ast);
        code_gen.out_asm_or_obj(false, None, inkwell::OptimizationLevel::None);
        code_gen.out_bc(None);
    }
}
