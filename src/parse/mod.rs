use pest::Parser;
use serde::Serialize;
use std::error::Error;

use super::ast::*;

#[derive(Parser, Serialize)]
#[grammar = "./parse/parse.pest"]
pub struct CC99Parser;

// TODO 使用自己的错误类型
pub fn parse(code: &str) -> Result<Box<AST>, Box<dyn Error>> {
    Ok(Box::new(AST::GlobalDeclaration(vec![])))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::preprocess::preprocess;
    use std::fs::File;
    use std::io::Read;
    use walkdir::WalkDir;

    //请使用cargo test -- --nocapture 开启输出
    #[test]
    fn process_test_file() {
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
            println!(">>> {} {} <<<", "Start compiling", source_path);

            let mut source_file = File::open(source_path).expect("Unable to open source file!");
            let mut source_content: String = String::new();

            source_file
                .read_to_string(&mut source_content)
                .expect("Unable to read source file!");

            let res = preprocess(&source_content).unwrap();
            let res = parse(&res).unwrap_or_else(|e| panic!("{}", e));
            //直接打印出来看看
            println!("{:?}", res);
            println!(">>> {} <<<", "Finish PreProcess");
        }
    }
}
